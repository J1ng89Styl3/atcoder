## 入力
input! {
    a: usize, #整数
    s: [Chars; H], # 文字
}

## リストの初期化
let matrix = vec![vec![0; cols]; rows];


## 数を一桁ずつ分解して数列に変換
let digits: Vec<u32> = number
    .to_string() // 数字を文字列に変換
    .chars()     // 各文字をイテレート
    .map(|c| c.to_digit(10).unwrap()) // 文字を数字に変換
    .collect();   // Vecに収集


## 構造体
let pos: Vec<usize> = vec![];
###　 前後の処理
let pos: VecDeque<(usize)> = VecDeque::new();　
