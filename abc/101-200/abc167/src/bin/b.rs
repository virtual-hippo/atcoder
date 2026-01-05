use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        k: i64,
    }
    let cards = [(a, 1), (b, 0), (c, -1)];

    let (ans, _) = cards.iter().fold((0, k), |(score, remaining), &(count, value)| {
        let take = count.min(remaining);
        (score + take * value, remaining - take)
    });
    println!("{}", ans);
}
