#[warn(unused_imports)]
use std::usize;
use ascii::Chars;
use proconio::input;
use ac_library::*;
use std::collections::VecDeque;

fn bfs(dmap: Vec<Vec<char>>, mut dmap_step: Vec<Vec<(isize, isize)>>) -> isize{
    let mut goal:(usize, usize) = (0, 0);
    let mut pos: VecDeque<(usize, usize, usize)> = VecDeque::new();
    for i in 0..dmap.len(){
        for j in 0..dmap[0].len(){
            if dmap[i][j] == 'S'{
                dmap_step[i][j] = (0, 0);
                pos.push_back((i, j, 0));
                pos.push_back((i, j, 1));
            }
            if dmap[i][j] == 'G'{
                goal = (i, j);
            }
        }
    }
    //マップはdmapで判定
    //ステップはsで判定

    while pos.len() > 0{
        let (y, x, t) = pos.pop_front().unwrap();
        // 横移動は0 -> 1 縦移動は 1 -> 0
        // 横移動
        if t == 0 {
            // 左に進める
            if x >= 1{
                // 壁でない
                if dmap[y][x - 1] != '#'{
                    if dmap_step[y][x - 1].1 == -1{
                        dmap_step[y][x - 1].1 = dmap_step[y][x].0 + 1;
                        pos.push_back((y, x - 1, 1));
                    }
                }
            }
            // 右に進める
            if x + 1 < dmap[0].len(){
                // 壁でない
                if dmap[y][x + 1] != '#'{
                    if dmap_step[y][x + 1].1 == -1{
                        dmap_step[y][x + 1].1 = dmap_step[y][x].0 + 1;
                        pos.push_back((y, x + 1, 1));
                    }
                }
            }
        }
        // 縦移動
        if t == 1{
            //上に進める
            if y >= 1{
                //壁でない
                if dmap[y - 1][x] != '#'{
                    if dmap_step[y - 1][x].0 == -1{
                        dmap_step[y - 1][x].0 = dmap_step[y][x].1 + 1;
                        pos.push_back((y - 1, x, 0));
                    }
                }
            }
            //下に進める
            if y + 1 < dmap.len(){
                //壁でない
                if dmap[y + 1][x] != '#'{
                    if dmap_step[y + 1][x].0 == -1{
                        dmap_step[y + 1][x].0 = dmap_step[y][x].1 + 1;
                        pos.push_back((y + 1, x, 0));
                    }
                }
            }
        }
    }

    let ans: isize = if dmap_step[goal.0][goal.1] == (-1, -1) {
        -1
    } else {
        let (x, y) = dmap_step[goal.0][goal.1]; // タプルの要素を展開
        if x == -1 && y == -1 {
            -1
        } else if x == -1 {
            y
        } else if y == -1 {
            x
        } else {
            x.min(y) // タプルの最小値を取得
        }
    };
    return ans;
}


fn main() {
    input!{
        h: usize,
        w: usize,
        s: [String; h],
    }
    let dmap: Vec<Vec<char>> = s.iter().map(|line| line.chars().collect()).collect();
    let dmap_step:Vec<Vec<(isize, isize)>> = vec![vec![(-1, -1); w]; h];
    let ans:isize = bfs(dmap, dmap_step);
    println!("{}", ans)
    
}


