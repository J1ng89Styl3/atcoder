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

const INF: f64 = f64::INFINITY;
const NEG_INF: f64 = -f64::INFINITY;


struct Fenwick {
    n: usize,
    t: Vec<i32>,
    max_pow: usize,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        let mut bit = 1;
        while (bit << 1) <= n { bit <<= 1; }
        Fenwick { n, t: vec![0; n + 1], max_pow: bit }
    }

    fn update(&mut self, mut i: usize, v: i32) {
        while i <= self.n {
            self.t[i] += v;
            i += i & (!i + 1);
        }
    }

    fn kth(&self, mut k: i32) -> usize {
        let mut pos = 0;
        let mut bit = self.max_pow;
        while bit > 0 {
            let nxt = pos + bit;
            if nxt <= self.n && self.t[nxt] < k {
                k -= self.t[nxt];
                pos = nxt;
            }
            bit >>= 1;
        }
        pos + 1
    }
}

// 全探索でいける
// 使わない、右と配置、下と配置でやっていく

fn dfs(mask: usize, val: u64, h: usize, w: usize, a: &Vec<Vec<u64>>, ans: &mut u64) {
    let n = h * w;
    if mask == (1 << n) - 1 {
        *ans = (*ans).max(val);
        return;
    }
    let mut i = 0;
    while i < n && (mask & (1 << i)) != 0 {
        i += 1;
    }
    let x = i / w;
    let y = i % w;
    dfs(mask | (1 << i), val ^ a[x][y], h, w, a, ans);
    if y + 1 < w {
        let j = i + 1;
        if (mask & (1 << j)) == 0 {
            dfs(mask | (1 << i) | (1 << j), val, h, w, a, ans);
        }
    }
    if x + 1 < h {
        let j = i + w;
        if (mask & (1 << j)) == 0 {
            dfs(mask | (1 << i) | (1 << j), val, h, w, a, ans);
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
    }
    let mut ans: u64 = 0;
    dfs(0, 0, h, w, &a, &mut ans);
    println!("{}", ans);
}
