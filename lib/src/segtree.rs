use std::{convert::Infallible, ops};
pub trait Monoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct SumI64(Infallible);
impl Monoid for SumI64 {
    type S = i64;
    fn identity() -> Self::S {
        0
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        a + b
    }
}

pub struct MinI64(Infallible);
impl Monoid for MinI64 {
    type S = i64;
    fn identity() -> Self::S {
        i64::MAX
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        *a.min(b)
    }
}

pub struct XorU64(Infallible);
impl Monoid for XorU64 {
    type S = u64;
    fn identity() -> Self::S {
        0
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
}

pub struct GcdU64(Infallible);
impl GcdU64 {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
impl Monoid for GcdU64 {
    type S = u64;
    fn identity() -> Self::S {
        0
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        Self::gcd(*a, *b)
    }
}

///
/// f ∘ g (= f(g(x)) で実装しているため, prod は配列の右側ほど先に適用される(通常とは逆の向き).
/// AtCoder での実用性を考えると修正したほう良さそう
///
pub struct Affine(Infallible);
impl Monoid for Affine {
    type S = (i64, i64);
    fn identity() -> Self::S {
        (1, 0)
    }
    fn op((a0, b0): &Self::S, (a1, b1): &Self::S) -> Self::S {
        (a0 * a1, a0 * b1 + b0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaxSubArrayState {
    pub max: i64,
    pub prefix: i64,
    pub suffix: i64,
    pub sum: i64,
}
impl MaxSubArrayState {
    pub fn from_value(x: i64) -> Self {
        Self {
            max: x.max(0),
            prefix: x.max(0),
            suffix: x.max(0),
            sum: x,
        }
    }
}

pub struct MaxSubArray(Infallible);
impl Monoid for MaxSubArray {
    type S = MaxSubArrayState;
    fn identity() -> Self::S {
        MaxSubArrayState { max: 0, prefix: 0, suffix: 0, sum: 0 }
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        MaxSubArrayState {
            max: a.max.max(b.max).max(a.suffix + b.prefix),
            prefix: (a.prefix).max(a.sum + b.prefix),
            suffix: (a.suffix + b.sum).max(b.suffix),
            sum: a.sum + b.sum,
        }
    }
}

pub struct Segtree<M: Monoid> {
    n: usize,
    size: usize,
    data: Vec<M::S>,
}
impl<M: Monoid> Segtree<M> {
    pub fn new(n: usize) -> Self {
        vec![M::identity(); n].into()
    }

    pub fn all_prod(&self) -> M::S {
        self.data[1].clone()
    }

    fn range_to_tuple<R: ops::RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        let l = match range.start_bound() {
            ops::Bound::Included(l) => *l,
            ops::Bound::Excluded(l) => l + 1,
            ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            ops::Bound::Included(r) => r + 1,
            ops::Bound::Excluded(r) => *r,
            ops::Bound::Unbounded => self.n,
        };
        (l, r)
    }

    /// 区間積を計算する。計算量: O(log n)
    pub fn prod<R: ops::RangeBounds<usize>>(&self, range: R) -> M::S {
        let (mut l, mut r) = self.range_to_tuple(range);
        debug_assert!(l <= r && r <= self.n);
        if l == 0 && r == self.n {
            return self.all_prod();
        }

        l += self.size;
        r += self.size;
        let mut sml = M::identity();
        let mut smr = M::identity();

        while l < r {
            if l % 2 == 1 {
                sml = M::op(&sml, &self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                smr = M::op(&self.data[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&sml, &smr)
    }

    pub fn set(&mut self, i: usize, x: M::S) {
        debug_assert!(i < self.n);
        self.apply(i, move |_| x);
    }

    pub fn get(&self, i: usize) -> &M::S {
        debug_assert!(i < self.n);
        &self.data[i + self.size]
    }

    fn update(&mut self, p: usize) {
        self.data[p] = M::op(&self.data[2 * p], &self.data[2 * p + 1]);
    }

    pub fn apply<F: FnOnce(&M::S) -> M::S>(&mut self, i: usize, f: F) {
        debug_assert!(i < self.n);
        let mut p = i + self.size;
        self.data[p] = f(&self.data[p]);
        while p > 1 {
            p >>= 1;
            self.update(p);
        }
    }
}

impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
    fn from(xs: Vec<M::S>) -> Self {
        let n = xs.len();
        let size = n.next_power_of_two();
        let data = {
            let mut data = vec![M::identity(); 2 * size];
            data[size..size + n].clone_from_slice(&xs);
            data
        };

        let mut tree = Self { n, size, data };

        for p in (1..tree.size).rev() {
            tree.update(p);
        }
        tree
    }
}
impl<M: Monoid> FromIterator<M::S> for Segtree<M> {
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        v.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_basic() {
        let xs = vec![1i64, 2, 3, 4, 5];
        let seg = Segtree::<SumI64>::from(xs);
        assert_eq!(seg.prod(0..5), 15);
        assert_eq!(seg.prod(1..4), 9);
        assert_eq!(seg.prod(0..1), 1);
        assert_eq!(seg.all_prod(), 15);
    }

    #[test]
    fn sum_empty_range_returns_identity() {
        let seg = Segtree::<SumI64>::from(vec![1i64, 2, 3, 4, 5]);
        assert_eq!(seg.prod(0..0), 0);
        assert_eq!(seg.prod(3..3), 0);
        assert_eq!(seg.prod(5..5), 0);
    }

    #[test]
    fn sum_set_then_prod() {
        let mut seg = Segtree::<SumI64>::from(vec![1i64, 2, 3, 4, 5]);
        seg.set(2, 100);
        assert_eq!(*seg.get(2), 100);
        assert_eq!(seg.prod(0..5), 1 + 2 + 100 + 4 + 5);
        assert_eq!(seg.prod(2..3), 100);
        assert_eq!(seg.all_prod(), 1 + 2 + 100 + 4 + 5);
    }

    #[test]
    fn sum_new_initializes_to_identity() {
        let seg = Segtree::<SumI64>::new(7);
        assert_eq!(seg.all_prod(), 0);
        assert_eq!(seg.prod(0..7), 0);
        for i in 0..7 {
            assert_eq!(*seg.get(i), 0);
        }
    }

    #[test]
    fn min_basic() {
        let xs = vec![5i64, 3, 8, 1, 9, 2, 7];
        let seg = Segtree::<MinI64>::from(xs);
        assert_eq!(seg.prod(0..7), 1);
        assert_eq!(seg.prod(0..3), 3);
        assert_eq!(seg.prod(4..7), 2);
        assert_eq!(seg.prod(2..5), 1);
        assert_eq!(seg.prod(3..3), i64::MAX);
        assert_eq!(seg.all_prod(), 1);
    }

    #[test]
    fn min_set_updates_propagate() {
        let mut seg = Segtree::<MinI64>::from(vec![5i64, 3, 8, 1, 9]);
        seg.set(3, 100);
        assert_eq!(seg.prod(0..5), 3);
        seg.set(1, -5);
        assert_eq!(seg.all_prod(), -5);
    }

    #[test]
    fn xor_basic() {
        let xs = vec![1u64, 2, 3, 4, 5];
        let seg = Segtree::<XorU64>::from(xs);
        // 1^2^3^4^5 = 1
        assert_eq!(seg.prod(0..5), 1);
        // 2^3^4 = 5
        assert_eq!(seg.prod(1..4), 5);
        assert_eq!(seg.prod(0..1), 1);
        assert_eq!(seg.all_prod(), 1);
    }

    #[test]
    fn xor_empty_range_returns_identity() {
        let seg = Segtree::<XorU64>::from(vec![1u64, 2, 3, 4, 5]);
        assert_eq!(seg.prod(0..0), 0);
        assert_eq!(seg.prod(3..3), 0);
        assert_eq!(seg.prod(5..5), 0);
    }

    #[test]
    fn xor_self_cancellation() {
        // 同じ値を XOR すると 0 になる性質
        let seg = Segtree::<XorU64>::from(vec![7u64, 7, 7, 7]);
        assert_eq!(seg.prod(0..2), 0);
        assert_eq!(seg.prod(0..4), 0);
        assert_eq!(seg.prod(1..3), 0);
        // 奇数個なら元の値が残る
        assert_eq!(seg.prod(0..1), 7);
        assert_eq!(seg.prod(0..3), 7);
    }

    #[test]
    fn xor_set_then_prod() {
        let mut seg = Segtree::<XorU64>::from(vec![1u64, 2, 3, 4, 5]);
        seg.set(2, 0xFF);
        assert_eq!(*seg.get(2), 0xFF);
        // 1^2^0xFF^4^5 = (1^2^4^5) ^ 0xFF = 2 ^ 0xFF = 0xFD
        assert_eq!(seg.prod(0..5), 1u64 ^ 2 ^ 0xFF ^ 4 ^ 5);
        assert_eq!(seg.prod(2..3), 0xFF);
        assert_eq!(seg.all_prod(), 1u64 ^ 2 ^ 0xFF ^ 4 ^ 5);
    }

    #[test]
    fn xor_new_initializes_to_identity() {
        let seg = Segtree::<XorU64>::new(7);
        assert_eq!(seg.all_prod(), 0);
        assert_eq!(seg.prod(0..7), 0);
        for i in 0..7 {
            assert_eq!(*seg.get(i), 0);
        }
    }

    // size=8 に丸められる長さ 7 で、奇数境界を含むあらゆる区間が素朴計算と一致するか検証。
    #[test]
    fn xor_all_ranges_match_naive() {
        let xs: Vec<u64> =
            (0..7).map(|i| (i as u64 + 1).wrapping_mul(0x9E3779B97F4A7C15)).collect();
        let seg = Segtree::<XorU64>::from(xs.clone());

        for l in 0..=7 {
            for r in l..=7 {
                let mut expected = XorU64::identity();
                for i in l..r {
                    expected ^= xs[i];
                }
                assert_eq!(seg.prod(l..r), expected, "mismatch at l={}, r={}", l, r);
            }
        }
    }

    // テスト対象とは独立した素朴な gcd 実装(反復版)。
    fn naive_gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }

    #[test]
    fn gcd_fn_basic() {
        assert_eq!(GcdU64::gcd(12, 18), 6);
        assert_eq!(GcdU64::gcd(18, 12), 6); // 引数の順序によらない
        assert_eq!(GcdU64::gcd(6, 24), 6); // 一方が他方を割り切る
        assert_eq!(GcdU64::gcd(7, 11), 1); // 互いに素
    }

    // 単位元(0)と u64 端のケース。素朴実装の網羅では届かない範囲を補う。
    #[test]
    fn gcd_fn_edge_cases() {
        assert_eq!(GcdU64::gcd(0, 0), 0); // モノイドの identity に必要
        assert_eq!(GcdU64::gcd(0, 7), 7);
        assert_eq!(GcdU64::gcd(7, 0), 7);
        assert_eq!(GcdU64::gcd(u64::MAX, u64::MAX), u64::MAX);
        assert_eq!(GcdU64::gcd(u64::MAX, 6), 3); // u64::MAX は奇数で 3 を因子に持つ
    }

    // 小さな値域で素朴実装と全パターン照合する保険。
    #[test]
    fn gcd_fn_matches_naive_small() {
        for a in 0u64..30 {
            for b in 0u64..30 {
                assert_eq!(GcdU64::gcd(a, b), naive_gcd(a, b), "mismatch at ({}, {})", a, b);
            }
        }
    }

    #[test]
    fn gcd_basic() {
        let xs = vec![12u64, 18, 24, 9, 15];
        let seg = Segtree::<GcdU64>::from(xs);
        // gcd(12,18,24,9,15) = 3
        assert_eq!(seg.prod(0..5), 3);
        // gcd(12,18) = 6
        assert_eq!(seg.prod(0..2), 6);
        // gcd(12,18,24) = 6
        assert_eq!(seg.prod(0..3), 6);
        // gcd(18,24) = 6
        assert_eq!(seg.prod(1..3), 6);
        // gcd(9,15) = 3
        assert_eq!(seg.prod(3..5), 3);
        // 単一要素はその値
        assert_eq!(seg.prod(2..3), 24);
        assert_eq!(seg.all_prod(), 3);
    }

    #[test]
    fn gcd_empty_range_returns_identity() {
        let seg = Segtree::<GcdU64>::from(vec![6u64, 12, 18]);
        assert_eq!(seg.prod(0..0), 0);
        assert_eq!(seg.prod(1..1), 0);
        assert_eq!(seg.prod(3..3), 0);
    }

    #[test]
    fn gcd_with_zeros() {
        // 0 は単位元として振る舞うので、非ゼロ要素のみで gcd が計算される
        let seg = Segtree::<GcdU64>::from(vec![0u64, 12, 0, 18, 0]);
        assert_eq!(seg.prod(0..5), 6); // gcd(12, 18) = 6
        assert_eq!(seg.prod(0..1), 0); // 0 のみ
        assert_eq!(seg.prod(2..3), 0); // 0 のみ
        assert_eq!(seg.prod(0..2), 12); // gcd(0, 12) = 12
        assert_eq!(seg.prod(3..5), 18); // gcd(18, 0) = 18

        // 全部 0
        let seg2 = Segtree::<GcdU64>::from(vec![0u64; 4]);
        assert_eq!(seg2.all_prod(), 0);
        assert_eq!(seg2.prod(0..4), 0);
    }

    #[test]
    fn gcd_coprime_returns_one() {
        let seg = Segtree::<GcdU64>::from(vec![7u64, 11, 13, 17, 19]);
        assert_eq!(seg.prod(0..5), 1);
        assert_eq!(seg.prod(0..2), 1);
        assert_eq!(seg.prod(2..5), 1);
    }

    #[test]
    fn gcd_all_same() {
        let seg = Segtree::<GcdU64>::from(vec![6u64; 5]);
        assert_eq!(seg.all_prod(), 6);
        assert_eq!(seg.prod(0..5), 6);
        assert_eq!(seg.prod(1..4), 6);
        assert_eq!(seg.prod(0..1), 6);
    }

    #[test]
    fn gcd_set_then_prod() {
        let mut seg = Segtree::<GcdU64>::from(vec![12u64, 18, 24, 30]);
        // gcd(12,18,24,30) = 6
        assert_eq!(seg.prod(0..4), 6);

        seg.set(2, 25);
        assert_eq!(*seg.get(2), 25);
        // gcd(12,18,25,30) = gcd(6,25,30) = gcd(1,30) = 1
        assert_eq!(seg.prod(0..4), 1);

        seg.set(2, 15);
        // gcd(12,18,15,30) = gcd(6,15,30) = gcd(3,30) = 3
        assert_eq!(seg.prod(0..4), 3);
        assert_eq!(seg.all_prod(), 3);
    }

    #[test]
    fn gcd_new_initializes_to_identity() {
        let seg = Segtree::<GcdU64>::new(7);
        assert_eq!(seg.all_prod(), 0);
        assert_eq!(seg.prod(0..7), 0);
        for i in 0..7 {
            assert_eq!(*seg.get(i), 0);
        }
    }

    // size=8 に丸められる長さ 7 で、奇数境界を含むあらゆる区間が素朴計算と一致するか検証。
    #[test]
    fn gcd_all_ranges_match_naive() {
        let xs: Vec<u64> = vec![12, 18, 24, 9, 15, 30, 7];
        let seg = Segtree::<GcdU64>::from(xs.clone());

        for l in 0..=7 {
            for r in l..=7 {
                let mut expected = GcdU64::identity();
                for i in l..r {
                    expected = naive_gcd(expected, xs[i]);
                }
                assert_eq!(seg.prod(l..r), expected, "mismatch at l={}, r={}", l, r);
            }
        }
    }

    // f = (a, b) は f(x) = a*x + b を表し、f ⊕ g = f ∘ g(右が先に適用)。
    #[test]
    fn affine_identity_acts_as_identity() {
        let id = Affine::identity();
        let f = (3, 7);
        assert_eq!(Affine::op(&id, &f), f);
        assert_eq!(Affine::op(&f, &id), f);
    }

    #[test]
    fn affine_op_is_non_commutative() {
        // f(x) = 2x + 3, g(x) = 5x + 1
        // f ⊕ g = f∘g: f(g(x)) = 2(5x+1)+3 = 10x+5
        // g ⊕ f = g∘f: g(f(x)) = 5(2x+3)+1 = 10x+16
        let f = (2, 3);
        let g = (5, 1);
        assert_eq!(Affine::op(&f, &g), (10, 5));
        assert_eq!(Affine::op(&g, &f), (10, 16));
        assert_ne!(Affine::op(&f, &g), Affine::op(&g, &f));
    }

    #[test]
    fn affine_segtree_preserves_composition_order() {
        let f = (2, 3); // 2x + 3
        let g = (5, 1); // 5x + 1
        let h = (3, 4); // 3x + 4
        let seg = Segtree::<Affine>::from(vec![f, g, h]);

        // 全区間: f ⊕ g ⊕ h = f(g(h(x)))
        let full = Affine::op(&Affine::op(&f, &g), &h);
        assert_eq!(seg.prod(0..3), full);

        // x=10 への適用結果でも検算: h(10)=34, g(34)=171, f(171)=345
        let (a, b) = full;
        assert_eq!(a * 10 + b, 345);

        // 部分区間
        assert_eq!(seg.prod(0..1), f);
        assert_eq!(seg.prod(1..2), g);
        assert_eq!(seg.prod(2..3), h);
        assert_eq!(seg.prod(0..2), Affine::op(&f, &g));
        assert_eq!(seg.prod(1..3), Affine::op(&g, &h));
    }

    // 非可換モノイドで、奇数境界を含むあらゆる区間が素朴計算と一致するか検証する。
    // size=8(2 の冪)に丸められる長さ 7 を選び、prod の左右走査の op 順序を網羅的にチェックする。
    #[test]
    fn affine_all_ranges_match_naive() {
        let fs: Vec<(i64, i64)> = (0..7).map(|i| (i + 1, i * 2)).collect();
        let seg = Segtree::<Affine>::from(fs.clone());

        for l in 0..=7 {
            for r in l..=7 {
                let mut expected = Affine::identity();
                for i in l..r {
                    expected = Affine::op(&expected, &fs[i]);
                }
                assert_eq!(seg.prod(l..r), expected, "mismatch at l={}, r={}", l, r);
            }
        }
    }

    #[test]
    fn affine_set_then_prod() {
        let fs: Vec<(i64, i64)> = (0..5).map(|i| (i + 1, i)).collect();
        let mut seg = Segtree::<Affine>::from(fs.clone());

        seg.set(2, (10, 100));
        let mut updated = fs.clone();
        updated[2] = (10, 100);

        let mut expected = Affine::identity();
        for f in &updated {
            expected = Affine::op(&expected, f);
        }
        assert_eq!(seg.all_prod(), expected);
        assert_eq!(*seg.get(2), (10, 100));
    }

    // 空区間を許す前提で、xs[l..r] における連続部分和の最大値・最大プレフィックス和・
    // 最大サフィックス和・総和を素朴に求めて期待状態を作る。
    fn naive_max_subarray_state(xs: &[i64]) -> MaxSubArrayState {
        let mut max = 0i64;
        for i in 0..xs.len() {
            let mut s = 0i64;
            for j in i..xs.len() {
                s += xs[j];
                max = max.max(s);
            }
        }
        let mut prefix = 0i64;
        let mut s = 0i64;
        for &x in xs {
            s += x;
            prefix = prefix.max(s);
        }
        let mut suffix = 0i64;
        let mut s = 0i64;
        for &x in xs.iter().rev() {
            s += x;
            suffix = suffix.max(s);
        }
        let sum = xs.iter().sum();
        MaxSubArrayState { max, prefix, suffix, sum }
    }

    #[test]
    fn max_subarray_from_value_basic() {
        // 正値: [x] と空のうち [x] が良い
        assert_eq!(
            MaxSubArrayState::from_value(5),
            MaxSubArrayState { max: 5, prefix: 5, suffix: 5, sum: 5 }
        );
        // 負値: 空区間の方が良いので max/prefix/suffix は 0 に丸まる。sum はそのまま
        assert_eq!(
            MaxSubArrayState::from_value(-3),
            MaxSubArrayState { max: 0, prefix: 0, suffix: 0, sum: -3 }
        );
        // 0: 空区間と同等
        assert_eq!(
            MaxSubArrayState::from_value(0),
            MaxSubArrayState { max: 0, prefix: 0, suffix: 0, sum: 0 }
        );
    }

    #[test]
    fn max_subarray_identity_acts_as_identity() {
        let id = MaxSubArray::identity();
        for &x in &[7, -4, 0, -1, 100] {
            let s = MaxSubArrayState::from_value(x);
            assert_eq!(MaxSubArray::op(&id, &s), s, "left identity failed for x={}", x);
            assert_eq!(MaxSubArray::op(&s, &id), s, "right identity failed for x={}", x);
        }
    }

    // 結合則のスポット確認: (a ⊕ b) ⊕ c == a ⊕ (b ⊕ c)
    #[test]
    fn max_subarray_op_is_associative() {
        let xs = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let states: Vec<_> = xs.iter().map(|&x| MaxSubArrayState::from_value(x)).collect();
        for i in 0..states.len() {
            for j in 0..states.len() {
                for k in 0..states.len() {
                    let lhs = MaxSubArray::op(&MaxSubArray::op(&states[i], &states[j]), &states[k]);
                    let rhs = MaxSubArray::op(&states[i], &MaxSubArray::op(&states[j], &states[k]));
                    assert_eq!(lhs, rhs, "associativity failed at ({}, {}, {})", i, j, k);
                }
            }
        }
    }

    #[test]
    fn max_subarray_all_negative_uses_empty_subarray() {
        // 全要素負なら空区間が最大なので max=0、prefix/suffix も 0
        let xs = vec![-5i64, -3, -8, -1, -2];
        let states: Vec<_> = xs.iter().map(|&x| MaxSubArrayState::from_value(x)).collect();
        let seg = Segtree::<MaxSubArray>::from(states);
        let s = seg.all_prod();
        assert_eq!(s.max, 0);
        assert_eq!(s.prefix, 0);
        assert_eq!(s.suffix, 0);
        assert_eq!(s.sum, -19);
    }

    #[test]
    fn max_subarray_all_positive_takes_full_range() {
        // 全要素正なら全区間が最大、prefix/suffix も全長
        let xs = vec![1i64, 2, 3, 4, 5];
        let states: Vec<_> = xs.iter().map(|&x| MaxSubArrayState::from_value(x)).collect();
        let seg = Segtree::<MaxSubArray>::from(states);
        let total: i64 = xs.iter().sum();
        let s = seg.all_prod();
        assert_eq!(s.max, total);
        assert_eq!(s.prefix, total);
        assert_eq!(s.suffix, total);
        assert_eq!(s.sum, total);
    }

    #[test]
    fn max_subarray_kadane_classic() {
        // Kadane の有名例: 答えは [4,-1,2,1] = 6
        let xs = vec![-2i64, 1, -3, 4, -1, 2, 1, -5, 4];
        let states: Vec<_> = xs.iter().map(|&x| MaxSubArrayState::from_value(x)).collect();
        let seg = Segtree::<MaxSubArray>::from(states);
        assert_eq!(seg.all_prod().max, 6);
        // 部分区間でも検算: [4,-1,2,1] のみ
        assert_eq!(seg.prod(3..7).max, 6);
        // [-2,1,-3] は空区間 0 が最大
        assert_eq!(seg.prod(0..3).max, 1);
    }

    #[test]
    fn max_subarray_empty_range_returns_identity() {
        let xs = vec![1i64, -2, 3, -4, 5];
        let states: Vec<_> = xs.iter().map(|&x| MaxSubArrayState::from_value(x)).collect();
        let seg = Segtree::<MaxSubArray>::from(states);
        assert_eq!(seg.prod(0..0), MaxSubArray::identity());
        assert_eq!(seg.prod(3..3), MaxSubArray::identity());
        assert_eq!(seg.prod(5..5), MaxSubArray::identity());
    }

    #[test]
    fn max_subarray_new_initializes_to_identity() {
        let seg = Segtree::<MaxSubArray>::new(7);
        assert_eq!(seg.all_prod(), MaxSubArray::identity());
        assert_eq!(seg.prod(0..7), MaxSubArray::identity());
        for i in 0..7 {
            assert_eq!(*seg.get(i), MaxSubArray::identity());
        }
    }

    #[test]
    fn max_subarray_set_then_prod() {
        let xs = vec![1i64, -2, 3, -4, 5];
        let states: Vec<_> = xs.iter().map(|&x| MaxSubArrayState::from_value(x)).collect();
        let mut seg = Segtree::<MaxSubArray>::from(states);
        // [1,-2,3,-4,5]: 連続部分和の最大は max(1, 3, 5, 3-4+5=4, 1-2+3=2) = 5
        assert_eq!(seg.all_prod().max, 5);

        seg.set(1, MaxSubArrayState::from_value(10));
        // [1,10,3,-4,5]: 全区間 1+10+3-4+5 = 15
        assert_eq!(seg.all_prod().max, 15);

        seg.set(3, MaxSubArrayState::from_value(-100));
        // [1,10,3,-100,5]: -100 で分断され 1+10+3=14 が最大
        assert_eq!(seg.all_prod().max, 14);
        assert_eq!(seg.prod(3..5).max, 5); // [-100, 5] → 5
        assert_eq!(seg.prod(0..3).max, 14); // [1,10,3] → 14
    }

    // size=8 に丸められる長さ 7 で、奇数境界を含むあらゆる区間が素朴計算と一致するか検証。
    // op の左右走査順序と全フィールド (max, prefix, suffix, sum) を網羅的にチェック。
    #[test]
    fn max_subarray_all_ranges_match_naive() {
        let xs: Vec<i64> = vec![-2, 1, -3, 4, -1, 2, 1];
        let states: Vec<_> = xs.iter().map(|&x| MaxSubArrayState::from_value(x)).collect();
        let seg = Segtree::<MaxSubArray>::from(states);

        for l in 0..=xs.len() {
            for r in l..=xs.len() {
                let expected = naive_max_subarray_state(&xs[l..r]);
                assert_eq!(seg.prod(l..r), expected, "mismatch at l={}, r={}", l, r);
            }
        }
    }

    // 正負混在・ゼロ・大きめ絶対値を含む別パターンでもナイーブと一致するか検証。
    #[test]
    fn max_subarray_all_ranges_match_naive_mixed() {
        let xs: Vec<i64> = vec![5, -10, 0, 3, -1, -1, 7, -8];
        let states: Vec<_> = xs.iter().map(|&x| MaxSubArrayState::from_value(x)).collect();
        let seg = Segtree::<MaxSubArray>::from(states);

        for l in 0..=xs.len() {
            for r in l..=xs.len() {
                let expected = naive_max_subarray_state(&xs[l..r]);
                assert_eq!(seg.prod(l..r), expected, "mismatch at l={}, r={}", l, r);
            }
        }
    }

    #[test]
    fn n_is_power_of_two() {
        let seg = Segtree::<SumI64>::from(vec![1i64, 2, 3, 4]);
        assert_eq!(seg.prod(0..4), 10);
        assert_eq!(seg.prod(1..3), 5);
    }

    #[test]
    fn n_is_one() {
        let mut seg = Segtree::<SumI64>::from(vec![42i64]);
        assert_eq!(seg.prod(0..1), 42);
        assert_eq!(seg.prod(0..0), 0);
        assert_eq!(seg.all_prod(), 42);
        seg.set(0, 7);
        assert_eq!(seg.all_prod(), 7);
    }

    #[test]
    fn n_is_zero() {
        let seg = Segtree::<SumI64>::new(0);
        assert_eq!(seg.all_prod(), 0);
        assert_eq!(seg.prod(..), 0);
        assert_eq!(seg.prod(0..0), 0);
    }

    #[test]
    fn prod_full_range_matches_all_prod_for_all_syntaxes() {
        let seg = Segtree::<SumI64>::from(vec![1i64, 2, 3, 4, 5]);
        let n = 5;
        let expected = seg.all_prod();
        assert_eq!(seg.prod(..), expected);
        assert_eq!(seg.prod(0..n), expected);
        assert_eq!(seg.prod(..n), expected);
        assert_eq!(seg.prod(0..), expected);
        assert_eq!(seg.prod(0..=n - 1), expected);
    }
}
