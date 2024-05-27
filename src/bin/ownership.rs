struct Grocery {
    id: i32,
    qty: i32
}

fn display_id(my_grocery: &Grocery){
    println!("id: {}", my_grocery.id);
}

fn display_qty(my_grocery: &Grocery){
    println!("qty: {}", my_grocery.qty);
}

fn main(){
    let my_grocery = Grocery{
        id: 1,
        qty: 5
    };

    display_id(&my_grocery);
    display_qty(&my_grocery);
}