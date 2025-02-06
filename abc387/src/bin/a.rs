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
use itertools::iproduct;

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

// 下方向 L 辺先の頂点の個数を数える関数
fn count_nodes(a: u128, L: u64, N: u128) -> u128 {
    if a > N { return 0; }
    if L >= 64 { return 0; } // L>=64 なら 2^L は十分大きいので 0 を返す
    let low = a << L; // a * 2^L
    if low > N { return 0; }
    let range = (1u128 << L) - 1;
    let high = low.saturating_add(range);
    let high = if high > N { N } else { high };
    high - low + 1
}
 
fn main() {
    input!{
        v1: i64,
        v2: i64,
        v3: i64,
    }
    let (a1,b1,c1) = (0,0,0);
    for (a2,b2,c2,a3,b3,c3) in iproduct!(-7..=7,-7..=7,-7..=7,-7..=7,-7..=7,-7..=7){
        let v12 = ((a1+7).min(a2+7)-a1.max(a2)).max(0)*((b1+7).min(b2+7)-b1.max(b2)).max(0)*((c1+7).min(c2+7)-c1.max(c2)).max(0);
        let v23 = ((a2+7).min(a3+7)-a2.max(a3)).max(0)*((b2+7).min(b3+7)-b2.max(b3)).max(0)*((c2+7).min(c3+7)-c2.max(c3)).max(0);
        let v31 = ((a3+7).min(a1+7)-a3.max(a1)).max(0)*((b3+7).min(b1+7)-b3.max(b1)).max(0)*((c3+7).min(c1+7)-c3.max(c1)).max(0);
        let v123 = ((a1+7).min(a2+7).min(a3+7)-a1.max(a2).max(a3)).max(0)*((b1+7).min(b2+7).min(b3+7)-b1.max(b2).max(b3)).max(0)*((c1+7).min(c2+7).min(c3+7)-c1.max(c2).max(c3)).max(0);
        if v123 == v3 && (v12+v23+v31)-3*v123 == v2 && 7*7*7*3-2*v2-3*v3 == v1{
            println!("Yes");
            println!("{} {} {} {} {} {} {} {} {}",a1,b1,c1,a2,b2,c2,a3,b3,c3);
            return;
        } 
    }
    println!("No")
}