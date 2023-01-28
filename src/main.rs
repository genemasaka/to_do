use std::io;
fn main() {
    to_do();
}
fn to_do() {
    let mut list: Vec<String> = vec![];
    let mut flag = true;
    while flag {
        println!("Please type in what you want to do:");
        let mut item = String::new();
        io::stdin()
            .read_line(&mut item)
            .expect("Failed to read item");
        let quit = String::from("quit");
        if item.to_string().trim() == quit {
            flag = false
        } else {
            println!("Add new item: [enter quit to exit]");
            list.push(item);
        }
    }
    println!("This is your current to_do list:\n");
    for item in list {
        println!("{}\n", item.to_string().trim());
    }
}
