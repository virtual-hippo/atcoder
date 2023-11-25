use proconio::input;
use proconio::marker::Chars;

fn _print(p: &Vec<Vec<char>>) {
    for i in 0..p.len() {
        for j in 0..p[i].len() {
            print!("{}", p[i][j]);
        }
        println!();
    }
}

fn rotate(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if p.len() != p[0].len() {
        panic!()
    }
    let mut ret = vec![vec!['.'; p.len()]; p.len()];
    for i in 0..p.len() {
        for j in 0..p[i].len() {
            ret[j][p[i].len() - 1 - i] = p[i][j];
        }
    }

    ret
}

fn write(
    p: &Vec<Vec<char>>,
    background: &mut Vec<Vec<char>>,
    (dx, dy): (usize, usize),
) -> Option<()> {
    for i in 0..4 {
        for j in 0..4 {
            if p[i][j] == '#' {
                if background[i + dx][j + dy] == '#' {
                    return None;
                }
                background[i + dx][j + dy] = p[i][j];
            }
        }
    }
    Some(())
}

fn ok(background: &Vec<Vec<char>>) -> bool {
    for dx in 0..4 {
        for dy in 0..4 {
            let mut flag = true;
            for i in 0..8 {
                for j in 0..8 {
                    if dx <= i && i < dx + 4 && dy <= j && j < dy + 4 {
                        if background[i][j] == '.' {
                            flag = false;
                        }
                    } else {
                        if background[i][j] == '#' {
                            flag = false;
                        }
                    }
                }
            }
            if flag {
                return true;
            }
        }
    }

    false
}

fn main() {
    input! {
        p1: [Chars; 4],
        p2: [Chars; 4],
        p3: [Chars; 4],
    }
    let mut current_p1 = p1;
    for _i1 in 0..4 {
        for x1 in 0..4 {
            for y1 in 0..4 {
                let mut background1 = vec![vec!['.'; 8]; 8];
                if write(&current_p1, &mut background1, (x1, y1)).is_none() {
                    continue;
                }
                let mut current_p2 = p2.clone();
                for _i2 in 0..4 {
                    for x2 in 0..4 {
                        for y2 in 0..4 {
                            let mut background2 = background1.clone();
                            if write(&current_p2, &mut background2, (x2, y2)).is_none() {
                                continue;
                            }
                            let mut current_p3 = p3.clone();
                            for _i3 in 0..4 {
                                for x3 in 0..4 {
                                    for y3 in 0..4 {
                                        let mut background3 = background2.clone();
                                        if write(&current_p3, &mut background3, (x3, y3)).is_none()
                                        {
                                            continue;
                                        }
                                        if ok(&background3) {
                                            println!("Yes");
                                            return;
                                        }
                                    }
                                }
                                current_p3 = rotate(&current_p3);
                            }
                        }
                    }
                    current_p2 = rotate(&current_p2);
                }
            }
        }
        current_p1 = rotate(&current_p1);
    }
    println!("No");
}
