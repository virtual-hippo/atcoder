use proconio::input;

fn a_run_lengths(s: &str) -> Vec<usize> {
    s.split(|c: char| c != 'A').map(|group| group.len()).collect()
}

fn remove_a(s: &str) -> String {
    s.chars().filter(|&ch| ch != 'A').collect()
}

fn main() {
    input! {
        s: String,
        t: String,
    }

    if remove_a(&s) != remove_a(&t) {
        println!("-1");
        return;
    }

    let sv = a_run_lengths(&s);
    let tv = a_run_lengths(&t);
    let ans: usize = (0..sv.len()).map(|i| sv[i].abs_diff(tv[i])).sum();
    println!("{}", ans);
}
