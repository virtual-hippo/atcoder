use proconio::{fastout, input, marker::Chars};

fn func1(n: usize, s: &Vec<char>, c: &Vec<u64>, m: usize) -> Vec<u64> {
    let mut s_cloned = s.clone();
    let mut cost = 0;
    let mut ret = vec![0; n];
    for i in 0..n {
        if i % 2 == m && s[i] == '1' {
            s_cloned[i] = '0';
            cost += c[i]
        }
        if i % 2 != m && s[i] == '0' {
            s_cloned[i] = '1';
            cost += c[i]
        }
        ret[i] = cost;
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [u64; n],
    }

    let costs1 = func1(n, &s, &c, 1);
    let costs2 = func1(n, &s, &c, 0);

    let mut ans = u64::MAX;
    for i in 0..n - 1 {
        let current1 = costs1[i] + (costs2[n - 1] - costs2[i]);
        let current2 = costs2[i] + (costs1[n - 1] - costs1[i]);
        ans = ans.min(current1.min(current2));
    }

    println!("{}", ans);
}
