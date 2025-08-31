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

// フェニック木
/*
""""""
使い方（0-indexed）
- Fenwick::new(n)
- add(i, v): 点iに+v（加算）
- sum(r): [1..r) ではなく、内部はBITなので呼び出しは prefix 用に sum(idx) を使う（この実装では idx は1-origin想定で呼ぶ）
  例: 区間[0, r) の和が欲しいとき -> sum(r)
""""""
*/
struct Fenwick {
    n: usize,
    bit: Vec<i32>,
}
impl Fenwick {
    fn new(n: usize) -> Self { Fenwick { n, bit: vec![0; n + 1] } }
    // 1-based index: add v to position i
    fn add(&mut self, mut i: usize, v: i32) {
        while i <= self.n { self.bit[i] += v; i += i & (!i + 1); }
    }
    // 1-based prefix sum up to i
    fn sum(&self, mut i: usize) -> i32 {
        let mut s = 0i32;
        while i > 0 { s += self.bit[i]; i &= i - 1; }
        s
    }
}

// セグ木
/*
""""""
使い方（0-indexed, 半開区間）
- SegmentTree::new(n) or SegmentTree::from_vec(&values)
- set(i, v) / update(i, v): 点代入
- add(i, delta): 点加算
- get(i) -> 値取得
- query(l, r) -> 区間和 [l, r)
メモ:
- 配列長nに対して内部は2n長の反復型。nは2の冪でなくてOK。
""""""
*/
struct SegmentTree {
    n: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        SegmentTree { n, tree: vec![0; n * 2] }
    }

    fn from_vec(values: &[usize]) -> Self {
        let n = values.len();
        let mut st = SegmentTree { n, tree: vec![0; n * 2] };
        st.tree[n..n + n].copy_from_slice(values);
        for i in (1..n).rev() {
            st.tree[i] = st.tree[i * 2] + st.tree[i * 2 + 1];
        }
        st
        }

    fn update(&mut self, idx: usize, v: usize) {
        let mut i = idx + self.n;
        self.tree[i] = v;
        while i > 1 {
            i >>= 1;
            self.tree[i] = self.tree[i * 2] + self.tree[i * 2 + 1];
        }
    }

    fn set(&mut self, idx: usize, v: usize) {
        self.update(idx, v);
    }

    fn add(&mut self, idx: usize, delta: usize) {
        let cur = self.get(idx);
        self.update(idx, cur + delta);
    }

    fn get(&self, idx: usize) -> usize {
        self.tree[idx + self.n]
    }

    fn query(&self, mut l: usize, mut r: usize) -> usize {
        let mut left_sum = 0;
        let mut right_sum = 0;
        l += self.n;
        r += self.n;
        while l < r {
            if (l & 1) == 1 { left_sum += self.tree[l]; l += 1; }
            if (r & 1) == 1 { r -= 1; right_sum += self.tree[r]; }
            l >>= 1;
            r >>= 1;
        }
        left_sum + right_sum
    }

    fn len(&self) -> usize { self.n }
}

// 遅延セグ木
struct LazySegmentTree {
    n: usize,
    size: usize,
    tree: Vec<usize>,
    lazy: Vec<usize>,
}

impl LazySegmentTree {
    /*
    """"""
    使い方（0-indexed, 半開区間）
    - LazySegmentTree::new(n) or LazySegmentTree::from_vec(&values)
    - range_add(l, r, delta): 区間 [l, r) に +delta を加算
    - query(l, r) -> 区間和 [l, r)
    - get(i) -> 値取得（query(i, i+1)の糖衣）
    メモ:
    - 内部は次の2冪sizeに拡張。余り部分は0として扱う。
    - 和モノイド（加算）前提。min/max等にしたい場合は結合/遅延の定義を変更。
    """"""
    */
    fn new(n: usize) -> Self {
        let mut size = 1usize;
        while size < n { size <<= 1; }
        LazySegmentTree { n, size, tree: vec![0; size * 2], lazy: vec![0; size * 2] }
    }

