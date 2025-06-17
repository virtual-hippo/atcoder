use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 7],
    }

    let mut bucket = [0; 14];
    for &x in &a {
        bucket[x] += 1;
    }

    for i in 1..13 {
        for j in i + 1..=13 {
            if bucket[i] > 1 && bucket[j] > 2 {
                println!("Yes");
                return;
            }
            if bucket[j] > 1 && bucket[i] > 2 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
