
fn main() {
    let s = "loveleetcode".to_owned();
    let res = get_dis(12, 3);
    println!("\n{}\n{:?}", s, res);
}

fn get_dis(l:usize, n:usize) -> Vec<i32> {
    let mut res = Vec::with_capacity(l);

    for i in n..=0 {
        print!("{i}");
        res.push(i as i32);
    }

    for i in 0..l {
        print!("{i}");
        res.push(i as i32);
    }
    res
}
