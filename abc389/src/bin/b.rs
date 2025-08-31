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
    }
}
