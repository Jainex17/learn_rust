fn main(){
    let mut i = 1;

    loop {
        if i == 11{
            break;
        }
        print!("{:?} ", i);
        i = i + 1;
    }
    println!();
    i = 1;
    while i <= 10 {
        print!("{:?} ", i);
        i = i + 1;
    }
}