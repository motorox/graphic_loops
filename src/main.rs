use std::io;
use std::ops::Range;
use graphic_loops::factorial;

fn main() {
    println!("Hello, world!");
    let rows:i32 = 10; //read_data();
    println!("Factorial({}) = {}", rows, graphic_loops::factorial(rows));
    for i in 0 .. rows+1 {
        let mut c = 0;
        while c <= ( rows - i - 2 ){
            print!(" ");
            c += 1;
        }
        for c in 0 .. i+1 {
            print!("{} ",factorial(i)/(factorial(c)*factorial(i-c)));
        }
        println!();
    }
}

//fn read_data() -> Option<u32> {
//    println!("How many rows? :");
//    let mut guess = String::new();
//    io::stdin().read_line(&mut guess)
//        .expect("failed to read line!");
//    let guess :u32 = match guess.trim().parse(){
//        Ok(num) => num,
//        Err(_) => expect("Type a number!"),
//    };
//    Option(guess)
//}
