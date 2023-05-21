// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
use std::collections::HashMap;

fn is_atcoder(ch: &char) -> bool {
    *ch == 'a' ||
    *ch == 't' ||
    *ch == 'c' ||
    *ch == 'o' ||
    *ch == 'd' ||
    *ch == 'e' ||
    *ch == 'r'
}

fn is_ok(ch: &char, num: i32, map: &mut HashMap<&char, i32>) -> bool {
    if is_atcoder(ch) == false {
        return false;
    }
    if let Some(x) = map.get_mut(&'@') {
        if *x >= num {
            *x -= num;
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();
    let mut keys = HashSet::new();

    for ch in s.iter() {
        if *ch != '@' {
            keys.insert(*ch);
        }
        *map_s.entry(ch).or_insert(0) += 1;
    }
    for ch in t.iter() {
        if *ch != '@' {
            keys.insert(*ch);
        }
        *map_t.entry(ch).or_insert(0) += 1;
    }

    for key in keys.iter() {
        if map_s.contains_key(key) && map_t.contains_key(key) {
            let num_s = map_s.get(key).unwrap();
            let num_t = map_t.get(key).unwrap();
            if num_s < num_t {
                if is_ok(key, *num_t - *num_s, &mut map_s) == false {
                    println!("No");
                    return;
                }
            } else if num_s > num_t {
                if is_ok(key, *num_s - *num_t, &mut map_t) == false {
                    println!("No");
                    return;
                }
            }
        } else if map_s.contains_key(key) {
            let num = map_s.get(key).unwrap();
            if is_ok(key, *num, &mut map_t) == false {
                println!("No");
                return;
            }
        } else if map_t.contains_key(key) {
            let num = map_t.get(key).unwrap();
            if is_ok(key, *num, &mut map_s) == false {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

