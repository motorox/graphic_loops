fn main(){
    let rows = 10;
    println!("\nPiramid of {} rows ...", rows);
    show_pyramid(rows, false);
    println!("\nDIAMOND ...");
    show_pyramid(rows, false);
    show_pyramid(rows-1, true);
}

fn odd_vals(i:i32) -> i32 {
    i*2-1
}

#[test]
fn test_odd_vals(){
    let arr = [1,3,5,7,9];
    let mut k = 1;
    for v in &arr {
        assert_eq!( odd_vals(k), *v);
        k += 1;
    }
}

fn show_pyramid(level: i32, reverse: bool) {
    // println!("{}, {}", level, reverse);
    let mut start:i32 = 1;
    let mut end:i32 = level;
    let mut step:i32 = 1;
    if reverse {
        start = level-1;
        end = 0;
        step = -1;
    }
    loop {
        if start == end {break;}
        for _b in 0..(odd_vals(level)/2 - start) {
            print!(" ");
        }
        for _c in 0..odd_vals(start){
            print!("*");
        }
        println!();
        start += step;
    }
}
