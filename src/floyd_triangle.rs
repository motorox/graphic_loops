fn main() {
    let rows:i32 = 10;
    println!("\nFloyd's triangle of {} rows ...", rows);
    floyd_triangle(rows);
}

fn floyd_triangle(rows: i32) {
    let mut cursor = 1;
    for i in 1 .. rows+1 {
        for j in 0 .. i {
            print!("{} ", cursor);
            cursor += 1;
        }
        println!();
    }
}
