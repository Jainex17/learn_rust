#[derive(Debug)]
struct Adult {
    name: String,
    age: i8
}

impl Adult {
    fn new(name: String, age: i8) -> Result<Self, String> {
        if age >= 21 {
            Ok(Self {
                name,
                age
            })
        }else {
            Err(String::from("Age must be at least 21"))
        }
    }
}
fn main(){
    let mark = Adult::new("mark".to_owned(), 25);
    let emily = Adult::new("emily".to_owned(), 15);

    match mark {
        Ok(data) => println!("name: {}, age: {}", data.name, data.age),
        Err(err) => println!("{err}"),
    }
    match emily {
        Ok(data) => println!("name: {}, age: {}", data.name, data.age),
        Err(err) => println!("{err}"),
    }
}