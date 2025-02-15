#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{fastout, input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, usize};
use superslice::Ext;

const COST_STATION: i32 = 5000;
const COST_RAIL: i32 = 100;

#[derive(Clone, Copy, Debug)]
struct Pos(usize, usize);

struct UnionFind {
    n: usize,
    /// 親頂点の番号
    parents: Vec<usize>,
    /// 連結成分の頂点の個数
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            n,
            parents: vec![usize::MAX; n * n],
            siz: vec![1; n * n],
        }
    }

    fn find_root(&mut self, idx: usize) -> usize {
        if self.parents[idx] == usize::MAX {
            return idx;
        }
        self.parents[idx] = self.find_root(self.parents[idx]);
        self.parents[idx]
    }

    fn is_same(&mut self, p: Pos, q: Pos) -> bool {
        let p_idx = p.0 * self.n + p.1;
        let q_idx = q.0 * self.n + q.1;
        self.find_root(p_idx) == self.find_root(q_idx)
    }

    fn unite(&mut self, p: Pos, q: Pos) {
        let p_idx = p.0 * self.n + p.1;
        let q_idx = q.0 * self.n + q.1;
        let p_root = self.find_root(p_idx);
        let q_root = self.find_root(q_idx);

        if self.siz[p_root] < self.siz[q_root] {
            self.parents[p_root] = q_root;
            self.siz[q_root] += self.siz[p_root];
        } else {
            self.parents[q_root] = p_root;
            self.siz[p_root] += self.siz[q_root];
        }
    }
}
fn calc_distance(pos1: Pos, pos2: Pos) -> i32 {
    let pos1 = (pos1.0 as i32, pos1.1 as i32);
    let pos2 = (pos2.0 as i32, pos2.1 as i32);

    (pos2.0 - pos1.0).abs() + (pos2.1 - pos1.1).abs()
}

#[derive(Clone, Debug)]
enum Building {
    Station,
    HorizontalRail,
    VerticalRail,
    LeftDownRail,
    LeftUpRail,
    RightUpRail,
    RightDownRail,
}

impl Building {
    fn is_station(&self) -> bool {
        matches!(self, Building::Station)
    }

    fn is_rail(&self) -> bool {
        matches!(
            self,
            Building::HorizontalRail
                | Building::VerticalRail
                | Building::LeftDownRail
                | Building::LeftUpRail
                | Building::RightUpRail
                | Building::RightDownRail
        )
    }

    fn is_up(&self) -> bool {
        matches!(
            self,
            Building::Station
                | Building::VerticalRail
                | Building::LeftUpRail
                | Building::RightUpRail
        )
    }

    fn is_down(&self) -> bool {
        matches!(
            self,
            Building::Station
                | Building::VerticalRail
                | Building::LeftDownRail
                | Building::RightDownRail
        )
    }

    fn is_left(&self) -> bool {
        matches!(
            self,
            Building::Station
                | Building::HorizontalRail
                | Building::LeftDownRail
                | Building::LeftUpRail
        )
    }

    fn is_right(&self) -> bool {
        matches!(
            self,
            Building::Station
                | Building::HorizontalRail
                | Building::RightDownRail
                | Building::RightUpRail
        )
    }
    fn to_string(&self) -> String {
        match self {
            Building::Station => "0".to_string(),
            Building::HorizontalRail => "1".to_string(),
            Building::VerticalRail => "2".to_string(),
            Building::LeftDownRail => "3".to_string(),
            Building::LeftUpRail => "4".to_string(),
            Building::RightUpRail => "5".to_string(),
            Building::RightDownRail => "6".to_string(),
        }
    }
}

enum Action {
    DoNothing,
    Build((Building, Pos)),
}

impl Action {
    fn to_string(&self) -> String {
        match self {
            Action::DoNothing => "-1".to_string(),
            Action::Build((building, pos)) => {
                format!("{} {} {}", building.to_string(), pos.0, pos.1)
            }
        }
    }
}

struct Answer<'a> {
    actions: &'a Vec<Action>,
    score: i32,
}

impl<'a> Answer<'a> {
    fn new(actions: &'a Vec<Action>, score: i32) -> Self {
        Self { actions, score }
    }

    fn to_string(&self) -> String {
        self.actions.iter().map(|a| a.to_string()).join("\n")
    }
}

struct Field {
    n: usize,
    fields: Vec<Vec<Option<Building>>>,
    uf: UnionFind,
}

