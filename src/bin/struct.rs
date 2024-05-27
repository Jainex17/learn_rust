enum Drink_Flavors {
    Lime,
    Grape,
    Cola
}

struct Drink {
    flavor: Drink_Flavors,
    fluid_ounces: i32
}

fn print_drink_info(mydrink: Drink){
    match mydrink.flavor {
        Drink_Flavors::Cola => println!("Drink flavor is Cola"),
        Drink_Flavors::Lime => println!("Drink flavor is Lime"),
        Drink_Flavors::Grape => println!("Drink flavor is Grape"),
    }

    println!("Drink fluid is {:?}", mydrink.fluid_ounces);
}

fn main(){
    let my_drink = Drink {
        flavor: Drink_Flavors::Cola,
        fluid_ounces: 2
    };

    print_drink_info(my_drink);
}