use itertools::*;
use proconio::{fastout, input};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const COST_STATION: i64 = 5000;
const COST_RAIL: i64 = 100;

#[derive(Clone, Copy, Debug)]
struct Pos(usize, usize);

#[derive(Clone)]
struct UnionFind {
    n: usize,
    /// 親頂点の番号
    parents: Vec<usize>,
    /// 連結成分の頂点の個数
    sizes: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            n,
            parents: vec![usize::MAX; n * n],
            sizes: vec![1; n * n],
        }
    }

    fn pos_to_index(&self, pos: Pos) -> usize {
        pos.0 * self.n + pos.1
    }

    fn find_root(&mut self, idx: usize) -> usize {
        if self.parents[idx] == usize::MAX {
            return idx;
        }
        let root = self.find_root(self.parents[idx]);
        self.parents[idx] = root;
        root
    }

    fn is_same(&mut self, p: Pos, q: Pos) -> bool {
        let p_idx = self.pos_to_index(p);
        let q_idx = self.pos_to_index(q);
        self.find_root(p_idx) == self.find_root(q_idx)
    }

    fn unite(&mut self, p: Pos, q: Pos) {
        let p_idx = self.pos_to_index(p);
        let q_idx = self.pos_to_index(q);
        let p_root = self.find_root(p_idx);
        let q_root = self.find_root(q_idx);

        if p_root != q_root {
            if self.sizes[p_root] < self.sizes[q_root] {
                self.parents[p_root] = q_root;
                self.sizes[q_root] += self.sizes[p_root];
            } else {
                self.parents[q_root] = p_root;
                self.sizes[p_root] += self.sizes[q_root];
            }
        }
    }
}

fn calc_distance(pos1: &Pos, pos2: &Pos) -> i64 {
    let pos1 = (pos1.0 as i64, pos1.1 as i64);
    let pos2 = (pos2.0 as i64, pos2.1 as i64);

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

#[derive(Clone)]
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

#[derive(Clone)]
struct Answer<'a> {
    actions: &'a Vec<Action>,
    score: i64,
}

impl<'a> Answer<'a> {
    fn new(actions: &'a Vec<Action>, score: i64) -> Self {
        Self { actions, score }
    }
}

