use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    let mut like_numbers = Vec::new();

    for bit in 0..(1 << 10) {
        let mut x: usize = 0;
        for i in (0..10).rev() {
            if bit & (1 << i) == 0 {
                x *= 10;
                x += i;
            }
        }
        if x > 0 {
            like_numbers.push(x);
        }
    }
    like_numbers.sort();
    println!("{}", like_numbers[k - 1]);
}
