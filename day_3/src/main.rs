#[macro_use]
extern crate text_io;

fn main() {
    let mut a : i64 = read!();
    let n : i64 = read!();
    let r : i64 = read!();
    let mut s : i64 = 0;

    for _i in 0..n{
        print!("{} ",a);
        s += a;
        a *=  r;
    }
    
    println!("");
    println!("{}", s);
}
