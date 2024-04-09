use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }
    let s = "wbwbwwbwbwbw".to_string().chars().collect::<Vec<char>>();
    let mut d = 0;
    let mut i = d;
    let (mut ww, mut bb) = (0, 0);
    loop {
        if s[i] == 'w' {
            ww += 1;
        } else {
            bb += 1;
        }
        if (ww, bb) == (w, b) {
            println!("Yes");
            return;
        }
        if ww > w || bb > b {
            if d < s.len() - 1 {
                d += 1;
                (ww, bb) = (0, 0);
                i = d;
                continue;
            } else {
                println!("No");
                return;
            }
        }
        i += 1;
        i %= s.len();
    }
}
