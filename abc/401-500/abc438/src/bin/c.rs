use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let stack = a.iter().fold(Vec::new(), |mut stack, &x| {
        match stack.last_mut() {
            Some((val, count)) if *val == x && *count + 1 == 4 => {
                stack.pop();
            },
            Some((val, count)) if *val == x => {
                *count += 1;
            },
            _ => {
                stack.push((x, 1));
            },
        }
        stack
    });

    let ans: usize = stack.iter().map(|(_, count)| count).sum();
    println!("{}", ans);
}
