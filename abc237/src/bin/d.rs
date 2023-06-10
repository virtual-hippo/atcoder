use proconio::input;
use proconio::marker::Chars;


fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut vec = vec![(n+1,n+1); n+1];
    for i in 0..n {
        if s[i] == 'L' {
            let (left,_) = vec[i];
            vec[i].0 = i+1;
            vec[i+1].1 = i;
            if left != n+1 {
                vec[i+1].0 = left;
                vec[left].1 = i + 1;
            }
        } else {
            let (_,right) = vec[i];
            vec[i].1 = i+1;
            vec[i+1].0 = i;
            if right != n+1 {
                vec[i+1].1 = right;
                vec[right].0 = i + 1;
            }
        }
    }
    let mut next = 0;
    while vec[next].0 != n+1 {
        next = vec[next].0;
    }
    while vec[next].1 != n+1 {
        print!("{} ", next);
        next = vec[next].1;
    }
    print!("{}", next);
}
