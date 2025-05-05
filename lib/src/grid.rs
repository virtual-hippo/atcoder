use proconio::{input, marker::Chars};

#[derive(Clone, Copy)]
pub struct Pos(usize, usize);

#[derive(Clone, Copy)]
pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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
            Dir::UP => i > 0 && self.grid[i - 1][j] == '.',
            Dir::DOWN => i < self.h - 1 && self.grid[i + 1][j] == '.',
            Dir::LEFT => j > 0 && self.grid[i][j - 1] == '.',
            Dir::RIGHT => j < self.w - 1 && self.grid[i][j + 1] == '.',
        }
    }

    /// 現在地を変更する
    pub fn move_pos(&mut self, dir: Dir) -> Result<(), ()> {
        if !self.can_move_pos(dir) {
            return Err(());
        }

        match dir {
            Dir::UP => self.pos.0 -= 1,
            Dir::DOWN => self.pos.0 += 1,
            Dir::LEFT => self.pos.1 -= 1,
            Dir::RIGHT => self.pos.1 += 1,
        }

        Ok(())
    }
}
