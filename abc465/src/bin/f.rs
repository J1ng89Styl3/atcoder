#[warn(unused_imports)]
use std::iter::FromIterator;
use std::usize;
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

fn lcm_cap(a: usize, b: usize, y: usize) -> usize {
    if a == 0 || b == 0{
        return 0;
    }
    let g = num::integer::gcd(a, b);
    let x = a/g;
    if x > y/b{
        y + 1
    }else{
        x * b
    }
}


fn main() {
    input! {
        n: usize,
        sy: [(Chars, usize); n],
        q: usize,
        query: [(Chars, Chars); q],
    }
    let mut dp = vec![vec![vec![vec![vec![vec![0; 11]; 11]; 11]; 11]; 11]; 11];

    for (s, y) in sy {
        let mut map = vec![0; 6];
        for i in 0..6 {
            map[i] = s[i].to_digit(10).unwrap() as usize + 1;
        }
        dp[map[0]][map[1]][map[2]][map[3]][map[4]][map[5]] += y;
    }

    for i1 in 1..11{
        for i2 in 0..11{
            for i3 in 0..11{
                for i4 in 0..11{
                    for i5 in 0..11{
                        for i6 in 0..11{
                            dp[i1][i2][i3][i4][i5][i6] += dp[i1 - 1][i2][i3][i4][i5][i6];
                        }
                    }
                }
            }
        }
    }

    for i1 in 0..11{
        for i2 in 1..11{
            for i3 in 0..11{
                for i4 in 0..11{
                    for i5 in 0..11{
                        for i6 in 0..11{
                            dp[i1][i2][i3][i4][i5][i6] += dp[i1][i2 - 1][i3][i4][i5][i6];
                        }
                    }
                }
            }
        }
    }

    for i1 in 0..11{
        for i2 in 0..11{
            for i3 in 1..11{
                for i4 in 0..11{
                    for i5 in 0..11{
                        for i6 in 0..11{
                            dp[i1][i2][i3][i4][i5][i6] += dp[i1][i2][i3 - 1][i4][i5][i6];
                        }
                    }
                }
            }
        }
    }

    for i1 in 0..11{
        for i2 in 0..11{
            for i3 in 0..11{
                for i4 in 1..11{
                    for i5 in 0..11{
                        for i6 in 0..11{
                            dp[i1][i2][i3][i4][i5][i6] += dp[i1][i2][i3][i4 - 1][i5][i6];
                        }
                    }
                }
            }
        }
    }

    for i1 in 0..11{
        for i2 in 0..11{
            for i3 in 0..11{
                for i4 in 0..11{
                    for i5 in 1..11{
                        for i6 in 0..11{
                            dp[i1][i2][i3][i4][i5][i6] += dp[i1][i2][i3][i4][i5 - 1][i6];
                        }
                    }
                }
            }
        }
    }

    for i1 in 0..11{
        for i2 in 0..11{
            for i3 in 0..11{
                for i4 in 0..11{
                    for i5 in 0..11{
                        for i6 in 1..11{
                            dp[i1][i2][i3][i4][i5][i6] += dp[i1][i2][i3][i4][i5][i6 - 1];
                        }
                    }
                }
            }
        }
    }
    let mut ans = vec![];
    for (s, t) in query {
        let mut m = vec![0; 6];
        let mut M = vec![0; 6];
        let mut ok = true;
        for i in 0..6 {
            let l = s[i].to_digit(10).unwrap() as usize;
            let r = t[i].to_digit(10).unwrap() as usize;
            if l > r {
                ok = false;
            }
            m[i] = l;
            M[i] = r + 1;
        }
        if !ok {
            ans.push(0 as i64);
            continue;
        }
        let mut res: i64 = 0;
        for mask in (0 as usize)..64 {
            let idx = [
                if mask & 1 == 0{ M[0] } else{ m[0] },
                if mask & 2 == 0{ M[1] } else{ m[1] },
                if mask & 4 == 0{ M[2] } else{ m[2] },
                if mask & 8 == 0{ M[3] } else{ m[3] },
                if mask & 16 == 0{ M[4] } else{ m[4] },
                if mask & 32 == 0{ M[5] } else{ m[5] },
            ];
            if mask.count_ones() % 2 == 0 {
                res += dp[idx[0]][idx[1]][idx[2]][idx[3]][idx[4]][idx[5]] as i64;
            } else {
                res -= dp[idx[0]][idx[1]][idx[2]][idx[3]][idx[4]][idx[5]] as i64;
            }
        }
        ans.push(res);
    }
    for e in ans {
        println!("{}", e);
    }
}
