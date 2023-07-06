use proconio::input;
use proconio::marker::Chars;

struct Sheet {
    h: usize,
    w: usize,
    sheet: Vec<Vec<char>>
}

impl Sheet {
    fn new(h: usize, w: usize, sheet: Vec<Vec<char>>) -> Self {
        Sheet {
            h, w, sheet
        }
    }

    fn new_by_input() -> Self {
        input! {
            (h,w): (usize, usize),
            sheet: [Chars; h],
        }
        Sheet {
            h, w, sheet
        }
    }

    fn same(&self, other: &Self, pos: (usize, usize)) -> bool {
        for i in 0..other.h {
            for j in 0..other.w {
                if self.sheet[pos.0+i][pos.1+j] != other.sheet[i][j] {
                    return false;
                }
            }
        }
        true
    }

    

    fn contain_black(&self, other: &Self, pos: (usize, usize), banpei: (usize, usize)) -> bool {
        for i in 0..other.h {
            for j in 0..other.w {
                if other.sheet[i][j] == '#' {
                    if self.sheet[pos.0+i][pos.1+j] == '.' {
                        return false;
                    }
                    if banpei.0 <= pos.0+i || banpei.1 <= pos.1+j {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn _print(&self) {
        for i in 0..self.h {
            for j in 0..self.w {
                print!("{}", self.sheet[i][j]);
            }
            println!("");
        }
    }
}


fn stack(base: &mut Sheet, other: &Sheet, pos: (usize, usize)) {
    if pos.0 + other.h > base.h || pos.1 + other.w > base.w {
        println!("muri");
        return;
    }

    for i in 0..other.h {
        for j in 0..other.w {
            if other.sheet[i][j] == '#' {
                base.sheet[pos.0+i][pos.1+j] = '#';
            }
        }
    }
}

fn check(current: &Sheet, a: &Sheet, b: &Sheet, x: &Sheet) -> bool {
    
    for i in 0..current.h + 1 - x.h {
        for j in 0..current.w + 1 - x.w {
            if current.same(x, (i,j)) {
                let a_ok = {
                    let mut ret_a = false;
                    for k in i..i+a.h {
                        for l in j..j+a.w {
                            if current.contain_black(a, (k,l), (i+x.h, j+x.w)) {
                                ret_a = true;
                            }
                        }
                    }
                    ret_a
                };
                let b_ok = {
                    let mut ret_b = false;
                    for k in i..i+b.h {
                        for l in j..j+b.w {
                            if current.contain_black(b, (k,l), (i+x.h, j+x.w)) {
                                ret_b = true;
                            }
                        }
                    }
                    ret_b
                };
                return a_ok && b_ok;
            }
        }
    }
    false
}

fn get_base(b: &Sheet) -> Sheet {
    let mut ret = Sheet::new(30, 30, vec![vec!['.'; 30]; 30]);
    for i in 10..10+b.h {
        for j in 10..10+b.w {
            if b.sheet[i-10][j-10] == '#' {
                ret.sheet[i][j] = '#';
            }
        }
    }
    ret
}

fn main() {
    let a = Sheet::new_by_input();
    let b = Sheet::new_by_input();
    let x = Sheet::new_by_input();

    for i in 0..30-a.h {
        for j in 0..30-a.w {
            let mut base = get_base(&b);
            stack(&mut base, &a, (i,j));
            
            if check(&base, &a, &b, &x) {
                println!("Yes");
                return;
            }
        }
    }
    
    println!("No");
}

