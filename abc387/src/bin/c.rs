use proconio::input;
use std::collections::HashMap;

/// g(n, k) を計算するためのメモ化再帰
fn g(n: i64, k: i64, memo: &mut HashMap<(i64, i64), i64>) -> i64 {
    // n が 0 以下の場合は、ヘビ数は存在しないので 0
    if n <= 0 {
        return 0;
    }
    // メモにあればそれを返す
    if let Some(&res) = memo.get(&(n, k)) {
        return res;
    }

    // 第1項: 2桁以上のヘビ数の寄与
    let mut res = 0;
    for i in 0..=9 {
        let next_n = (n - i) / 10;      // 一桁(i)を取り除いた残り
        let next_k = k.max(i);         // 先頭桁が「k より真に大きい」条件を維持するため
        res += g(next_n, next_k, memo);
    }

    // 第2項: 1桁ヘビ数の寄与
    //   1桁の数はすべて「先頭桁が k より真に大きい」ものとして扱う
    //   ⇒ 1桁目が k+1, k+2, ..., min(n, 9) の個数
    let single_digit = (n.min(9) - k).max(0);
    res += single_digit;

    // メモに格納して返す
    memo.insert((n, k), res);
    res
}

/// n 以下のヘビ数の個数を求める
/// すなわち g(n, 0) を計算するラッパ関数
fn count_snake(n: i64) -> i64 {
    if n <= 0 {
        // 0 以下には正の整数が無いのでヘビ数も無い
        return 0;
    }
    let mut memo = HashMap::new();
    g(n, 0, &mut memo)
}

fn main() {
    input! {
        l: i64,
        r: i64,
    }

    // [L, R] に含まれるヘビ数の個数 = g(R, 0) - g(L-1, 0)
    let ans = count_snake(r) - count_snake(l - 1);
    println!("{}", ans);
}
