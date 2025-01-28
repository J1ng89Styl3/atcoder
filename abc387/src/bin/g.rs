use std::collections::*;
use ac_library::{ModInt998244353 as o, SccGraph};
use proconio::{marker::*, *};

fn main()
{
    // 1) 入力
    input!{
        n: usize,
        m: usize,
        a: [Usize1; n]
    }
    let mut s = SccGraph::new(n);
    for i in 0 .. n {
        s.add_edge(a[i], i);
    }
    let mut p = vec![0; n];
    let mut h = HashSet::new();
    for v in s.scc()
    {
        for &w in v.iter()
        {
            p[w] = v[0];
        }
        if v.len() > 1 || a[v[0]] == v[0] {
            h.insert(v[0]);
        }
    }
    let mut g = vec![vec![]; n];
    for i in 0 .. n
    {
        if p[a[i]] != p[i] {
            g[p[a[i]]].push(i);
        }
    }
    let mut d = vec![vec![o::new(1); m]; n];
    fn f(g: &Vec<Vec<usize>>, p: usize, d: &mut Vec<Vec<o>>)
    {
        // 子を先に計算
        for &v in g[p].iter()
        {
            f(g, v, d);
            let mut t = o::new(0);
            for i in (0 .. d[p].len()).rev()
            {
                t += d[v][i];
                d[p][i] *= t;
            }
        }
    }
    for &r in &h {
        f(&g, r, &mut d);
    }
    let ans = h.iter()
               .map(|&r| d[r].iter().sum::<o>())
               .product::<o>();

    println!("{}", ans);
}
