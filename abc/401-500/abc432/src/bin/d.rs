use ac_library::*;
use itertools::*;
use proconio::{fastout, input};

#[derive(Clone, Copy, Debug)]
struct Rect {
    lx: i64,
    ly: i64,
    rx: i64,
    ry: i64,
}

impl Rect {
    fn size(&self) -> i64 {
        (self.rx - self.lx) * (self.ry - self.ly)
    }

    fn is_touched(&self, other: &Self) -> bool {
        if (self.rx == other.lx || self.lx == other.rx) && (!(self.ry <= other.ly) && !(other.ry <= self.ly)) {
            true
        } else if (self.ry == other.ly || self.ly == other.ry) && (!(self.rx <= other.lx) && !(other.rx <= self.lx)) {
            true
        } else {
            false
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
    }

    let mut rects = vec![Rect { lx: 0, ly: 0, rx: x, ry: y }];

    for _ in 0..n {
        storm(&mut rects);
    }

    let m = rects.len();

    let mut dsu = Dsu::new(m);
    for a in 0..m - 1 {
        for b in a + 1..m {
            if rects[a].is_touched(&rects[b]) {
                dsu.merge(a, b);
            }
        }
    }
    let mut counter = vec![0; m];
    for a in 0..m {
        counter[dsu.leader(a)] += rects[a].size();
    }

    //
    let answer = counter.iter().filter(|&&v| v > 0).copied().sorted().collect_vec();
    println!("{}", answer.len());
    for (i, cnt) in answer.iter().enumerate() {
        if i == answer.len() - 1 {
            println!("{}", cnt);
        } else {
            print!("{} ", cnt);
        }
    }
}

fn storm(rects: &mut Vec<Rect>) {
    input! {
        c: char,
        a: i64,
        b: i64,
    }

    let mut new_rects = vec![];

    if c == 'X' {
        while let Some(mut rect) = rects.pop() {
            if rect.rx <= a {
                rect.ly -= b;
                rect.ry -= b;
                new_rects.push(rect);
            } else if a <= rect.lx {
                rect.ly += b;
                rect.ry += b;
                new_rects.push(rect);
            } else {
                new_rects.push(Rect {
                    lx: rect.lx,
                    ly: rect.ly - b,
                    rx: a,
                    ry: rect.ry - b,
                });

                new_rects.push(Rect {
                    lx: a,
                    ly: rect.ly + b,
                    rx: rect.rx,
                    ry: rect.ry + b,
                });
            }
        }
    }

    if c == 'Y' {
        while let Some(mut rect) = rects.pop() {
            if rect.ry <= a {
                rect.lx -= b;
                rect.rx -= b;
                new_rects.push(rect);
            } else if a <= rect.ly {
                rect.lx += b;
                rect.rx += b;
                new_rects.push(rect);
            } else {
                new_rects.push(Rect {
                    lx: rect.lx - b,
                    ly: rect.ly,
                    rx: rect.rx - b,
                    ry: a,
                });

                new_rects.push(Rect {
                    lx: rect.lx + b,
                    ly: a,
                    rx: rect.rx + b,
                    ry: rect.ry,
                });
            }
        }
    }

    std::mem::swap(rects, &mut new_rects);
}
