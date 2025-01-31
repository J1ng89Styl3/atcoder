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


fn main(){
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }
    let mut s_bitset = bitvec![0; n*(n - 1)];
    for i in 0..m{
        for j in 0..(n - 1){
            for k in (j + 1)..n{
                if a[j][i] == a[k][i]{
                    let bit_val = s_bitset[j * n + k];
                    s_bitset.set(j * n + k, !bit_val);
                }
            }
        } 
    }
    let ans = s_bitset.count_ones();
    println!("{}", ans);
}