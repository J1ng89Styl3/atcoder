#[warn(unused_imports)]
use std::iter::FromIterator;
use std::usize;
use ascii::Chars;
use bitvec::vec;
use im_rc::hashmap::Values;
use im_rc::{hashmap, HashSet};
use indexing::container_traits::FixedLength;
use num::abs;
use petgraph::visit::GraphProp;
use proconio::input;
use proconio::{marker::*, *};
use ac_library::*;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::cmp;
use ac_library::Dsu;
use ac_library::SccGraph;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use superslice::*;
use itertools::{Itertools, Update};
use num::Integer;
use std::process::exit;
use ac_library::ModInt998244353 as Mint;
use std::collections::BTreeMap;
use ac_library::Segtree;
use fixedbitset::FixedBitSet;
use bitvec::prelude::*;
use std::collections::BTreeSet;
use itertools::iproduct;
use ac_library::modint::ModInt;
use std::io::{self, Read};

const INF: f64 = f64::INFINITY;
const NEG_INF: f64 = -f64::INFINITY;
fn step1_main(s: &str) {
    let mut lines = s.lines();
    let m: usize = lines.next().unwrap().parse().unwrap();
    let mut menu_price: HashMap<usize, usize> = HashMap::new();
    let mut menu_stock: HashMap<usize, usize> = HashMap::new();

    for _ in 0..m {
        let v: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        menu_price.insert(v[0], v[2]); // 料理番号, 価格
        menu_stock.insert(v[0], v[1]); // 料理番号, 在庫数
    }
    let mut order_info: Vec<(usize, usize, usize)> = vec![];
    for line in lines {
        let mut it = line.split_whitespace();

        let _cmd: &str = it.next().unwrap();
        let t: usize = it.next().unwrap().parse().unwrap();
        let d: usize = it.next().unwrap().parse().unwrap();
        let n: usize = it.next().unwrap().parse().unwrap();

        order_info.push((t, d, n));
    }
    let ans = step1_solve(&mut menu_stock, &order_info);
    for a in ans {
        println!("{}", a);
    }
}

fn step1_solve(
    menu_stock: &mut HashMap<usize, usize>,
    order_info: &Vec<(usize, usize, usize)>,
) -> Vec<String> {
    let mut ans :Vec<String> = vec![];
    for &(t, d, n) in order_info {
        let stock = *menu_stock.get(&d).unwrap();
        if stock >= n {
            for _ in 0..n {
                ans.push(format!("received order {} {}", t, d));
            }
            menu_stock.insert(d, stock - n);
        } else {
            ans.push(format!("sold out {}", t));
        }
    }
    return ans
}

fn step2_main(s: &str) {
    let mut lines = s.lines();
    let v: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let m: usize = v[0];
    let k: usize = v[1];

    // メニュー情報はここでは使わないので読み飛ばす
    for _ in 0..m {
        lines.next();
    }
    let mut event_info: Vec<(String, usize)> = vec![];
    for line in lines {
        let mut it = line.split_whitespace();
        let cmd = it.next().unwrap();
        // 注文受注
        if cmd == "received" {
            let _order = it.next().unwrap();
            let _t: usize = it.next().unwrap().parse().unwrap();
            let d: usize = it.next().unwrap().parse().unwrap();
            event_info.push((cmd.to_string(), d));
        } 
        // 料理完成
        else if cmd == "complete" {
            let d: usize = it.next().unwrap().parse().unwrap();
            event_info.push((cmd.to_string(), d));
        }
    }
    let ans = step2_solve(k, &event_info);
    for a in ans {
        println!("{}", a);
    }
}

fn step2_solve(k: usize, event_info: &Vec<(String, usize)>) -> Vec<String> {
    let mut ans = vec![];
    let mut cooking: HashMap<usize, usize> = HashMap::new();
    let mut cooking_count = 0usize;
    let mut waiting: VecDeque<usize> = VecDeque::new();
    for (cmd, d) in event_info {
        if cmd == "received" {
            if cooking_count < k {
                cooking_count += 1;
                *cooking.entry(*d).or_insert(0) += 1;
                ans.push(format!("{}", d));
            } else {
                waiting.push_back(*d);
                ans.push("wait".to_string());
            }
        } else if cmd == "complete" {
            let cnt = cooking.get(d).copied().unwrap_or(0);
            if cnt == 0 {
                ans.push("unexpected input".to_string());
            } else {
                if cnt == 1 {
                    cooking.remove(d);
                } else {
                    cooking.insert(*d, cnt - 1);
                }
                cooking_count -= 1;
                if let Some(next_d) = waiting.pop_front() {
                    cooking_count += 1;
                    *cooking.entry(next_d).or_insert(0) += 1;
                    ans.push(format!("ok {}", next_d));
                } else {
                    ans.push("ok".to_string());
                }
            }
        }
    }
    return ans
}


