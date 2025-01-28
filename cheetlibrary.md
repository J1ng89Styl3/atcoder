**構造体**
use ac_library::ModInt998244353 as Mint; //ModInt998244353の構造体の定義

**union find**
https://github.com/atcoder/ac-library/blob/master/document_ja/dsu.md
let mut dsu = Dsu::new(n);
dsu.merge(u, v); # 結合
dsu.same(u, v); # 判定
dsu.leader(u); # 連結成分の代表元を返す
dsu.size(u); # uの所属する連結成分のサイズ

**Fenwick Tree**
https://atcoder.github.io/ac-library/production/document_ja/fenwicktree.html
区間和を更新、計算しやすい構造
let mut bit = FenwickTree::new(n, 0); # 初期値を0にした長さnの数列を作成
bit.add(i, x); # i番目の値をb[i] + xに更新
bit.sum(u..v); # 区間和の算出