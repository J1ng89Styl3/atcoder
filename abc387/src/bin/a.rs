#[warn(unused_imports)]
use std::iter::FromIterator;
use std::usize;
use ascii::Chars;
use bitvec::vec;
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

fn main() {
    input! {
        a:isize,
        b:isize,
        c:isize,
        d:isize,
    }
    let mut answer = 0;
    let x_diff = a.abs_diff(c);
    let y_diff = b.abs_diff(d);
    answer += (x_diff / 4) * y_diff * 4;
    let x_black = [3, 3, 1, 1];
    let point_black = [[2, 1], [1, 2], [0, 1], [1, 0]];
    for i in 0..x_diff % 4 {
        answer += x_black[(a + 1e9 as isize + i as isize) as usize % 4] * (y_diff / 2);
        answer += point_black[(a + 1e9 as isize + i as isize) as usize % 4]
            [(b + 1e9 as isize) as usize % 2]
            * (y_diff % 2);
    }
    println!("{answer}");
}