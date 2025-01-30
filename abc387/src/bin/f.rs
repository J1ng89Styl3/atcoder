#[warn(unused_imports)]
use std::iter::FromIterator;
use std::usize;
use ascii::Chars;
use im_rc::HashSet;
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

fn main(){
    input!{
        n: usize, //辺数
        m: usize, //数
        a: [usize; n],
    }
    let mut graph = SccGraph::new(n);
    let mut tree: Vec<HashSet<usize>> = vec![HashSet::new(); n]; //子のノードを保存
    for i in 0..n{
        graph.add_edge(i, a[i] - 1);
        tree[a[i] - 1].insert(i);
    }
    let mut last:usize = 1;
    let mut dp:Vec<Vec<usize>> = vec![vec![0; m]; n];
    let scc = graph.scc();
    for i in 0..scc.len(){
        let this_node:HashSet<usize> = scc[i].iter().cloned().collect();
        // 子の値を求める
        let mut child_node: HashSet<usize> = HashSet::new();
        for j in &scc[i]{
            child_node = child_node.union(tree[*j].clone());
        }
        child_node = child_node.difference(this_node);
        // 子の数がなかった場合
        if child_node.len() == 0{
            for node in &scc[i]{
                for j in 0..m{
                    dp[*node][j] = last*(j + 1);
                }
            }
        }
        // 子の数があった場合
        else{
            for j in 0..m{
                for node in this_node{
                    for child in child_node{
                        dp[*node][j]
                    }
                }
            }
        }

    }

}

