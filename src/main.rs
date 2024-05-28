use dotenv::dotenv;

mod useless_projects;

fn main() {
    println!("Main :)");

    dotenv().ok();
    useless_projects::main();
}