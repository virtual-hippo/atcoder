use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut stack = vec![];

    let mut ans = a.len();

    for i in 0..n {
        if stack.len() == 0 {
            stack.push((a[i], 1));
            continue;
        }

        let tail = stack.len() - 1;

        if stack[tail].0 == a[i] {
            stack[tail].1 += 1;

            if stack[tail].1 >= 4 {
                stack[tail].1 -= 4;
                ans -= 4;

                if stack[tail].1 == 0 {
                    stack.pop();
                }
            }
        } else {
            stack.push((a[i], 1));
        }
    }

    println!("{}", ans);
}