    fn from_vec(values: &[usize]) -> Self {
        let n = values.len();
        let mut st = Self::new(n);
        for i in 0..n { st.tree[st.size + i] = values[i]; }
        for i in (1..st.size).rev() { st.tree[i] = st.tree[i * 2] + st.tree[i * 2 + 1]; }
        st
    }

    fn apply_node(&mut self, idx: usize, add: usize, len: usize) {
        if add == 0 { return; }
        self.tree[idx] += add * len;
        self.lazy[idx] += add;
    }

    fn push(&mut self, idx: usize, len: usize) {
        let add = self.lazy[idx];
        if add == 0 || len == 1 { return; }
        let half = len / 2;
        let left = idx * 2;
        let right = left + 1;
        self.apply_node(left, add, half);
        self.apply_node(right, add, half);
        self.lazy[idx] = 0;
    }

    fn range_add(&mut self, l: usize, r: usize, delta: usize) {
        self.range_add_inner(1, 0, self.size, l, r, delta);
    }

    fn range_add_inner(&mut self, idx: usize, nl: usize, nr: usize, l: usize, r: usize, delta: usize) {
        if r <= nl || nr <= l { return; }
        if l <= nl && nr <= r {
            self.apply_node(idx, delta, nr - nl);
            return;
        }
        self.push(idx, nr - nl);
        let mid = (nl + nr) / 2;
        self.range_add_inner(idx * 2, nl, mid, l, r, delta);
        self.range_add_inner(idx * 2 + 1, mid, nr, l, r, delta);
        self.tree[idx] = self.tree[idx * 2] + self.tree[idx * 2 + 1];
    }

    fn query(&mut self, l: usize, r: usize) -> usize {
        self.query_inner(1, 0, self.size, l, r)
    }

    fn query_inner(&mut self, idx: usize, nl: usize, nr: usize, l: usize, r: usize) -> usize {
        if r <= nl || nr <= l { return 0; }
        if l <= nl && nr <= r { return self.tree[idx]; }
        self.push(idx, nr - nl);
        let mid = (nl + nr) / 2;
        let left_sum = self.query_inner(idx * 2, nl, mid, l, r);
        let right_sum = self.query_inner(idx * 2 + 1, mid, nr, l, r);
        left_sum + right_sum
    }

    fn get(&mut self, i: usize) -> usize { self.query(i, i + 1) }
    fn len(&self) -> usize { self.n }
}


fn main(){
    input! { n: usize }
    let maxv: usize = 500_000;
    let mut lr: Vec<(i32,i32)> = Vec::with_capacity(n);
    for _ in 0..n { input!{ l: i32, r: i32 } lr.push((l,r)); }
    input! { q: usize }
    let mut xs: Vec<usize> = Vec::with_capacity(q);
    for _ in 0..q { input!{ x: usize } xs.push(x); }

    // Fenwick 長は b+1 まで更新するので +2 取る
    let mut fw = Fenwick::new(maxv + 2);

    let mut g = |v: usize, fw: &Fenwick| -> i64 {
        if v == 0 { return i64::MIN/4; }
        v as i64 + fw.sum(v) as i64
    };

    for &(l, r) in &lr {
        if g(maxv, &fw) < l as i64 || g(1, &fw) > r as i64 { continue; }
        let mut lo = 1usize; let mut hi = maxv + 1;
        while lo < hi {
            let mid = (lo + hi) >> 1;
            if g(mid, &fw) >= l as i64 { hi = mid; } else { lo = mid + 1; }
        }
        if lo > maxv { continue; }
        let a = lo;
        lo = a; hi = maxv + 1;
        while lo < hi {
            let mid = (lo + hi) >> 1;
            if g(mid, &fw) <= r as i64 { lo = mid + 1; } else { hi = mid; }
        }
        let b = lo - 1;
        if a <= b {
            fw.add(a, 1);
            fw.add(b + 1, -1);
        }
    }

    let mut out = String::new();
    for &x in &xs {
        let inc = fw.sum(x) as i64;
        out.push_str(&(x as i64 + inc).to_string());
        out.push('\n');
    }
    print!("{}", out);
}
