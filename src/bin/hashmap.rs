use std::collections::HashMap;

fn main(){
    let mut map:HashMap<&str, i32>  = HashMap::new();

    map.insert("chairs", 5);
    map.insert("beds", 3);
    map.insert("tables", 2);
    map.insert("couches", 0);

    let mut total_item  = 0;
    for (item, stock) in map.iter() {
        total_item = total_item + stock;
        match stock {
            0 => println!("{} is out of stock", item),
            _ => println!("{} {} available", stock, item),
        }
    }

    println!("There are {} items in store", total_item);
}