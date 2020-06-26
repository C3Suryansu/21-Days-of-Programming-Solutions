/*
DAY 12:
Given a long integer, we need to find if the difference between sum of odd digits and sum of even digits is 0 or not. 
The indexes start from zero (0 index is for leftmost digit).

Details:
The program can be used to check if the difference between the sum of digits placed in the odd index positions and the sum of the 
digits placed in the even index positions is 0 or not.

Inputs:
Input a long integer value.

Outputs:
Find the difference between the sum of digits placed in the odd index positions and the sum of the digits placed in the even index positions 
and find if the difference between the two and if it’s 0 print YES or print NO.

Sample:

Inputs
1212112

Outputs:
YES

Explanation:
the odd position element is 2+2+1=5
the even position element is 1+1+1+2=5
the difference is 5-5=0.so print yes.
Note:- Take the 0th index as even


*/
#[macro_use]
extern crate text_io;

fn main() {
    let mut num:i64 = read!();
    let mut sum:i64 = 0;
    let mut c:i64 = 1;
    while num > 0{
        let val:i64 = num%10;
        sum = sum + (c * val);
        num = num / 10;
        c = c * -1;
    }
    if sum == 0{
        println!("YES");
    }else{
        println!("NO");
    }
}
