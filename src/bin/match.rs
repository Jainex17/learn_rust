enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32),
}

fn main(){
    let num = 3;
    let some_bool = false;

    match num {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        other => println!("its something else: {}", other),
    }

    match some_bool{
        true => println!("its true"),
        false => {
            println!("its false");
            println!("ok man");
        },        
    }

    // advance match
    let tickets = vec![
        Ticket::Vip(3000, String::from("Ann")),
        Ticket::Standard(500)
    ];

    for item in tickets{
        match item {
            Ticket::Backstage(price, name) => println!("BackStage => Name: {:?}, Price: {:?}", name, price),
            Ticket::Vip(price, name) => println!("Vip => Name: {:?}, Price: {:?}", name, price),
            Ticket::Standard(price) => println!("Standard => Price: {:?}", price),
        }
    }
}
