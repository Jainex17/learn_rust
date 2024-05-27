fn coordinate() -> (f64, f64) {
    (2.32,6.00)
}

fn main(){
    let (x_cord, y_cord) = coordinate();

    println!("x coordinate is {:?}", x_cord);
    println!("y coordinate is {:?}", y_cord);

    if(y_cord > 5.00){
        println!("y coordinate is gratear then 5");
    }
    else if y_cord < 5.00 {
        println!("y coordinate is less then 5");
    }else {
        println!("y coordinate is 5");
    }
}