use proconio::{fastout, input};

fn is_kaibun_num(base: u64, n: u64) -> bool {
    let digits: Vec<u64> = std::iter::successors(Some(n), |&num| if num > 0 { Some(num / base) } else { None })
        .take_while(|&num| num > 0)
        .map(|num| num % base)
        .collect();

    let len = digits.len();
    len > 0 && (0..len / 2).all(|i| digits[i] == digits[len - 1 - i])
}

#[fastout]
fn main() {
    input! {
        a: u64,
        n: u64,
    }

    let mut ans = 0;

    for i in 1..1_000_000 {
        let si = i.to_string();
        let rsi = si.chars().rev().collect::<String>();

        let mut target_s = format!("{}{}", si, rsi);
        let target = target_s.parse::<u64>().unwrap();
        if target <= n && is_kaibun_num(a, target) {
            ans += target;
        }

        target_s.remove(target_s.len() / 2);
        let target = target_s.parse::<u64>().unwrap();
        if target <= n && is_kaibun_num(a, target) {
            ans += target;
        }
    }

    println!("{}", ans);
}
