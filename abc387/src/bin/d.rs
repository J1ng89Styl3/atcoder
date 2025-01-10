use ascii::Chars;
use proconio::input;
use ac_library::*;

fn bfs(start: (usize, usize), s: Chars, dmap: Chars){
    let mut pos: Vec<(usize, usize)> = vec![start];
    dmap[start.0][start.1] = (0, 0);
    while pos.len() > 0{
        let (y, x) = pos.pop();
        // 横移動は0 -> 1 縦移動は 1 -> 0
        // 横移動

        if now.0 != -1 {
            // 進むことができる
            if now.1 - 1 >= 0{
                // 壁でない
                if s[now.0][now.1] != "#"{
                    if s[now.0][now.1 - 1].1 == -1 || s[now.0][now.1 - 1].1 > s[now.0][now.1].0 + 1{
                        s[now.0][now.1 - 1].1 = s[now.0][now.1].0 + 1;
                        pos.append(());

                    }

                }
            }

        }
    }
    return ans;
}


fn main() {
    input!{
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut dmap:Vec<Vec<(isize, isize)>> = vec![vec![(-1, -1); w]; h];
    let mut start:(usize, usize) = (0, 0);
    for i in 0..h{
        for j in 0..w{
            if s[i][j] == "S"{
                start = (i, j);
                break;
            }
        }
    }
    let ans:isize = bfs(start, s, dmap);
    println!("{}", ans)
    
}


