use std::collections::BTreeMap;
use std::ops::Bound;
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

    /// 点 x を含む区間 [p_left, p_right) があれば Some((p_left, p_right)) を返す
    #[inline]
    fn find_containing(&self, x: i64) -> Option<(i64, i64)> {
        self.range_map
            .range((Bound::Unbounded, Bound::Included(x)))
            .next_back()
            .filter(|&(&_p_left, &p_right)| x < p_right)
            .map(|(&p_left, &p_right)| (p_left, p_right))
    }

    /// 点 x がいずれかの区間に含まれているか
    pub fn contains(&self, x: i64) -> bool {
        self.find_containing(x).is_some()
    }

    /// 区間 [left, right) を追加し、重なる区間をマージする
    /// 戻り値: マージ後の区間 (merged_left, merged_right)
    pub fn insert(&mut self, left: i64, right: i64) -> (i64, i64) {
        if left >= right {
            return (left, left);
        }

        // 1. マージ対象の区間を収集し、マージ後の範囲を計算
        let (merged_left, merged_right, ranges_to_remove) = self.collect_overlapping_ranges(left, right);

        // 2. 重なる区間を削除
        for p_left in ranges_to_remove {
            let p_right = self.range_map.remove(&p_left).unwrap();
            self.total_length -= p_right - p_left;
        }

        // 3. マージした区間を挿入
        self.range_map.insert(merged_left, merged_right);
        self.total_length += merged_right - merged_left;

        (merged_left, merged_right)
    }

    /// [left, right) と重なる or 隣接する区間を収集し、マージ後の範囲を返す
    fn collect_overlapping_ranges(&self, left: i64, right: i64) -> (i64, i64, Vec<i64>) {
        let mut merged_left = left;
        let mut merged_right = right;
        let mut ranges_to_remove = Vec::new();

        if left >= right {
            return (merged_left, merged_right, ranges_to_remove);
        }

        // left 以下の左端を持つ区間で、left と重なる or 隣接するものを探す
        if let Some((&p_left, &p_right)) = self
            .range_map
            .range((Bound::Unbounded, Bound::Included(left)))
            .next_back()
            .filter(|&(_, &p_right)| left <= p_right)
        {
            merged_left = merged_left.min(p_left);
            merged_right = merged_right.max(p_right);
            ranges_to_remove.push(p_left);
        }

        // left より大きい左端を持つ区間で、right と重なる or 隣接するものを収集
        for (&p_left, &p_right) in self.range_map.range((Bound::Excluded(left), Bound::Included(right))) {
            merged_right = merged_right.max(p_right);
            ranges_to_remove.push(p_left);
        }

        (merged_left, merged_right, ranges_to_remove)
    }

    /// 区間 [left, right) を削除する（部分的に重なる場合は分割）
    pub fn remove(&mut self, left: i64, right: i64) {
        if left >= right {
            return;
        }

        let (ranges_to_remove, fragments_to_add) = self.collect_removal_targets(left, right);

        // 削除
        for p_left in ranges_to_remove {
            let p_right = self.range_map.remove(&p_left).unwrap();
            self.total_length -= p_right - p_left;
        }

        // 分割後の残りを追加
        for (p_left, p_right) in fragments_to_add {
            self.range_map.insert(p_left, p_right);
            self.total_length += p_right - p_left;
        }
    }

    /// 削除対象の区間と、分割後に残す断片を収集する
    fn collect_removal_targets(&self, left: i64, right: i64) -> (Vec<i64>, Vec<(i64, i64)>) {
        let mut ranges_to_remove = Vec::new();
        let mut fragments_to_add = Vec::new();

        if left >= right {
            return (ranges_to_remove, fragments_to_add);
        }

        // left を含む可能性のある区間（左端が left 以下で最大）
        if let Some((&p_left, &p_right)) = self
            .range_map
            .range((Bound::Unbounded, Bound::Included(left)))
            .next_back()
            .filter(|&(_, &p_right)| left < p_right)
        {
            // p_left <= left < p_right
            ranges_to_remove.push(p_left);

            // p_left < left < p_right
            if p_left < left {
                fragments_to_add.push((p_left, left)); // [p_left, left) を残す
            }
            // p_left <= left < right  < p_right
            if right < p_right {
                fragments_to_add.push((right, p_right)); // [right, p_right) を残す
            }
        }

        // 左端が (left, right) にある区間を収集
        for (&p_left, &p_right) in self.range_map.range((Bound::Excluded(left), Bound::Excluded(right))) {
            // left < p_left < right
            ranges_to_remove.push(p_left);

            // left < p_left < right < p_right
            if right < p_right {
                fragments_to_add.push((right, p_right)); // [right, p_right) を残す
            }
        }

        (ranges_to_remove, fragments_to_add)
    }

    /// 管理している区間の個数
    pub fn range_count(&self) -> usize {
        self.range_map.len()
    }

    /// 被覆している点の総数（区間長の合計）- O(1)
    pub fn covered_length(&self) -> i64 {
        self.total_length
    }

    /// x 以上で self に含まれていない最小値を返す (mex)
    /// 計算量: O(log N)
    pub fn min_excluded_from(&self, x: i64) -> i64 {
        self.find_containing(x).map(|(_, p_right)| p_right).unwrap_or(x)
    }

    /// x 以下で self に含まれていない最大値を返す
    /// 計算量: O(log N)
    pub fn max_excluded_to(&self, x: i64) -> i64 {
        self.find_containing(x).map(|(p_left, _)| p_left - 1).unwrap_or(x)
    }

    /// 全区間のイテレータ
    pub fn iter(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.range_map.iter().map(|(&p_left, &p_right)| (p_left, p_right))
    }
}