fn step3_main(s: &str) {
    let mut lines = s.lines();
    let v: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let m: usize = v[0];
    // let k: usize = v[1];

    // メニュー情報はここでは使わないので読み飛ばす
    for _ in 0..m {
        lines.next();
    }
    let mut events: Vec<(String, usize, usize)> = vec![];
    // let mut event_info: Vec<(String, usize)> = vec![];
    for line in lines {
        let mut it = line.split_whitespace();
        let cmd = it.next().unwrap();
        // 注文受注
        if cmd == "received" {
            let _order = it.next().unwrap();
            let t: usize = it.next().unwrap().parse().unwrap();
            let d: usize = it.next().unwrap().parse().unwrap();
            // event_info.push((cmd.to_string(), d))    ;
            events.push((cmd.to_string(), t, d));
        } 
        // 料理完成
        else if cmd == "complete" {
            let d: usize = it.next().unwrap().parse().unwrap();
            // event_info.push((cmd.to_string(), d));
            events.push((cmd.to_string(), 0, d));
        }
    }
    let ans = step3_solve(&events);
    for a in ans {
        println!("{}", a);
    }
}

fn step3_solve(events: &Vec<(String, usize, usize)>) -> Vec<String> {
    // 注文管理はキューで行う
    let mut ans = vec![];
    let mut order_queue: HashMap<usize, VecDeque<usize>> = HashMap::new();
    for (cmd, t, d) in events {
        if cmd == "received" {
            order_queue
                .entry(*d)
                .or_insert(VecDeque::new())
                .push_back(*t);
        } else if cmd == "complete" {
            let t = order_queue.get_mut(d).unwrap().pop_front().unwrap();
            ans.push(format!("ready {} {}", t, d));
        }
    }
    return ans
}

fn step4_main(s: &str) {
    let mut lines = s.lines();
    let m: usize = lines.next().unwrap().parse().unwrap();
    let mut menu_price: HashMap<usize, usize> = HashMap::new();
    for _ in 0..m {
        let v: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let d = v[0];
        let p = v[2];
        menu_price.insert(d, p);
    }
    let mut event_info: Vec<(String, usize, usize)> = vec![];

    for line in lines {
        let mut it = line.split_whitespace();
        let cmd = it.next().unwrap();
        if cmd == "received" {
            let _order = it.next().unwrap();
            let t: usize = it.next().unwrap().parse().unwrap();
            let d: usize = it.next().unwrap().parse().unwrap();
            event_info.push((cmd.to_string(), t, d));
        } else if cmd == "ready" {
            let t: usize = it.next().unwrap().parse().unwrap();
            let d: usize = it.next().unwrap().parse().unwrap();
            event_info.push((cmd.to_string(), t, d));
        } else if cmd == "check" {
            let t: usize = it.next().unwrap().parse().unwrap();
            event_info.push((cmd.to_string(), t, 0));
        }
    }
    let ans = step4_solve(&menu_price, &event_info);
    for a in ans {
        println!("{}", a);
    }
}
fn step4_solve(menu_price: &HashMap<usize, usize>, event_info: &Vec<(String, usize, usize)>) -> Vec<String> {
    let mut ans :Vec<String> = vec![];
    let mut total_price: HashMap<usize, usize> = HashMap::new();
    let mut unserved_count: HashMap<usize, usize> = HashMap::new();
    for (cmd, t, d) in event_info {
        if cmd == "received" {
            let price = *menu_price.get(d).unwrap();
            *total_price.entry(*t).or_insert(0) += price;
            *unserved_count.entry(*t).or_insert(0) += 1;
        } else if cmd == "ready" {
            let count = unserved_count.get_mut(t).unwrap();
            *count -= 1;
        } else if cmd == "check" {
            let count = *unserved_count.get(t).unwrap_or(&0);
            if count == 0 {
                let price = *total_price.get(t).unwrap_or(&0);
                ans.push(format!("{}", price));

                total_price.insert(*t, 0);
                unserved_count.insert(*t, 0);
            } else {
                ans.push("please wait".to_string());
            }
        }
    }
    return ans;
}



fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut lines = s.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let rest = lines.collect::<Vec<_>>().join("\n");

    if n == 1 {
        step1_main(&rest);
    } else if n == 2 {
        step2_main(&rest);
    } else if n == 3 {
        step3_main(&rest);
    } else if n == 4 {
        step4_main(&rest);
    }
}
