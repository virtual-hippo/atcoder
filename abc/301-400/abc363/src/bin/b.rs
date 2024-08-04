use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        l: [usize; n],
    }
    let mut i = 0;
    loop {
        let cnt = l.iter().filter(|&&v| v + i >= t).count();
        if cnt >= p {
            println!("{}", i);
            return;
        }
        i += 1;
    }
}