impl Field {
    fn new(n: usize) -> Self {
        Self {
            n,
            fields: vec![vec![None; n]; n],
            uf: UnionFind::new(n),
        }
    }

    fn build(&mut self, building: &Building, Pos(r, c): Pos) {
        // すでに建てられている場合はエラー
        if let Some(ref existing) = self.fields[r][c] {
            debug_assert!(!existing.is_station());
            if building.is_rail() {
                debug_assert!(self.fields[r][c].is_none());
            }
        }
        self.fields[r][c] = Some(building.clone());

        // 隣接する区画と接続
        // 上
        if building.is_up() && r > 0 {
            if let Some(ref other) = self.fields[r - 1][c] {
                if other.is_down() {
                    self.uf.unite(Pos(r, c), Pos(r - 1, c));
                }
            }
        }
        // 下
        if building.is_down() && r < self.n - 1 {
            if let Some(ref other) = self.fields[r + 1][c] {
                if other.is_up() {
                    self.uf.unite(Pos(r, c), Pos(r + 1, c));
                }
            }
        }
        // 左
        if building.is_left() && c > 0 {
            if let Some(ref other) = self.fields[r][c - 1] {
                if other.is_right() {
                    self.uf.unite(Pos(r, c), Pos(r, c - 1));
                }
            }
        }
        // 右
        if building.is_right() && c < self.n - 1 {
            if let Some(ref other) = self.fields[r][c + 1] {
                if other.is_left() {
                    self.uf.unite(Pos(r, c), Pos(r, c + 1));
                }
            }
        }
    }

    fn is_connected(&mut self, s: Pos, t: Pos) -> bool {
        debug_assert!(calc_distance(s, t) > 4);

        let stations_s = self.collect_stations(&s);
        let stations_t = self.collect_stations(&t);

        for station_s in stations_s.iter() {
            for station_t in stations_t.iter() {
                if self.uf.is_same(*station_s, *station_t) {
                    return true;
                }
            }
        }
        false
    }

    fn collect_stations(&self, Pos(rr, cc): &Pos) -> Vec<Pos> {
        itertools::iproduct!(-2_i32..3, -2_i32..3)
            .filter(|&(dr, dc)| dr.abs() + dc.abs() <= 2)
            .map(|(dr, dc)| ((*rr as i32 + dr), (*cc as i32 + dc)))
            .filter(|&(r, c)| {
                r >= 0
                    && r < self.n as i32
                    && c >= 0
                    && c < self.n as i32
                    && self.fields[r as usize][c as usize].is_some()
                    && self.fields[r as usize][c as usize]
                        .as_ref()
                        .unwrap()
                        .is_station()
            })
            .map(|(r, c)| Pos(r as usize, c as usize))
            .collect()
    }
}

struct Solver {
    n: usize,
    m: usize,
    k: usize,
    t: usize,
    home: Vec<Pos>,
    workspace: Vec<Pos>,
    distance: Vec<i32>,
    is_connected: Vec<bool>,
    field: Field,
    money: i32,
    actions: Vec<Action>,
}

impl Solver {
    fn new(n: usize, m: usize, k: usize, t: usize) -> Self {
        Self {
            n,
            m,
            k,
            t,
            home: Vec::with_capacity(m),
            workspace: Vec::with_capacity(m),
            distance: Vec::with_capacity(m),
            is_connected: Vec::with_capacity(m),
            field: Field::new(n),
            money: k as i32,
            actions: Vec::with_capacity(t),
        }
    }

    fn update_is_connected(&mut self) {
        for i in 0..self.m {
            if !self.is_connected[i] && self.field.is_connected(self.home[i], self.workspace[i]) {
                self.is_connected[i] = true;
            }
        }
    }

    fn calc_income(&mut self) -> i32 {
        (0..self.m)
            .filter(|&i| self.is_connected[i])
            .map(|i| self.distance[i])
            .sum()
    }

    fn build_rail(&mut self, building: &Building, Pos(r, c): Pos) -> Result<(), ()> {
        if self.money < COST_RAIL {
            return Err(());
        }

        self.field.build(building, Pos(r, c));
        self.money -= COST_RAIL;
        self.actions
            .push(Action::Build((building.clone(), Pos(r, c))));
        Ok(())
    }

    fn build_station(&mut self, Pos(r, c): Pos) -> Result<(), ()> {
        if self.money < COST_RAIL {
            return Err(());
        }
        self.field.build(&Building::Station, Pos(r, c));
        self.money -= COST_STATION;
        self.actions
            .push(Action::Build((Building::Station, Pos(r, c))));
        Ok(())
    }

