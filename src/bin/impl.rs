enum Box_Color {
    Red,
    White,
    Black
}

struct Box {
    dimensions: (f64, f64, f64),
    weight: f64,
    color: Box_Color,
}

impl Box {

    fn create_new_box(dimensions: (f64, f64, f64), weight: f64, color: Box_Color) -> Self {
        Self {
            dimensions,
            weight,
            color
        }
    }

    fn display_box_info(&self){
        println!("dimensions: ({}, {}, {})", self.dimensions.0, self.dimensions.1, self.dimensions.2);
        println!("width: {}", self.weight);

        match self.color {
            Box_Color::Black => println!("color: black"),
            Box_Color::Red => println!("color: red"),
            Box_Color::White => println!("color: white"),
        }
    }
}

fn main(){
    let shipping_box = Box {
        dimensions: (12.2, 21.3, 45.23),
        weight: 23.2,
        color: Box_Color::Red
    };

    shipping_box.display_box_info();

    let newbox = Box::create_new_box((21.1,32.1,42.5), 32.2, Box_Color::Black);   

    newbox.display_box_info();
}