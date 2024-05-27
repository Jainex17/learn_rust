fn main(){
    let is_monday: bool = false;
    let is_tuesday: bool = true;

    if is_monday{
        println!("Today is Monday");
    }
    else if is_tuesday{
     println!("Today is Tuesday");   
    }

    let num = -12;

    if num > 5 {
        println!("{num} is grater then 5");
    }else if num == 5{
        println!("{num} is exactly 5");
    }else {
        println!("{num} is less then 5");
    }
}