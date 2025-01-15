use std::iter::FromIterator;
#[warn(unused_imports)]
use std::usize;
use ascii::Chars;
use im_rc::HashSet;
use indexing::container_traits::FixedLength;
use num::abs;
use proconio::input;
use ac_library::*;
use web_sys::js_sys::Math::max;
use std::collections::VecDeque;
use std::cmp;
use ac_library::Dsu;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

"""
一つずつ拡張していく
"""

fn main(){
    input!{
        n: usize,
        m: usize,
        k: usize,
        mut edge: [(usize, usize, usize); m],
        a: [usize; k],
        b: [usize; k]
    }
}