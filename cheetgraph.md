## functional graph
https://atcoder.jp/contests/abc357/editorial/10185
分岐の根を定める
木dp


## 強連結成分(SCC)
ある有向グラフにおいて、互いに行き来が可能な頂点の集合を求められる。
let mut graph = SccGraph::new();
for (a, b) in ab{
    graph.add_edge(a, b);
}
let scc = graph.scc();
println!("{}", scc.len());