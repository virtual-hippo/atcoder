use proconio::input;

fn is_center(p1: i64, p2: i64) -> bool {
    p1 == p2 * 2
}

fn on_taikakusen(p1: (i64, i64), p2: (i64, i64)) -> bool {
    if p2.0 * p1.1 == p2.1 * p1.0 {
        return true;
    }

    // y = -(h/w)x + h
    // wy - wh = -(h)x
    let ((w, h), (x, y)) = (p1, p2);
    if w * y - w * h == -h * x {
        return true;
    }
    false
}

fn get_len_divided_by_suisen(p1: i64, p2: i64) -> i64 {
    if 2 * p1 < p2 {
        p1
    } else {
        p2 - p1
    }
}

fn on_shu(p1: (i64, i64), p2: (i64, i64)) -> bool {
    let ((w, h), (x, y)) = (p1, p2);
    if x == 0 || y == 0 || x == w || y == h {
        return true;
    }
    false
}

fn main() {
    input! {
        (w,h): (i64, i64),
        (x,y): (i64, i64),
    }
    let is_x_center_point = is_center(w, x);
    let is_y_center_point = is_center(h, y);
    let half_menseki = (w * h) as f64 / 2.0;

    if is_x_center_point && is_y_center_point {
        println!("{} 1", half_menseki);
    } else {
        println!("{} 0", half_menseki);
    }
}
