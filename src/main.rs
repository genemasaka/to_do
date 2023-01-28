//imports the standard input/output library from the Rust standard library.
use std::io;

//entry point of the program.
fn main() {
    // calls the to_do function which creates a list of items user inputs.
    to_do();
}

//creates an empty vector of strings called list and a variable flag set to true.
fn to_do() {
    let mut list: Vec<String> = vec![];
    let mut flag = true;
    
   //while loop that continues as long as flag is true.
    while flag {
        // the program prompts the user to enter an item and assigns it to the variable item.
        println!("Please type in what you want to do:");
        let mut item = String::new();
        io::stdin()
            .read_line(&mut item)
            .expect("Failed to read item");
        
        //string variable called quit is created and assigned the string "quit".
        let quit = String::from("quit");
        
        //checks if the input item is equal to the quit string, if so the flag is set to false and the loop terminates.
        if item.to_string().trim() == quit {
            flag = false
        } else {
            //If the item is not equal to quit, the program adds the item to the list vector and prompts for another input.
            println!("Add new item: [enter quit to exit]");
            list.push(item);
        }
    }
    //once the loop terminates, the program prints the current to_do list items stored in the list vector.
    println!("This is your current to_do list:\n");
    for item in list {
        println!("{}\n", item.to_string().trim());
    }
}
