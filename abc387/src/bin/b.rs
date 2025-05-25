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

const INF: f64 = f64::INFINITY;
const NEG_INF: f64 = -f64::INFINITY;

struct Fenwick {
    n: usize,
    bit: Vec<usize>,
}
impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { n, bit: vec![0; n + 1] }
    }

    fn add(&mut self, idx: usize, v: usize) {
        let mut i = idx + 1;
        while i <= self.n {
            self.bit[i] += v;
            i += i & i.wrapping_neg();
        }
    }

    fn sum(&self, mut idx: usize) -> usize {
        let mut s = 0;
        while idx > 0 {
            s += self.bit[idx];
            idx -= idx & idx.wrapping_neg();
        }
        s
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let n1 = a + b + c;
    let n2 = c + d;
    let m = cmp::max(n1, n2);

    let mut fact = vec![ModInt::new(1); m + 1];
    for i in 1..=m {
        fact[i] = fact[i - 1] * ModInt::new(i);
    }

    let mut invfact = vec![ModInt::new(1); m + 1];
    invfact[m] = fact[m].inv();
    for i in (1..=m).rev() {
        invfact[i - 1] = invfact[i] * ModInt::new(i);
    }

    let mut ans = ModInt::new(0);

    for u in 0..=c {
        let n = n1 - u;
        let t1 = fact[n] * invfact[b] * invfact[n - b];
        let t1_next = if u < c {
            fact[n - 1] * invfact[b] * invfact[n - 1 - b]
        } else {
            ModInt::new(0)
        };
        let f = t1 - t1_next;

        let comb = fact[d + u] * invfact[u] * invfact[d];

        ans += f * comb;
    }

    println!("{}", ans.val());
}