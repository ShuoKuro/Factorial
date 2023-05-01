fn main() {
    let n: i32 = 0;
    let mut ans: i32 = 1;
    if n < 0 {
        println!("{}", 0);
    } else {
        for n in 0..n {
            ans *= n + 1;
        }
        println!("{}", ans);
    }
}
