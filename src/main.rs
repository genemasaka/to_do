use std::io;

fn main() {
    to_do();
}

fn to_do() {
    let mut to_do_list: Vec<String> = vec![];
    let mut flag = true;
    println!("Welcome to your to_do list");
    let breaker = String::from("break");
    while flag {
        println!("please type in what you want to do:");

        let mut item = String::new();

        io::stdin()
            .read_line(&mut item)
            .expect("Uh oh! Unable to read input");

        if item != breaker {
            to_do_list.push(item.replace("\n", ""));
            println!("\nTo_Do List:\n");
            for item in to_do_list.iter() {
                println!("{}", item);
            }
        } else {
            flag = false;
        }
    }
}
