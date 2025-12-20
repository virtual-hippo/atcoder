use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

fn get_sheet() -> Vec<Vec<char>> {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    }
    s
}

fn get_positions(sheet: &[Vec<char>]) -> Vec<(i32, i32)> {
    sheet
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect()
}

fn shift_positions(pos: &[(i32, i32)], dr: i32, dc: i32) -> HashSet<(i32, i32)> {
    pos.iter().map(|&(r, c)| (r + dr, c + dc)).collect()
}

#[fastout]
fn main() {
    let a = get_sheet();
    let b = get_sheet();
    let x = get_sheet();

    let pos_a = get_positions(&a);
    let pos_b = get_positions(&b);
    let pos_x = get_positions(&x);
    let set_x: HashSet<_> = pos_x.iter().cloned().collect();

    // Aの最初の'#'をXの各'#'に配置
    for &(xr, xc) in &pos_x {
        let (ar, ac) = pos_a[0];
        let set_a = shift_positions(&pos_a, xr - ar, xc - ac);

        // Aの全'#'がXの'#'上にあるか確認
        if !set_a.is_subset(&set_x) {
            continue;
        }

        // Bの最初の'#'をXの各'#'に配置
        for &(yr, yc) in &pos_x {
            let (br, bc) = pos_b[0];
            let set_b = shift_positions(&pos_b, yr - br, yc - bc);

            // Bの全'#'がXの'#'上にあるか確認
            if !set_b.is_subset(&set_x) {
                continue;
            }

            // A ∪ B = X か確認
            if set_a.union(&set_b).count() == set_x.len() {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
