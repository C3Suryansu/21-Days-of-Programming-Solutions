/*
DAY 4:
Given a time in 12-hour AM/PM format, convert it to military (24-hour) time.

Details:
The program can be used to convert a time in 12-hour AM/PM format to military(24-hour) time.

Inputs:
Enter a time in 12-hour AM/PM format. 

Outputs:
Convert the input time to military time and print it on the screen.

Sample:

Input
11:30:30 PM

Output
23:30:30
*/
#[macro_use]
extern crate text_io;

fn main() {
    println!("Enter time in <hours:min:seconds>am/pm without any spaces and brackets");
    let date:String = read!();
    let mut l = 0;
    let mut h = "";
    for i in date.chars(){
        if i == ':'{
            h = &date[..l];
            break;
        }
        l = l + 1;
    }
    let mut hh = h.parse::<i32>().unwrap();
    let length = date.len();
    let time = &date[length-2 .. length];
    if time == "pm"{
        hh = hh + 12;
    }
    let s = &date[length-4 .. length-2];
    let m = &date[length-7 .. length-5];
    print!("{}:{}:{} hours\n",hh,m,s);
}
