pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = vec![num % 10];
    //There are no 10+ digit armstrong numbers able to be stored in a u32
    for e in 1..10 {
        let n = num / 10u32.pow(e);
        if n == 0 {
            break;
        }
        digits.push(n % 10);
    }
    let len = digits.len() as u32;
    let mut total = 0;
    for d in digits {
        total += d.pow(len);
    }
    total == num
}
