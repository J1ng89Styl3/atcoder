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


fn main() {
    input! {
        n:usize,
        q:usize,
    }

    let mut lefts = BTreeMap::new();
    for i in 0..=n + 1 {
        lefts.insert(i, i);
    }
    let mut cnts = vec![1; n + 2];
    for _ in 0..q {
        input! {
            t:usize,
        }

        if t == 1 {
            input! {
                x:usize,
                c_nxt:usize,
            }

            let (&l_x, &c_prv) = lefts.range(..=x).last().unwrap();
            let (&r_x, &c_r) = lefts.range(x + 1..).next().unwrap();
            let (&_, &c_l_l) = lefts.range(..l_x).last().unwrap();

            cnts[c_prv] -= r_x - l_x;
            *lefts.get_mut(&l_x).unwrap() = c_nxt;
            cnts[c_nxt] += r_x - l_x;

            if c_nxt == c_l_l {
                lefts.remove(&l_x);
            }
            if c_nxt == c_r {
                lefts.remove(&r_x);
            }
        } else {
            input! {
                c:usize,
            }

            let ans = cnts[c];
            println!("{}", ans);
        }
    }
}
