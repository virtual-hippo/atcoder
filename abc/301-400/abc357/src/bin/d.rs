use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    const M: u64 = 998_244_353;

    let k = count_digits(n);
    let r = 10_u64.pow(k) % M;

    let rn = modular_multiplicative_inverse::power(r, n, M);
    let r1_inv = modular_multiplicative_inverse::modular_inv(r - 1, M);

    let ans = n % M * (rn - 1) % M * r1_inv % M;
    println!("{}", ans);
}

/// 桁数を返す関数
fn count_digits(n: u64) -> u32 {
    if n < 10 {
        return 1;
    }
    1 + count_digits(n / 10)
}

pub mod modular_multiplicative_inverse {
    /// a の 逆元を m で割った余りを返す関数
    pub fn modular_inv(a: u64, m: u64) -> u64 {
        power(a, m - 2, m)
    }

    /// a の b 乗を m で割った余りを返す関数
    /// 繰り返し二乗法を用いて高速に計算する
    pub fn power(a: u64, b: u64, m: u64) -> u64 {
        if b == 0 {
            return 1;
        }

        if b % 2 == 0 {
            let v = power(a, b / 2, m) % m;
            v * v % m
        } else {
            let v = power(a, b / 2, m) % m;
            v * v % m * a % m
        }
    }
}
