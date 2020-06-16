use std::io;

fn main() {
    let mut num = String::new();

    io::stdin().read_line(&mut num)
    .expect("Failed to read line");

    let num: i64 = num.trim().parse()
   .expect("please give me correct string number!");

    //let mut num : i64 = 3;
    for i in 0..(num + 1){
        for j in 1..(num - i + 1){
            print!(" ");
        }
        for k in 65..(65 + i) {
            print!("{}", k as u8 as char);
        }
        for l in (65..(65 + ( i - 1 ))).rev(){
            print!("{}", l as u8 as char);
        }
        println!("");
    }
}
