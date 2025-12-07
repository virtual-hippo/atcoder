use my_atcoder_lib::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        q: usize,
    }

    let mut range_map = RangeMap::new();

    for _ in 0..q {
        input! {
            l: i64,
            r: i64,
        }
        range_map.insert(l, r + 1);
        println!("{}", n - range_map.covered_length());
    }
}

pub mod my_atcoder_lib {
    use std::collections::BTreeMap;
    use std::ops::Bound;

    /// 半開区間 [start, end) を管理するデータ構造
    #[derive(Debug, Clone, Default)]
    pub struct RangeMap {
        range_map: BTreeMap<i64, i64>,
        total_length: i64,
    }

    impl RangeMap {
        pub fn new() -> Self {
            Self {
                range_map: BTreeMap::new(),
                total_length: 0,
            }
        }

        /// 点 x を含む区間 [start, end) があれば Some((start, end)) を返す
        #[inline]
        fn find_containing(&self, x: i64) -> Option<(i64, i64)> {
            self.range_map
                .range((Bound::Unbounded, Bound::Included(x)))
                .next_back()
                .filter(|&(&_start, &end)| x < end)
                .map(|(&start, &end)| (start, end))
        }

        /// 点 x がいずれかの区間に含まれているか
        pub fn contains(&self, x: i64) -> bool {
            self.find_containing(x).is_some()
        }

        /// 区間 [left, right) を追加し、重なる区間をマージする
        /// 戻り値: マージ後の区間 (merged_start, merged_end)
        pub fn insert(&mut self, left: i64, right: i64) -> (i64, i64) {
            if left >= right {
                return (left, left);
            }

            // 1. マージ対象の区間を収集し、マージ後の範囲を計算
            let (merged_start, merged_end, ranges_to_remove) = self.collect_overlapping_ranges(left, right);

            // 2. 重なる区間を削除
            for start in ranges_to_remove {
                let end = self.range_map.remove(&start).unwrap();
                self.total_length -= end - start;
            }

            // 3. マージした区間を挿入
            self.range_map.insert(merged_start, merged_end);
            self.total_length += merged_end - merged_start;

            (merged_start, merged_end)
        }

        /// [left, right) と重なる or 隣接する区間を収集し、マージ後の範囲を返す
        fn collect_overlapping_ranges(&self, left: i64, right: i64) -> (i64, i64, Vec<i64>) {
            let mut merged_start = left;
            let mut merged_end = right;
            let mut ranges_to_remove = Vec::new();

            // left 以下の左端を持つ区間で、left と重なる or 隣接するものを探す
            if let Some((&start, &end)) = self
                .range_map
                .range((Bound::Unbounded, Bound::Included(left)))
                .next_back()
                .filter(|&(_, &end)| end >= left)
            {
                merged_start = merged_start.min(start);
                merged_end = merged_end.max(end);
                ranges_to_remove.push(start);
            }

            // left より大きい左端を持つ区間で、right と重なる or 隣接するものを収集
            for (&start, &end) in self.range_map.range((Bound::Excluded(left), Bound::Included(right))) {
                merged_end = merged_end.max(end);
                ranges_to_remove.push(start);
            }

            (merged_start, merged_end, ranges_to_remove)
        }

        /// 区間 [left, right) を削除する（部分的に重なる場合は分割）
        pub fn remove(&mut self, left: i64, right: i64) {
            if left >= right {
                return;
            }

            let (ranges_to_remove, fragments_to_add) = self.collect_removal_targets(left, right);

            // 削除
            for start in ranges_to_remove {
                let end = self.range_map.remove(&start).unwrap();
                self.total_length -= end - start;
            }

            // 分割後の残りを追加
            for (start, end) in fragments_to_add {
                self.range_map.insert(start, end);
                self.total_length += end - start;
            }
        }

        /// 削除対象の区間と、分割後に残す断片を収集する
        fn collect_removal_targets(&self, left: i64, right: i64) -> (Vec<i64>, Vec<(i64, i64)>) {
            let mut ranges_to_remove = Vec::new();
            let mut fragments_to_add = Vec::new();

            // left を含む可能性のある区間（左端が left 以下で最大）
            if let Some((&start, &end)) = self.range_map.range(..=left).next_back().filter(|&(_, &end)| end > left) {
                ranges_to_remove.push(start);

                if start < left {
                    fragments_to_add.push((start, left)); // [start, left) を残す
                }
                if end > right {
                    fragments_to_add.push((right, end)); // [right, end) を残す
                }
            }

            // 左端が (left, right) にある区間を収集
            for (&start, &end) in self.range_map.range((Bound::Excluded(left), Bound::Excluded(right))) {
                ranges_to_remove.push(start);

                if end > right {
                    fragments_to_add.push((right, end)); // [right, end) を残す
                }
            }

            (ranges_to_remove, fragments_to_add)
        }

        /// 管理している区間の個数
        pub fn interval_count(&self) -> usize {
            self.range_map.len()
        }

        /// 被覆している点の総数（区間長の合計）- O(1)
        pub fn covered_length(&self) -> i64 {
            self.total_length
        }

        /// x 以上で self に含まれていない最小値を返す (mex)
        /// 計算量: O(log N)
        pub fn min_excluded_from(&self, x: i64) -> i64 {
            self.find_containing(x).map(|(_, end)| end).unwrap_or(x)
        }

        /// x 以下で self に含まれていない最大値を返す
        /// 計算量: O(log N)
        pub fn max_excluded_to(&self, x: i64) -> i64 {
            self.find_containing(x).map(|(start, _)| start - 1).unwrap_or(x)
        }

        /// 全区間のイテレータ
        pub fn iter(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
            self.range_map.iter().map(|(&start, &end)| (start, end))
        }
    }
}
