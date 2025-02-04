#[warn(unused_imports)]
use std::iter::FromIterator;
use std::usize;
use ascii::Chars;
use bitvec::vec;
use im_rc::hashmap::Values;
use im_rc::{hashmap, HashSet};
use indexing::container_traits::FixedLength;
use num::abs;
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

const INF: f64 = f64::INFINITY;
const NEG_INF: f64 = -f64::INFINITY;


// セグメントツリー関連のモノイド
struct Xor; //xor
impl ac_library::Monoid for Xor {
    type S = i64;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
    }
}

struct Min; //最小値
impl ac_library::Monoid for Min {
    type S = i64;

    fn identity() -> Self::S {
        i64::MAX
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.min(b)
    }
}

struct Max; //最大値
impl ac_library::Monoid for Max {
    type S = i64;

    fn identity() -> Self::S {
        i64::MIN
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
}

struct Sum; //和
impl ac_library::Monoid for Sum {
    type S = i64;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a + b
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        pattern: [String; n],
        queries: [(usize, usize, usize, usize); q],
    }
    let mut pre = vec![vec![0usize; n + 1]; n + 1];
    for i in 0..n {
        // pattern[i] の各文字を走査
        let row: Vec<char> = pattern[i].chars().collect();
        for j in 0..n {
            // P[i][j] が 'B' ならば 1, 'W' ならば 0 として加算
            let add = if row[j] == 'B' { 1 } else { 0 };
            pre[i + 1][j + 1] = pre[i + 1][j] + pre[i][j + 1] - pre[i][j] + add;
        }
    }
    // 1ブロック（パターン全体）の黒マス数
    let total_block = pre[n][n];
    let f = |x: i64, y: i64| -> i64 {
        // x または y が負の場合は領域が存在しないので 0
        if x < 0 || y < 0 {
            return 0;
        }
        let X = x + 1; // 行数
        let Y = y + 1; // 列数
        let n_i64 = n as i64;
        let qx = X / n_i64;
        let rx = (X % n_i64) as usize; // 余り部分の行数（0 <= rx < n）
        let qy = Y / n_i64;
        let ry = (Y % n_i64) as usize; // 余り部分の列数
        qx * qy * (total_block as i64)
            + qx * (pre[n][ry] as i64)
            + qy * (pre[rx][n] as i64)
            + (pre[rx][ry] as i64)
    };

    for (A, B, C, D) in queries {
        let ans = f(C as i64, D as i64)
            - f(A as i64 - 1, D as i64)
            - f(C as i64, B as i64 - 1)
            + f(A as i64 - 1, B as i64 - 1);
        println!("{}", ans);
    }
}
