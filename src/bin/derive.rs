#[derive(Debug)]
enum Postion {
    Manager,
    Worker
}

#[derive(Debug)]
struct Employee {
    postion: Postion,
    salary: i32
}

fn main(){
    let my_postion = Postion::Manager;
    println!("{:?}", my_postion); // can print in debug because of derive

    let emp = Employee{
        postion: Postion::Worker,
        salary: 20000
    };

    println!("{:?}", emp); // with help of derive can print in debug
}