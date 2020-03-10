use std::io;
use std::ops::Range;

fn main() {
    println!("Hello, world!");
    let rows:i32 = 10; //read_data();
    println!("Factorial({}) = {}", rows, factorial(rows));
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

fn factorial (n:i32) -> u64 {
    let mut c = 1;
    let mut res:u64 = 1;
    while c <= n {
        res = res * c as u64;
        c +=1;
    }
    res
}

#[test]
fn test_factorial() {
    let fact = factorial(10);
    assert_eq!(fact, 3628800);
}
