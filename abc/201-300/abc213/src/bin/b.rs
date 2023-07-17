use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let sorted = {
        let mut ret = a.iter().enumerate().map(|(i, &v)|(i,v)).collect::<Vec<(usize,usize)>>();
        ret.sort_by(|(_,a), (_,b)| b.cmp(a));
        ret
    };
    println!("{}", sorted[1].0 + 1);
}
