#[warn(unused_imports)]
use std::usize;
use ascii::Chars;
use indexing::container_traits::FixedLength;
use num::abs;
use proconio::input;
use ac_library::*;
use std::collections::VecDeque;
use std::cmp;

fn solve(n: usize, xh: Vec<(usize, usize)>) -> f64{
    let mut max_ans:f64 = 10_f64.powi(9);
    let mut min_ans:f64 = 0.0;
    // 0の実験
    let mut last: f64 = -f64::MAX;
    let mut bool = true;
    for (x, h) in &xh{
        let this = (*h as f64)/(*x as f64);
        // 高さが足りなかった
        if this <= last{
            bool = false;
            break;
        }
        last = this
    }
    if bool{
       return -1.00;
    }
    // ここからが正しい答え
    while max_ans - min_ans > 10_f64.powi(-10){
        let this_ans:f64 = (max_ans + min_ans)/2.0;
        let mut last:(f64, f64) = (0.0, 0.0);
        let mut bool = true;
        let mut count: usize = 0;

        for (x, h) in &xh{
            if count == 0{
                count += 1;
                last = (*x as f64, *h as f64);
                continue;
            }
            if (this_ans - *h as f64)*last.0 >= (this_ans - last.1 as f64)*(*x as f64){
                min_ans = this_ans;
                bool = false;
                break;

            }
            last = (*x as f64, *h as f64);
        }
        if bool{
            max_ans = this_ans;
        }
    }
    return max_ans;
}

fn main(){
    input!{
        n: usize,
        xh: [(usize, usize); n],
    }
    let ans:f64 = solve(n, xh.clone());
    if ans < 0.0{
        println!("-1");
    }else{
        println!("{}", ans);
    }
}