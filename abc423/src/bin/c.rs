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

fn main() {
    input! {
        n: usize,
        r: usize,
        l: [usize; n],
    }
    let mut left = n + 1;
    let mut right = 0;
    let mut count = 0;
    let mut closed = vec![0; n + 1];
    let mut closed_count = 0;

    for (i, &v) in l.iter().enumerate() {
        if v == 0 {
            count += 1;
            if left == n + 1{
                left = i + 1;
            }
            right = i + 1;
        }
    }
    if count == 0 {
        println!("{}", 0);
        return;
    }
    for i in 1..(n + 1) {
        if l[i - 1] == 1 {
            closed[i] = closed[i - 1] + 1;
        }
        else {
            closed[i] = closed[i - 1];
        }
    }
    closed_count = closed[right] - closed[left - 1];
    if r < left - 1{
        closed_count += cmp::max(left - 1, r) - r;
    }
    else if r > right{
        closed_count += r - cmp::min(right, r);
    }
    let ans = count + 2 * closed_count;
    println!("{}", ans);
}
