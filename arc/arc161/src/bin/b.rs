// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn f(x: u64) -> usize {
    let mut ret = 0;
    let string = format!("{:>018b}", x).to_string();
    for byte in string.into_bytes() {
        if byte == 49 {
            ret += 1;
        }
    }
    ret
}

fn f2(x: u64) -> u64 {
    let string = format!("{:>018b}", x).to_string();
    let mut cnt = 0;
    let mut ret = 0;
    for byte in string.into_bytes() {
        if byte == 49 {
            cnt += 1;
        }
        if cnt <= 3 {
            ret <<= 1;
            ret += byte as u64 -48
        } else {
            ret <<= 1;
        }
    }
    ret
}

fn f3(x: u64) -> u64 {
    let string = format!("{:>018b}", x).to_string();
    let mut cnt = 0;
    let mut base = 0;
    for byte in string.into_bytes() {
        if byte == 49 {
            cnt += 1;
        }
        if cnt <= 1 {
            base <<= 1;
            base += byte as u64 -48
        } else {
            base <<= 1;
        }
    }
    if  x <= base + 2 {
        f4(x)
    } else {
        let mut i = 0;
        while (3 << i) | base < x {
            i += 1;
        }
        (3 << (i-1)) | base
    }
}

fn f4(x: u64) -> u64 {
    let mut i = 0;
    while 7 << i < x {
        i += 1;
    }
    7 << (i-1)
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64,
        }
        if n < 7 {
            println!("-1");
        } else if f(n) == 3 {
            println!("{}", n);
        } else if 3 < f(n) {
            println!("{}", f2(n));
        } else if f(n) == 2 {
            println!("{}", f3(n));
        } else if f(n) == 1 {
            println!("{}", f4(n));
        }
    }
}