#[derive(Clone)]
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
        debug_assert!(calc_distance(&s, &t) > 4);

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
        itertools::iproduct!(-2_i64..3, -2_i64..3)
            .filter(|&(dr, dc)| dr.abs() + dc.abs() <= 2)
            .map(|(dr, dc)| ((*rr as i64 + dr), (*cc as i64 + dc)))
            .filter(|&(r, c)| {
                r >= 0
                    && r < self.n as i64
                    && c >= 0
                    && c < self.n as i64
                    && self.fields[r as usize][c as usize].is_some()
                    && self.fields[r as usize][c as usize]
                        .as_ref()
                        .unwrap()
                        .is_station()
            })
            .map(|(r, c)| Pos(r as usize, c as usize))
            .collect()
    }

    fn is_connected_rail(&self, s: Pos, t: Pos) -> bool {
        if s.0 != t.0 && s.1 != t.1 {
            panic!("s.0 != t.0 && s.1 != t.1");
        }

        if s.0 == t.0 && s.1 == t.1 {
            panic!("s.0 == t.0 && s.1 == t.1");
        }

        if s.0 == t.0 {
            if s.1 < t.1 {
                if let Some(ref rail_s) = self.fields[s.0][s.1] {
                    if let Some(ref rail_t) = self.fields[t.0][t.1] {
                        if rail_s.is_right() && rail_t.is_left() {
                            return true;
                        }
                    }
                }
            } else {
                if let Some(ref rail_s) = self.fields[s.0][s.1] {
                    if let Some(ref rail_t) = self.fields[t.0][t.1] {
                        if rail_s.is_left() && rail_t.is_right() {
                            return true;
                        }
                    }
                }
            }
        } else if s.1 == t.1 {
            if s.0 < t.0 {
                if let Some(ref rail_s) = self.fields[s.0][s.1] {
                    if let Some(ref rail_t) = self.fields[t.0][t.1] {
                        if rail_s.is_down() && rail_t.is_up() {
                            return true;
                        }
                    }
                }
            } else {
                if let Some(ref rail_s) = self.fields[s.0][s.1] {
                    if let Some(ref rail_t) = self.fields[t.0][t.1] {
                        if rail_s.is_up() && rail_t.is_down() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

struct SolverInput {
    n: usize,
    m: usize,
    k: usize,
    t: usize,
    home: Vec<Pos>,
    workspace: Vec<Pos>,
    distance: Vec<i64>,
}

impl SolverInput {
    fn new() -> Self {
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

        let mut home = Vec::new();
        let mut workspace = Vec::new();
        let mut distance = Vec::new();

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

            home.push(home_i);
            workspace.push(workspace_i);
            distance.push(calc_distance(&home_i, &workspace_i));
        }

        Self {
            n,
            m,
            k,
            t,
            home,
            workspace,
            distance,
        }
    }
}

struct SolverState {
    is_connected: Vec<bool>,
    field: Field,
    money: i64,
    actions: Vec<Action>,
    income: i64,
    // 建設できなかった駅
    station_queue: VecDeque<Pos>,
}

impl SolverState {
    fn new(input: &SolverInput) -> Self {
        let mut is_connected = Vec::with_capacity(input.m);
        is_connected.resize(input.m, false);

        let field = Field::new(input.n);

        let actions = Vec::with_capacity(input.t);

        Self {
            is_connected,
            field,
            money: input.k as i64,
            actions,
            income: 0,
            station_queue: VecDeque::new(),
        }
    }
}
enum SolverError {
    NotEnoughMoney(i64),
    TooManyActions,
}

struct Solver<'a> {
    input: &'a SolverInput,
    state: SolverState,
    best_actions: Vec<Action>,
}

impl<'a> Solver<'a> {
    fn new(input: &'a SolverInput) -> Self {
        Self {
            input,
            state: SolverState::new(input),
            best_actions: vec![Action::DoNothing; input.t],
        }
    }

    fn update_is_connected(&mut self) {
        for i in 0..self.input.m {
            if !self.state.is_connected[i]
                && self
                    .state
                    .field
                    .is_connected(self.input.home[i], self.input.workspace[i])
            {
                self.state.is_connected[i] = true;
            }
        }
    }

    fn calc_income(&mut self) -> i64 {
        (0..self.input.m)
            .filter(|&i| self.state.is_connected[i])
            .map(|i| self.input.distance[i])
            .sum()
    }

    fn build_rail(&mut self, building: &Building, Pos(r, c): Pos) -> Result<(), SolverError> {
        if self.state.money < COST_RAIL {
            return Err(SolverError::NotEnoughMoney(COST_RAIL));
        }
        if self.state.actions.len() >= self.input.t {
            return Err(SolverError::TooManyActions);
        }

        // すでにレールが建てられている場合はスキップ
        if let Some(_) = self.state.field.fields[r][c].as_ref() {
            return Ok(());
        }

        self.state.field.build(building, Pos(r, c));
        self.state.money -= COST_RAIL;
        self.state
            .actions
            .push(Action::Build((building.clone(), Pos(r, c))));

        if r > 0 {
            if let Some(exisiting) = self.state.field.fields[r - 1][c].as_ref() {
                if exisiting.is_station() && building.is_up() {
                    self.update_is_connected();
                    self.state.income = self.calc_income();
                }
            }
        }
        if r < self.input.n - 1 {
            if let Some(exisiting) = self.state.field.fields[r + 1][c].as_ref() {
                if exisiting.is_station() && building.is_down() {
                    self.update_is_connected();
                    self.state.income = self.calc_income();
                }
            }
        }
        if c > 0 {
            if let Some(exisiting) = self.state.field.fields[r][c - 1].as_ref() {
                if exisiting.is_station() && building.is_left() {
                    self.update_is_connected();
                    self.state.income = self.calc_income();
                }
            }
        }
        if c < self.input.n - 1 {
            if let Some(exisiting) = self.state.field.fields[r][c + 1].as_ref() {
                if exisiting.is_station() && building.is_right() {
                    self.update_is_connected();
                    self.state.income = self.calc_income();
                }
            }
        }

        self.collect_money();
        Ok(())
    }

    fn build_station(&mut self, Pos(r, c): Pos) -> Result<(), SolverError> {
        if self.state.money < COST_STATION {
            self.state.station_queue.push_back(Pos(r, c));
            return Err(SolverError::NotEnoughMoney(COST_STATION));
        }
        if self.state.actions.len() >= self.input.t {
            return Err(SolverError::TooManyActions);
        }

        // すでに駅が建てられている場合はスキップ
        if let Some(existing) = self.state.field.fields[r][c].as_ref() {
            if existing.is_station() {
                return Ok(());
            }
        }

        self.state.field.build(&Building::Station, Pos(r, c));
        self.state.money -= COST_STATION;

        self.state
            .actions
            .push(Action::Build((Building::Station, Pos(r, c))));

        self.update_is_connected();
        self.state.income = self.calc_income();

        self.collect_money();
        Ok(())
    }

    fn buildnothing(&mut self) -> Result<(), SolverError> {
        if self.state.actions.len() >= self.input.t {
            return Err(SolverError::TooManyActions);
        }
        self.state.actions.push(Action::DoNothing);
        self.collect_money();
        Ok(())
    }

    fn collect_money(&mut self) {
        self.state.money += self.state.income;
    }

    fn _replace_donothing(&mut self, start_idx: usize) {
        for i in start_idx..self.state.actions.len() {
            self.state.actions[i] = Action::DoNothing;
        }
    }

    fn connect_points(&mut self, pos0: Pos, pos1: Pos) -> Result<(), SolverError> {
        let mut pre: Option<Pos> = None;

        if (calc_distance(&pos0, &pos1) as usize) + self.state.actions.len() >= self.input.t {
            return Err(SolverError::TooManyActions);
        }

        // pos0 -> pos1 への垂直のレールを建てる
        {
            if pos0.0 < pos1.0 {
                // 垂直のレールを建てる
                for r in pos0.0 + 1..pos1.0 {
                    let pos = Pos(r, pos0.1);
                    self.build_rail(&Building::VerticalRail, pos)?;

                    if let Some(pre) = pre {
                        if !self.state.field.is_connected_rail(pre, pos) {
                            self.build_station(pre)?;
                        }
                    }
                    pre = Some(pos);
                }

                // ゴールへ向けて曲がったレールを建てる
                {
                    let pos = Pos(pos1.0, pos0.1);
                    if pos0.1 < pos1.1 {
                        self.build_rail(&Building::RightUpRail, pos)?;
                    } else if pos0.1 > pos1.1 {
                        self.build_rail(&Building::LeftUpRail, pos)?;
                    }
                    if let Some(pre) = pre {
                        if !self.state.field.is_connected_rail(pre, pos) {
                            self.build_station(pre)?;
                        }
                    }
                    pre = Some(pos);
                }
            } else if pos0.0 > pos1.0 {
                // 垂直のレールを建てる
                for r in ((pos1.0 + 1)..pos0.0).rev() {
                    let pos = Pos(r, pos0.1);
                    self.build_rail(&Building::VerticalRail, pos)?;
                    if let Some(pre) = pre {
                        if !self.state.field.is_connected_rail(pre, pos) {
                            self.build_station(pre)?;
                        }
                    }
                    pre = Some(pos);
                }

                // ゴールへ向けて曲がったレールを建てる
                {
                    let pos = Pos(pos1.0, pos0.1);
                    if pos0.1 < pos1.1 {
                        self.build_rail(&Building::RightDownRail, pos)?;
                    } else if pos0.1 > pos1.1 {
                        self.build_rail(&Building::LeftDownRail, pos)?;
                    }
                    if let Some(pre) = pre {
                        if !self.state.field.is_connected_rail(pre, pos) {
                            self.build_station(pre)?;
                        }
                    }
                    pre = Some(pos);
                }
            }
        }

        // pos0 -> pos1 への水平のレールを建てる
        {
            // 水平のレールを建てる
            if pos0.1 < pos1.1 {
                for c in pos0.1 + 1..pos1.1 {
                    let pos = Pos(pos1.0, c);
                    self.build_rail(&Building::HorizontalRail, pos)?;
                    if let Some(pre) = pre {
                        if !self.state.field.is_connected_rail(pre, pos) {
                            self.build_station(pre)?;
                        }
                    }
                    pre = Some(pos);
                }
            }
            // 水平のレールを建てる
            else if pos0.1 > pos1.1 {
                for c in ((pos1.1 + 1)..pos0.1).rev() {
                    let pos = Pos(pos1.0, c);
                    self.build_rail(&Building::HorizontalRail, pos)?;
                    if let Some(pre) = pre {
                        if !self.state.field.is_connected_rail(pre, pos) {
                            self.build_station(pre)?;
                        }
                    }
                    pre = Some(pos);
                }
            }
        }

        // 駅を建てる
        self.build_station(pos0)?;
        self.build_station(pos1)?;

        Ok(())
    }

    fn connect_home_and_workspace(&mut self, pi: usize) -> Result<(), SolverError> {
        // 接続する人の家と駅を結ぶ
        let pos0 = self.input.home[pi];
        let pos1 = self.input.workspace[pi];

        self.connect_points(pos0, pos1)?;

        Ok(())
    }

    fn select_pi(&mut self) -> Result<usize, SolverError> {
        // 作れるレールの数
        let rail_count = (self.input.k as i64) - COST_STATION * 2;
        let mut husoku_rail_count = (self.input.n * self.input.n) as i64;

        let mut pi = 0;

        while pi < self.input.m {
            if self.state.is_connected[pi] {
                pi += 1;
                continue;
            }

            if self.input.distance[pi] - 1 <= rail_count {
                break;
            } else {
                husoku_rail_count = husoku_rail_count.min(self.input.distance[pi] - 1 - rail_count);
            }
            pi += 1;
        }

        if pi == self.input.m {
            debug_assert!(husoku_rail_count > 0);
            return Err(SolverError::NotEnoughMoney(husoku_rail_count * COST_RAIL));
        }

        Ok(pi)
    }

    fn execute(&mut self, time_limit: &Duration, start_time: &Instant, pi: usize) -> Answer {
        while self.state.actions.len() == 0 && self.state.actions.len() < self.input.t {
            if start_time.elapsed() >= *time_limit {
                break;
            }

            while self.state.station_queue.len() > 0 && self.state.money >= COST_STATION {
                let pos = self.state.station_queue.pop_front().unwrap();
                if let Err(SolverError::TooManyActions) = self.build_station(pos) {
                    break;
                }
            }

            if cfg!(feature = "debug") {
                println!(
                    "# actions.len={}, self.money={}, self.income={}",
                    self.state.actions.len(),
                    self.state.money,
                    self.state.income
                );
            }

            let result_connect = match self.select_pi() {
                Ok(_pi) => self.connect_home_and_workspace(pi),
                Err(err) => Err(err),
            };

            match result_connect {
                Ok(_) => {}
                Err(SolverError::NotEnoughMoney(cost)) => {
                    if cost == COST_RAIL {
                        while self.state.actions.len() < self.input.t
                            && self.state.money < COST_RAIL
                        {
                            if let Err(SolverError::TooManyActions) = self.buildnothing() {
                                break;
                            }
                        }
                    }
                }
                Err(SolverError::TooManyActions) => {
                    eprintln!("# TooManyActions");
                    break;
                }
            }
        }

        debug_assert!(self.state.actions.len() <= self.input.t);
        // もしターン数を超えてしまっていたら取り除く
        while self.state.actions.len() > self.input.t {
            self.state.actions.pop();
        }

        // もしターン数を超えていない場合はDoNothingを追加
        while self.state.actions.len() < self.input.t {
            if let Err(SolverError::TooManyActions) = self.buildnothing() {
                break;
            }
        }

        Answer::new(&self.state.actions, self.state.money)
    }

    fn print_best_actions(&self) {
        println!(
            "{}",
            self.best_actions.iter().map(|a| a.to_string()).join("\n")
        );
    }

    fn solve(&mut self, time_limit: &Duration, start_time: &Instant) {
        let mut best_score = self.input.k as i64;
        for i in 0..self.input.m {
            let Answer { actions, score } = self.execute(&time_limit, &start_time, i);
            if score > best_score {
                best_score = score;
                self.best_actions = actions.clone();
            }
            self.state = SolverState::new(self.input);
        }

        self.print_best_actions();
        eprintln!("#score: {}", best_score);
    }
}

#[fastout]
fn main() {
    let time_limit = Duration::from_millis(1900);
    let start_time = Instant::now();

    let input = SolverInput::new();

    let mut solver = Solver::new(&input);
    solver.solve(&time_limit, &start_time);
}
