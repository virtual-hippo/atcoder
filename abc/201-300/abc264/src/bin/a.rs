use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let atcoder = vec!['a','t','c','o','d','e','r'];
    for i in l-1..r {
        print!("{}", atcoder[i]);
    }
}

