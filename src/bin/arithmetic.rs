fn display_result(num: i32) {
    println!("{num}")
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn sub(a: i32, b: i32) -> i32 {
    let sub_ans = a - b;
    sub_ans
}


fn main(){
    let sum_ans: i32  = sum(10, 20);
    print!("sum of 20 + 10 is ");
    display_result(sum_ans);
    
    print!("sum of 20 - 10 is ");
    display_result(sub(20, 10))
}