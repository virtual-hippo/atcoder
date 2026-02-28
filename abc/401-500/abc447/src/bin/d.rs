use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let (_, _, ans) = s.chars().fold((0_usize, 0, 0), |(a, b, c), ch| match ch {
        'A' => (a + 1, b, c),
        'B' => (a, a.min(b + 1), c),
        'C' => (a, b, b.min(c + 1)),
        _ => unreachable!(),
    });
    println!("{}", ans);
}
