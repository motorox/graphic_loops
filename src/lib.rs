pub fn factorial (n:i32) -> u64 {
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

pub fn odd_vals(i:i32) -> i32 {
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
