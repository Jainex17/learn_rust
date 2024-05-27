use std::io;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Calculator {
    num1: f64,
    num2: f64,
    operation: Operation,
}

pub fn main() {
    print!("Choose an operation: \n1. Add\n2. Subtract\n3. Multiply\n4. Divide\n");

    let mut input_choice = String::new();
    io::stdin().read_line(&mut input_choice).expect("Failed to read line");

    let choice: Operation = match input_choice.trim() {
        "1" => Operation::Add,
        "2" => Operation::Subtract,
        "3" => Operation::Multiply,
        "4" => Operation::Divide,
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    let mut input_num1 = String::new();
    println!("Enter Number");
    io::stdin().read_line(&mut input_num1).expect("Failed to read line");
    
    let mut input_num2 = String::new();
    println!("Enter Number");
    io::stdin().read_line(&mut input_num2).expect("Failed to read line");
    
    let cal: Calculator = match parse_expression(&input_num1, &input_num2) {
        Ok((num1, num2)) => Calculator {
            num1,
            num2,
            operation: choice,
        },
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    
    let ans = match cal.operation{
        Operation::Add => cal.num1 + cal.num2,
        Operation::Subtract => cal.num1 - cal.num2,
        Operation::Multiply => cal.num1 * cal.num2,
        Operation::Divide => if cal.num2 == 0.0 {
            println!("why the fuck are you dividing value with 0");
            return;
        }else{
            cal.num1 / cal.num2
        },
    };

    println!("{:?} {:?} {:?} is {:?}", cal.num1, cal.operation, cal.num2, ans);

}

fn parse_expression(input1: &String, input2: &String) -> Result<(f64, f64), String> {
    let num1: f64 = match input1.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err("I said enter fucking number".to_owned()),
    };

    let num2: f64 = match input2.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err("I said enter fucking number".to_owned()),
    };

    Ok((num1, num2))
}