use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        m: usize,
    }
    let h_str = format!("{:>02}", h);
    let m_str = format!("{:>02}", m);

    let h_vec = h_str.chars().map(|ch| ch as u8 - b'0').collect::<Vec<u8>>();
    let m_vec = m_str.chars().map(|ch| ch as u8 - b'0').collect::<Vec<u8>>();

    let [mut a, mut b] = [h_vec[0], h_vec[1]];
    let [mut c, mut d] = [m_vec[0], m_vec[1]];

    while !(a == 0 && b == 0 && c == 0 && d == 0) {
        let rev = (a * 10 + c, b * 10 + d);
        if rev.0 < 24 && rev.1 < 60 {
            println!("{}{} {}{}", a, b, c, d);
            return;
        }

        d += 1;
        if d == 10 {
            c += 1;
            d = 0;
        }
        if c == 6 {
            c = 0;
            b += 1;
        }

        if a == 2 && b == 4 {
            a = 0;
            b = 0;
            continue;
        }

        if b == 10 {
            a += 1;
            b = 0;
        }
    }

    println!("0 0");
}
