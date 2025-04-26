#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};
use std::{collections::VecDeque, usize};
use superslice::Ext;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Input {
    n: usize,       // n=20 (座標)
    m: usize,       // m=40 (目的地の個数)
    goal: Vec<Pos>, // 目的地の座標
}

impl Input {
    fn from_stdin() -> Self {
        input! {
            n: usize, // 都市の個数 n=800
            m: usize, // 都市のグループの個数 1 <= m <= 400
        }

        let mut goal = Vec::with_capacity(m);
        for _ in 0..m {
            input! {
                x: usize,
                y: usize,
            }
            goal.push(Pos { x, y });
        }

        Self { n, m, goal }
    }
}

#[derive(Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn into_char(&self) -> char {
        match self {
            Dir::Up => 'U',
            Dir::Down => 'D',
            Dir::Left => 'L',
            Dir::Right => 'R',
        }
    }
}
impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_char())
    }
}

#[derive(Clone)]
enum Action {
    Move(Dir),
    Slide(Dir),
    Apply(Dir),
}
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Apply(dir) => write!(f, "A {}", dir),
            Action::Move(dir) => write!(f, "M {}", dir),
            Action::Slide(dir) => write!(f, "S {}", dir),
        }
    }
}

#[derive(Clone)]
struct Info {
    state: Vec<Vec<char>>, // 座標の状態
    next: usize,           // 次の目的地
    last_visited: usize,   // 最後に訪れた目的地
    now_pos: Pos,          // 現在の座標
    hisotry: Vec<Action>,  // 行動履歴
    start_time: Instant,
    time_limit: Duration,
}

impl Info {
    fn new(input: &Input) -> Self {
        let state = vec![vec!['.'; input.n]; input.n];
        let time_limit = Duration::from_millis(1935);
        let start_time = Instant::now();

        Self {
            state,
            next: 1,
            last_visited: 0,
            now_pos: input.goal[0],
            hisotry: vec![],
            time_limit: Duration::from_millis(1935),
            start_time: Instant::now(),
        }
    }

    fn is_time_up(&self) -> bool {
        self.start_time.elapsed() >= self.time_limit
    }
}

/// 出力した行動列の長さを T、訪れることが出来た目的地の数をm としたとき、以下のスコアが得られる。
/// m<M−1 の場合、 m+1
/// m=M−1 の場合、M+2NM−T
fn calculate_score(input: &Input, info: &Info) -> usize {
    if info.last_visited < input.m - 1 {
        info.last_visited
    } else {
        input.m + 2 * input.n * input.m - info.hisotry.len()
    }
}

fn print_result(info: &Info) {
    for action in &info.hisotry {
        println!("{}", action);
    }
}

/// 2点間のマンハッタン距離を計算する
fn calculate_distance(a: &Pos, b: &Pos) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

/// 現在地から進む方向において最初にぶつかる壁を探す
fn find_latest_wall(dir: &Dir, info: &Info) -> Pos {
    match dir {
        Dir::Up => {
            for i in (0..info.now_pos.x).rev() {
                if info.state[i][info.now_pos.y] == '#' {
                    return Pos { x: i + 1, y: info.now_pos.y };
                }
            }
            Pos { x: 0, y: info.now_pos.y }
        },
        Dir::Down => {
            for i in info.now_pos.x + 1..info.state.len() {
                if info.state[i][info.now_pos.y] == '#' {
                    return Pos { x: i - 1, y: info.now_pos.y };
                }
            }
            Pos {
                x: info.state.len() - 1,
                y: info.now_pos.y,
            }
        },
        Dir::Left => {
            for i in (0..info.now_pos.y).rev() {
                if info.state[info.now_pos.x][i] == '#' {
                    return Pos { x: info.now_pos.x, y: i + 1 };
                }
            }
            Pos { x: info.now_pos.x, y: 0 }
        },
        Dir::Right => {
            for i in info.now_pos.y + 1..info.state[0].len() {
                if info.state[info.now_pos.x][i] == '#' {
                    return Pos { x: info.now_pos.x, y: i - 1 };
                }
            }
            Pos {
                x: info.now_pos.x,
                y: info.state[0].len() - 1,
            }
        },
    }
}

