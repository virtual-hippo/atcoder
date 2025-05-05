use proconio::{input, marker::Chars};

#[derive(Clone, Copy)]
pub struct Pos(usize, usize);

#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    pub fn into_char(&self) -> char {
        match self {
            Dir::Up => 'U',
            Dir::Down => 'D',
            Dir::Left => 'L',
            Dir::Right => 'R',
        }
    }
    pub fn from_char(c: char) -> Self {
        match c {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid direction"),
        }
    }
}
impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_char())
    }
}

/// 空きマスを '.', 障害物のあるマスを '#' としたグリッド
#[derive(Clone)]
pub struct Grid {
    /// グリッドの行数
    h: usize,
    /// グリッドの列数
    w: usize,
    /// グリッドの状態
    grid: Vec<Vec<char>>,
    /// 現在のポジション
    pos: Pos,
}

impl Grid {
    /// 新しいグリッドを作成する
    pub fn new(h: usize, w: usize, pos: Pos) -> Self {
        input! {
            s: [Chars; h],
        }

        Self { h, w, grid: s, pos }
    }

    /// グリッドを90度右回転させる
    pub fn rotate90(&self) -> Self {
        let mut new = vec![vec![' '; self.w]; self.h];
        for i in 0..self.h {
            for j in 0..self.w {
                new[j][self.h - 1 - i] = self.grid[i][j];
            }
        }

        Self {
            h: self.h,
            w: self.w,
            grid: new,
            pos: self.pos,
        }
    }

    /// グリッドの内容を表示する
    pub fn eprint(&self) {
        for i in 0..self.h {
            for j in 0..self.w {
                eprint!("{}", self.grid[i][j]);
            }
            eprintln!();
        }
    }

    pub fn can_move_pos(&self, dir: Dir) -> bool {
        let Pos(i, j) = self.pos;

        match dir {
            Dir::Up => i > 0 && self.grid[i - 1][j] == '.',
            Dir::Down => i < self.h - 1 && self.grid[i + 1][j] == '.',
            Dir::Left => j > 0 && self.grid[i][j - 1] == '.',
            Dir::Right => j < self.w - 1 && self.grid[i][j + 1] == '.',
        }
    }

    /// 現在地を変更する
    pub fn move_pos(&mut self, dir: Dir) -> Result<(), ()> {
        if !self.can_move_pos(dir) {
            return Err(());
        }

        match dir {
            Dir::Up => self.pos.0 -= 1,
            Dir::Down => self.pos.0 += 1,
            Dir::Left => self.pos.1 -= 1,
            Dir::Right => self.pos.1 += 1,
        }

        Ok(())
    }
}
