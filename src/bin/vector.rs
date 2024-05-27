fn main(){
    let my_number = vec![10, 20, 30, 40];

    for num in &my_number {
        match num{
            30 => println!("thirty "),
            _ => println!("{} ", num),
        }
    }

    println!("length of vector is {}", my_number.len());
}