## 入力
input! {
    a: usize, #整数
    s: [Chars; H], # 文字
}

## リストの初期化
let matrix = vec![vec![0; cols]; rows];


## 変換
## 数を一桁ずつ分解して数列に変換
let digits: Vec<u32> = number
    .to_string() // 数字を文字列に変換
    .chars()     // 各文字をイテレート
    .map(|c| c.to_digit(10).unwrap()) // 文字を数字に変換
    .collect();   // Vecに収集

## 文字を文字列に変換


## 構造体
let pos: Vec<usize> = vec![];
###　 前後の処理
let pos: VecDeque<(usize)> = VecDeque::new();　

## 辞書型の初期化
let mut map: HashMap = HashMap::new();
*dict_b.entry(*v).or_insert(0) += 1; //辞書型の操作


## 集合型
let mut set = HashSet::new();  // Set
dict_b.get(&v).copied().unwrap_or(0);

let mut heap = BinaryHeap::new(); //優先度付きキュー(大きい方から取り出す)
let mut min_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new(); //優先度付きキュー(小さい方から取り出す)

## 複数値でのソート
edges.sort_by_key(|&(_, _, w)| w);

## 出力に関するもの
let result = numbers
        .iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");

