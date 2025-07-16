use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut clients = vec![usize::MAX; n];
    let mut head = usize::MAX;
    let mut commits = vec![];

    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            // pull
            1 => {
                input! {
                    p: usize,
                }
                let p = p - 1;
                clients[p] = head;
            },
            // commit
            2 => {
                input! {
                    p: usize,
                    s: String,
                }
                let p = p - 1;

                commits.push((clients[p], s.clone()));
                clients[p] = commits.len() - 1;
            },
            // push
            3 => {
                input! {
                    p: usize,
                }
                let p = p - 1;
                head = clients[p];
            },
            _ => unreachable!(),
        }
    }

    let mut ans = vec![];
    while head != usize::MAX {
        ans.push(commits[head].1.clone());
        head = commits[head].0;
    }
    ans.reverse();

    let ans = ans.join("");
    println!("{}", ans);
}
