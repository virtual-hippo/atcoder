use proconio::input;
use std::collections::*;

fn main() {
    input! {
        s: String,
    }

    let map = s.chars().fold(HashMap::new(), |mut map, ch| {
        *map.entry(ch).or_insert(0) += 1;
        map
    });
    let (_, v) = map.iter().map(|(&ch, &x)| (ch, x)).max_by_key(|(_, v)| *v).unwrap();

    let ans = s.chars().filter(|c| map[c] != v).collect::<String>();
    println!("{}", ans);
}
