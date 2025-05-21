use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    let s_run_length = s.chars().dedup_with_count().collect_vec();
    let t_run_length = t.chars().dedup_with_count().collect_vec();

    if s_run_length.len() != t_run_length.len() {
        println!("No");
        return;
    }

    for i in 0..s_run_length.len() {
        if s_run_length[i].1 != t_run_length[i].1 {
            println!("No");
            return;
        }
        if s_run_length[i].0 > t_run_length[i].0 {
            println!("No");
            return;
        }
        if s_run_length[i].0 < t_run_length[i].0 && s_run_length[i].0 == 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
