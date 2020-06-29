/*
DAY 15:
Tanya and Shiva have developed a secret way to talk with each other. They use a certain code to encrypt their message. 
Write a simple program to decrypt their hidden message.

Details:
The program is to decrypt a message which is based on the follow rules:-
1 – Every alternate character is replaced by either the previous or the next character as per their ascii values.
2- The 1st  character is replaced by the next ascii value character, the 2nd character by the previous ascii value character, 
the 3rd character again by the next ascii value character, and so on.

Inputs:
Take the encrypted message as input.
Outputs:
Print the hidden message as the output.

Sample:
Input:-
Idmkp Vpqmc
Output:-
Hello World

Easter Egg:- 
There is a hidden message by Tanya and Shiva, on the poster of the 21 days of programming event, software edition. 
It is encrypted in the same way as this program, try finding that string and uncovering the secret message.
*/
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
    .expect("Failed to read line");
    line.pop();
    line.pop();

    let new:&str = &line;
    let mut c:i32 = 0;
    let l:i32 = new.len() as i32;
    //let mut s:i32 = 0;

    for i in new.chars(){
        if i == ' '{
            print!(" ");
            //s += 1;
        }else if c%2 == 1{
            print!("{}",((i as char as u8) + 1) as u8 as char);
            c += 1;
        }else{
            print!("{}",((i as char as u8) - 1) as u8 as char);
            c += 1;
        }
    }
}
//EDDKBQF W@SHBAMDT ONU X@S
//changing the algorithm from odd - even, to even odd, we get the encoded message as 
//DECALRE VARIABLES NOT WAR
