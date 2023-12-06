use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut answers = vec![0_u64];
    for i in 0..61 {
        if n & (1 << i) != 0 {
            let len = answers.len();
            for j in 0..len {
                answers.push(answers[j] | 1 << i);
            }
        }
    }
    for answer in answers {
        println!("{}", answer);
    }
}
