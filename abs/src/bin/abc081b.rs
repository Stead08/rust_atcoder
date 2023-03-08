use proconio::input;
//整数を二進数表記した時に、1の位から連続する0の個数の最小値を求めれば良い
fn main() {
    input! { n: usize }
    let s = (0..n)
        .map(|_| {
            input! { a: u32 }
            a.trailing_zeros() //最下位の桁から連続する0の個数を求める
        })
        .min()
        .unwrap();
    println!("{}", s);
}
