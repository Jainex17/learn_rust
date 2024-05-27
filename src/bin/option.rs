struct Student {
    name: String,
    locker: Option<i32>,
}

fn display_std_info(student: Student){
    print!("Name: {},", student.name);

    match student.locker {
        Some(num) => println!(" locker: {}", num),
        None => {}
    }
}

fn main(){
    let std1 = Student{
        name: "mark".to_owned(),
        locker: Some(2),
    };
    let std2 = Student{
        name: "emily".to_owned(),
        locker: None
    };

    display_std_info(std1);
    display_std_info(std2);
}