fn print_result(is_gt_100: bool){
    match is_gt_100 {
        true => println!("number is grater then or equal to 100"),
        false => println!("numbe is less then 100"),
    }
}

fn main(){
    let num = 12;
    
    let is_gt_100 = num >= 100;
    print_result(is_gt_100);
}