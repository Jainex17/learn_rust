enum Colors {
    Black,
    Red
}

// advance anum
enum Discount {
    Percent(f64),
    Flat(i32),
    Custom(String)
}

fn main(){
    let pick_color: Colors = Colors::Red;
    
    match pick_color {
        Colors::Black => println!("slected color is black"),
        Colors::Red => println!("slected color is red"),
    }

    let discount = Discount::Percent(10.0);
    match discount {
        Discount::Percent(abc) => println!("{}% discount", abc),
        Discount::Flat(flat) => println!("{} flat discount", flat),
        Discount::Custom(custom) => println!("{} custom discount", custom),
    }
}