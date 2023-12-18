use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut map: HashMap<String, (bool, bool)> = HashMap::new();
    for i in 0..n {
        if s[i].starts_with("!") {
            let key = (&s[i][1..]).to_string();
            if let Some(val) = map.get_mut(&key) {
                (*val).1 = true;
            } else {
                map.insert(key, (false, true));
            }
        } else {
            if let Some(val) = map.get_mut(&s[i]) {
                (*val).0 = true;
            } else {
                map.insert(s[i].clone(), (true, false));
            }
        }
    }
    for (k, v) in map.iter() {
        if v.0 && v.1 {
            println!("{}", k);
            return;
        }
    }
    println!("satisfiable");
}
