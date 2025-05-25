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

fn main() {
    input! {
        s: String
    }
    let n = s.len() as i64;

    let digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
    let s_start = digits[0] as i64;

    let mut b_before: i64 = 0;
    for &i in digits.iter().skip(1) {
        let desired = (s_start - i as i64).rem_euclid(10);
        let delta = (desired -  b_before % 10 + 10) % 10;
        b_before += delta;
    }
    let m = (b_before - s_start + 9) / 10;
    let k = s_start + 10 * m;

    let ans = n + k;
    println!("{}", ans);
}