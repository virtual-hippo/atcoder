use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut amebas = vec![(1, 0)];
    for i in 0..n {
        let parent = amebas[a[i] - 1];
        let child1 = (parent.0 * 2, parent.1 + 1);
        let child2 = (parent.0 * 2 + 1 + 1, parent.1 + 1);
        amebas.push(child1);
        amebas.push(child2);
    }
    for ameba in amebas.iter() {
        println!("{}", ameba.1);
    }
}
