/*
DAY 17:
Write a program which uses a recursive function to print a Fibonacci series.

Details:
The program will use a recursive function to print the Fibonacci series. The function will have the following parameters as input:-
a-     First element of the series
b-     Second element of the series
n-     number of elements of the series you wanna display
The program should take user input of the above parameters and pass it to the function for execution.
You can add more parameters if you want, but the above 3 are required for certainity.

Inputs:
Input the starting element a, second element b, and the number of the elements you wanna display.

Outputs:
Print the series till the nth element.

Sample:

Inputs
1
2
5

Outputs:
1 2 3 5 8
*/
#[macro_use]
extern crate text_io;

fn fibbonacci(n:i32, mut f_num:i32, mut s_num:i32) -> i32{
    if n == 0{
        return 1
    }else{
        print!("{} ", s_num);
        let temp:i32 = f_num;
        f_num = s_num;
        s_num += temp;
        fibbonacci(n-1, f_num, s_num);
        return 1
    }
}
fn main() {
    let f_num:i32 = read!();
    let s_num:i32 = read!();
    let n:i32 = read!();
    print!("{} ", f_num);
    fibbonacci(n, f_num, s_num);
}
