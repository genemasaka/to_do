use std::io;

fn main() {
    to_do();
}

fn to_do() {
  let mut to_do_list:Vec<String> = vec![];
  let mut flag = true;
  println!("Welcome to your to_do list");

  while flag {
  println!("please type in what you want to do:");
    
  let mut item = String::new();
 
  io::stdin().read_line(& mut item).expect("Uh oh! Unable to read input");
  to_do_list.push(item);
  println!("{:?}", to_do_list);
  // if item == "break" { //Break the loop
  //           flag = false;
  //       } else {
  //           to_do_list.push();
  //           println!("You added to your to_do list");
            
  //       }
  //       println!("Add new item? [Enter 0 to exit]");
        
}
  
  }

