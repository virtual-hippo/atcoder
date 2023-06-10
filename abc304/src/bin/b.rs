// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    if n <= 10_u32.pow(3) - 1 {
        println!("{}", n);
    } else if n <= 10_u32.pow(4) - 1 {
        println!("{}", (n / 10) * 10);
    } else if n <= 10_u32.pow(5) - 1 {
        println!("{}", (n / 100) * 100);
    } else if n <= 10_u32.pow(6) - 1 {
        println!("{}", (n / 1000) * 1000);
    } else if n <= 10_u32.pow(7) - 1 {
        println!("{}", (n / 10000) * 10000);
    } else if n <= 10_u32.pow(8) - 1 {
        println!("{}", (n / 100000) * 100000);
    } else if n <= 10_u32.pow(9) - 1 {
        println!("{}", (n / 1000000) * 1000000);
    }
//     N が 103−1 以下ならば、
// N をそのまま出力する。
// N が 103 以上 104−1 以下ならば、
// N の一の位を切り捨てて出力する。
// N が 104 以上 105−1 以下ならば、
// N の十の位以下を切り捨てて出力する。
// N が 105 以上 106−1 以下ならば、
// N の百の位以下を切り捨てて出力する。
// N が 106 以上 107−1 以下ならば、
// N の千の位以下を切り捨てて出力する。
// N が 107 以上 108−1 以下ならば、
// N の一万の位以下を切り捨てて出力する。
// N が 108 以上 109−1 以下ならば、N の十万の位以下を切り捨てて出力する。
    
}

