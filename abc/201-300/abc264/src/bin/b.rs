use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
    }
    let is_black = {
        ((1 <= r && r <= 15) && (c == 1 || c == 15) ) ||
        ((1 <= c && c <= 15) && (r == 1 || r == 15) ) ||
        ((3 <= r && r <= 13) && (c == 3 || c == 13) ) ||
        ((3 <= c && c <= 13) && (r == 3 || r == 13) ) ||
        ((5 <= r && r <= 11) && (c == 5 || c == 11) ) ||
        ((5 <= c && c <= 11) && (r == 5 || r == 11) ) ||
        ((7 <= r && r <= 9) && (c == 7 || c == 9) ) ||
        ((7 <= c && c <= 9) && (r == 7 || r == 9) )
    };
    if is_black {
        println!("black")
    } else {
        println!("white")
    }
}

