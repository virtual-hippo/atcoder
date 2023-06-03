use proconio::input;


fn main() {
    input! {
        _n: usize,
        a: i64,
        b: i64,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
    }
    let is_contain = |x: i64, y:i64| -> bool {
        let ret1 = b-a == y-x;
        let ret2 = b+a == y+x;
        ret1 || ret2
    };


    for i in p..=q {
        for j in r..=s {
            if is_contain(i as i64,j as i64) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

