use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t],
    }
    let mut rows = vec![0; n];
    let mut cols = vec![0; n];
    let mut diagonal0 = 0;
    let mut diagonal1 = 0;

    for i in 0..t {
        // yoko
        let ii = (a[i] - 1) / n;
        rows[ii] += 1;
        if rows[ii] == n {
            println!("{}", i + 1);
            return;
        }

        // tate
        let jj = (a[i] - 1) % n;
        cols[jj] += 1;
        if cols[jj] == n {
            println!("{}", i + 1);
            return;
        }

        // migisagari
        if ii == jj {
            diagonal0 += 1;
        }
        if diagonal0 == n {
            println!("{}", i + 1);
            return;
        }

        // migiagari
        if (n - 1) - ii == jj {
            diagonal1 += 1;
        }
        if diagonal1 == n {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", -1);
}
