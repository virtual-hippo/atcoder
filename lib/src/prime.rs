/// 約数一覧を取得
pub fn get_primes(n: usize) -> Vec<usize> {
    let mut i = 1;
    let mut ret = vec![];
    while i * i <= n {
        if i * i == n {
            ret.push(i);
        } else if n % i == 0 {
            ret.push(i);
            ret.push(n / i);
        }
        i += 1;
    }
    ret
}
