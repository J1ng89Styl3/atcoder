**Fenwick Tree**
https://atcoder.github.io/ac-library/production/document_ja/fenwicktree.html
区間和を更新、計算しやすい構造
let mut bit = FenwickTree::new(n, 0); # 初期値を0にした長さnの数列を作成
bit.add(i, x); # i番目の値をb[i] + xに更新
bit.sum(u..v); # 区間和の算出

**Segment Tree**
https://atcoder.github.io/ac-library/production/document_ja/segtree.html
i番目の要素にxを代入
i番目の要素を取得
lからr番目の要素のモノイド積の計算を行うことができる
let mut segtree = Segtree::<Monoid>::new(n);; # Monoidを行うサイズnのセグ木を作成 
segtree.set(i, x); # xをi番目の値にセット　
segtree.get(i); # i番目の値を取得
segtree.prod(l, r); #[l, r - 1]の間に対して、モノイドに合わせて算出
segtree.all_prod(); #[0, n - 1]の間に対して、モノイドに合わせて算出
segtree.max_right(l, F); #F (prod(l, r))を満たす最大のrを返す O(log n)
segtree.min_left(f, F); # F(prod(l, r))を満たす最小のlを返す O(log n)
**Fの書き方サンプル**
|x| *x <= 10

