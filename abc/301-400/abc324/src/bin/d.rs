use proconio::input;

type Bucket = [u8; 10];

fn digit_counts(mut x: u64) -> Bucket {
    let mut counts = [0u8; 10];
    if x == 0 {
        counts[0] = 1;
    } else {
        while x > 0 {
            counts[(x % 10) as usize] += 1;
            x /= 10;
        }
    }
    counts
}

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let bucket = s.chars().map(|ch| (ch as u8 - b'0')).fold([0; 10], |mut acc: Bucket, x: u8| {
        acc[x as usize] += 1;
        acc
    });

    let mx = 10_u64.pow(n as u32);

    let ans = (0..)
        .take_while(|x| x * x < mx)
        .map(|x| x * x)
        .map(|x| {
            let digits_diff = n - if x == 0 { 1 } else { x.ilog10() as usize + 1 };
            if digits_diff > 0 {
                let mut b = digit_counts(x);
                b[0] += digits_diff as u8;
                b
            } else {
                digit_counts(x)
            }
        })
        .filter(|xs| *xs == bucket)
        .count();

    println!("{}", ans);
}