/// 以下の2つの方法の内，次の目的地との距離が近くなるのはどれかを判定し実行する
/// 1. Move: 上下左右どれかに1マスだけ移動する
/// 2. Slide: 上下左右どれかに可能な限り移動する (現在の座標 (x,y) とすると (0,y) or (x,0) or (n,y) or (x,n) となるように移動する)
fn do_best_action(input: &Input, info: &mut Info, next_goal: &Pos) {
    let now = info.now_pos;

    // 現在地から次の目的地までのマンハッタン距離
    let current_dist = calculate_distance(&now, &next_goal);

    // 各方向への移動後の距離を計算
    let mut best_action = None;
    let mut min_dist = current_dist;

    // Move actions
    let moves = [
        (Dir::Up, Pos { x: now.x.saturating_sub(1), y: now.y }),
        (
            Dir::Down,
            Pos {
                x: (now.x + 1).min(input.n - 1),
                y: now.y,
            },
        ),
        (Dir::Left, Pos { x: now.x, y: now.y.saturating_sub(1) }),
        (
            Dir::Right,
            Pos {
                x: now.x,
                y: (now.y + 1).min(input.n - 1),
            },
        ),
    ];

    for (dir, pos) in moves.iter() {
        if *pos == now {
            continue;
        } // Skip if can't move

        let dist = calculate_distance(&pos, &next_goal);

        if dist < min_dist {
            min_dist = dist;
            best_action = Some(Action::Move(dir.clone()));
        }
    }

    // Slide actions
    let slides = [
        (Dir::Up, find_latest_wall(&Dir::Up, info)),
        (Dir::Down, find_latest_wall(&Dir::Down, info)),
        (Dir::Left, find_latest_wall(&Dir::Left, info)),
        (Dir::Right, find_latest_wall(&Dir::Right, info)),
    ];

    for (dir, pos) in slides.iter() {
        if *pos == now {
            continue;
        } // Skip if already at edge

        let dist = calculate_distance(&pos, &next_goal);

        if dist < min_dist {
            min_dist = dist;
            best_action = Some(Action::Slide(dir.clone()));
        }
    }

    // Execute the best action
    if let Some(action) = best_action {
        match &action {
            Action::Move(dir) => match dir {
                Dir::Up => info.now_pos.x -= 1,
                Dir::Down => info.now_pos.x += 1,
                Dir::Left => info.now_pos.y -= 1,
                Dir::Right => info.now_pos.y += 1,
            },
            Action::Slide(dir) => match dir {
                Dir::Up => info.now_pos.x = 0,
                Dir::Down => info.now_pos.x = input.n - 1,
                Dir::Left => info.now_pos.y = 0,
                Dir::Right => info.now_pos.y = input.n - 1,
            },
            _ => {},
        }
        // eprintln!("Now: {} {}", info.now_pos.x, info.now_pos.y);
        info.hisotry.push(action);
    }
}

fn create_initial_answer(input: &Input, info: &mut Info) {
    while info.hisotry.len() < 2 * input.n * input.m && info.next < input.m {
        let next_goal = input.goal[info.next];
        do_best_action(input, info, &next_goal);

        // Check if we reached the next goal
        if info.now_pos == next_goal {
            info.last_visited = info.next;
            info.next += 1;
        }
    }
}

fn solve(input: &Input, info: &mut Info) {
    // 初期解作成
    create_initial_answer(input, info);

    print_result(info);
}

#[fastout]
fn main() {
    let input = Input::from_stdin();
    let mut info = Info::new(&input);
    solve(&input, &mut info);
}
