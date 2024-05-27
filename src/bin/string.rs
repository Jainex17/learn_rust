struct Person {
    name: String,
    age: i32,
    fav_color: String
}

fn display_info(person: &Person){
    println!("name: {}", person.name);
    println!("favorite color: {}", person.fav_color);
}

fn main(){
    let data = vec![
        Person{ 
            name: String::from("sophia"),
            age: 5,
            fav_color: String::from("yellow")
        },
        Person{ 
            name: String::from("Ann"),
            age: 12,
            fav_color: String::from("blue")
        },
        Person{ 
            name: String::from("bankai"),
            age: 13,
            fav_color: String::from("green")
        },
    ];


    for item in data{
        if item.age <= 10 {
            display_info(&item);
        }
    }

    let str = "Hello World";
    
    println!("LowerCase: {}", str.to_lowercase());
    println!("UpperCase: {}", str.to_uppercase());

}