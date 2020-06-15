/*
DAY 1:
Write a Program to solve Quadratic Equation depending upon the value of Delta.
Details:
The program is used to solve quadratic equation i.e. ax2 + bx + c = 0 depending upon the value of Delta. 
The formula for calculating Delta is d = b*b-4*a*c.
Inputs:
Take the coefficients a, b and c as input.
Outputs:
Print the value of Delta.
Print the Roots.
Sample:
Inputs
5
2
1
Outputs:
-16
(-2 + 4i)/10 
(-2 – 4i)/10
*/

use std::io;

fn main() {

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    io::stdin().read_line(&mut a)
    .expect("Failed to read line");
    io::stdin().read_line(&mut b)
    .expect("Failed to read line");
    io::stdin().read_line(&mut c)
    .expect("Failed to read line");

    let a: f64 = a.trim().parse()
   .expect("please give me correct string number!");
    let b: f64 = b.trim().parse()
   .expect("please give me correct string number!");
    let c: f64 = c.trim().parse()
   .expect("please give me correct string number!");

    println!("{}", (b * b) - (4.0 * a * c));

    let mut D : f64 = ((b * b) - (4.0 * a * c));

    let mut det = D.abs().sqrt();

    if D < 0.0 {
        println!("(-{} - {}i)/{}", b, det, 2.0 * a);
        println!("(-{} + {}i)/{}", b, det, 2.0 * a);
    }
    else if D > 0.0{
        println!("(-{} - {})/{}", b, det, 2.0 * a);
        println!("(-{} + {})/{}", b, det, 2.0 * a);
    }
    else{
        println!("-{}/{}", b, 2.0 * a);
    }
}