    fn build_nothing(&mut self) {
        self.actions.push(Action::DoNothing);
    }

    fn replace_do_nothing(&mut self, start_idx: usize) {
        for i in start_idx..self.actions.len() {
            self.actions[i] = Action::DoNothing;
        }
    }

    fn prosess_one(&mut self) -> Result<(), ()> {
        // 接続する人を見つける
        let pi = {
            let rail_count = (self.k as i32) - COST_STATION * 2;
            let mut pi = 0;
            while pi < self.m {
                if calc_distance(self.home[pi], self.workspace[pi]) - 1 <= rail_count {
                    break;
                }
                pi += 1;
            }
            pi
        };
        debug_assert_ne!(pi, self.m);

        // 接続する人の家に駅を建てる
        self.build_station(self.home[pi])?;
        self.build_station(self.workspace[pi])?;

        // 接続する人の家と駅を結ぶ
        let pos0 = self.home[pi];
        let pos1 = self.workspace[pi];

        // pos0 -> pos1 への垂直のレールを建てる
        {
            if pos0.0 < pos1.0 {
                // 垂直のレールを建てる
                for r in pos0.0 + 1..pos1.0 {
                    self.build_rail(&Building::VerticalRail, Pos(r, pos0.1))?;
                }

                // ゴールへ向けて曲がったレールを建てる
                if pos0.1 < pos1.1 {
                    self.build_rail(&Building::RightUpRail, Pos(pos1.0, pos0.1))?;
                } else if pos0.1 > pos1.1 {
                    self.build_rail(&Building::LeftUpRail, Pos(pos1.0, pos0.1))?;
                }
            } else if pos0.0 > pos1.0 {
                // 垂直のレールを建てる
                for r in (pos1.0 + 1)..pos0.0 {
                    self.build_rail(&Building::VerticalRail, Pos(r, pos0.1))?;
                }

                // ゴールへ向けて曲がったレールを建てる
                if pos0.1 < pos1.1 {
                    self.build_rail(&Building::RightDownRail, Pos(pos1.0, pos0.1))?;
                } else if pos0.1 > pos1.1 {
                    self.build_rail(&Building::LeftDownRail, Pos(pos1.0, pos0.1))?;
                }
            }
        }

        // pos0 -> pos1 への水平のレールを建てる
        {
            // 水平のレールを建てる
            if pos0.1 < pos1.1 {
                for c in pos0.1 + 1..pos1.1 {
                    self.build_rail(&Building::HorizontalRail, Pos(pos1.0, c))?;
                }
            }
            // 水平のレールを建てる
            else if pos0.1 > pos1.1 {
                for c in (pos1.1 + 1)..pos0.1 {
                    self.build_rail(&Building::HorizontalRail, Pos(pos1.0, c))?;
                }
            }
        }
        self.update_is_connected();

        Ok(())
    }

    fn solve(&mut self) -> Answer {
        let mut income = 0;
        while self.actions.len() < self.t {
            let start_idx = self.actions.len();
            if self.actions.len() == 0 {
                let result_process = self.prosess_one();
                if result_process.is_err() {
                    self.replace_do_nothing(start_idx);
                }
                income = self.calc_income();
            } else {
                self.build_nothing();
            }
            self.money += income;
        }

        debug_assert!(self.actions.len() <= self.t);
        // もしターン数を超えてしまっていたら取り除く
        while self.actions.len() > self.t {
            self.actions.pop();
        }

        Answer::new(&self.actions, self.money)
    }
}

#[fastout]
fn main() {
    input! {
        // N は区画の縦・横の数で、 N=50 を満たす。
        n: usize,
        // M はR国の人の数で、 50≤M≤1600 を満たす。
        m: usize,
        // K は鉄道会社Xの初期資金であり、 11000≤K≤20000 を満たす。
        k: usize,
        //T はゲームのターン数であり、 T=800 を満たす。
        t: usize,
    }

    let mut solver = Solver::new(n, m, k, t);

    {
        for _ in 0..m {
            input! {
                i: usize,
                j: usize,
            }
            let home_i = Pos(i, j);
            input! {
                i: usize,
                j: usize,
            }
            let workspace_i = Pos(i, j);

            solver.home.push(home_i);
            solver.workspace.push(workspace_i);
            solver.distance.push(calc_distance(home_i, workspace_i));
            solver.is_connected.push(false);
        }
    }

    let ans = solver.solve();
    println!("{}", ans.to_string());
    eprintln!("score={}", ans.score);
}
