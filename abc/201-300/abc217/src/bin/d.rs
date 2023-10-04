use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        l: usize,
        q: usize,
    }

    let mut mokuzai = BTreeSet::new();
    mokuzai.insert(0);
    mokuzai.insert(l);

    for _ in 0..q {
        input! {
           c: usize,
           x: usize,
        }
        match c {
            1 => {
                mokuzai.insert(x);
            }
            2 => {
                println!(
                    "{}",
                    mokuzai.range(x..).next().unwrap() - mokuzai.range(..x).last().unwrap()
                );
            }
            _ => unreachable!(),
        }
    }
}
