/*
    A program to calculate the area of a rectangle. Topic: Structs
*/

use std::io;


#[derive(Debug)]

struct Rectangle {
    length: u32,
    width: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    
    fn square(size: u32) -> Self {
        Self {
            length: size,
            width: size
        }
    }
    fn perimeter(&self) -> u32 {
        let perimeter = 2 * (self.length + self.width);
        perimeter
    }
    
    fn diagonal(&self) -> f64 {
        let p1: f64 = (self.length as f64) * (self.length as f64);
        let p2: f64 = (self.width as f64) * (self.width as f64);
        // calculate diagonal
        let diag = (p1 + p2).sqrt();
        diag
    }
    
} 

fn main() {
    let mut in_length = String::new();
    let mut in_width = String::new();
    let mut option = String::new();
    'outer: loop { 
        option.clear();
        println!("Do you want to...");
        println!("\t1.) Make a square");
        println!("\t2.) Make a rectangle");
        println!("\t3.) Exit the program");
        io::stdin().read_line(&mut option).expect("Readln error at ln 50 of src/main.rs");
        // let option = option.trim();

        if option.trim() == "2" {
            
            in_length.clear();
            in_width.clear();
            // Get length
            println!("Enter your rectangle's length: ");
            io::stdin().read_line(&mut in_length).expect("Readln failure at ln 59 in src/main.rs");

            // Get width
            println!("Enter your rectangle's width");
            io::stdin().read_line(&mut in_width).expect("Readln failure at ln 63 in src/main.rs");

            // Convert in_length to a u32
            let in_length: u32 = match in_length.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            // Convert in_width to a u32
            let in_width: u32 = match in_width.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let rect1 = Rectangle {
                length: in_length,
                width: in_width
            };
            
            
            let mut choice = String::new();

            'rect_inner: loop {
                println!("What would you like to do...");
                println!("\t1.) Find the area of your rectangle");
                println!("\t2.) Find the perimeter of your rectangle");
                println!("\t3.) Find the length of your rectangle's diagonals");
                println!("\t4.) View the size of your rectangle");
                println!("\t5.) Make a different shape");
                choice.clear();
                io::stdin().read_line(&mut choice).expect("Readln error at ln 92 of src/main.rs");

                if choice.trim() == "1" {
                    println!(
                        "The area of your rectangle is {}", 
                        rect1.area()
                    );
                } else if choice.trim() == "2" {
                    println!(
                        "The perimeter of your rectangle is {}",
                        rect1.perimeter()
                    );
                } else if choice.trim() == "3" {
                    println!(
                        "The length of your rectangle's diagonals is {}",
                        rect1.diagonal()
                    );
                } else if choice.trim() == "4" {
                    println!(
                        "The length of your rectangle is {}, and its width is {}",
                        rect1.length,
                        rect1.width
                    );
                } else if choice.trim() == "5" {
                    break 'rect_inner;
                } else {
                    println!("Sorry, I don't understand that");
                }

            }
        
        } else if option.trim() == "1" {

            println!("Enter the size of your square: ");
            in_length.clear();
            io::stdin().read_line(&mut in_length).expect("Readln failure at ln 127 in src/main.rs");
            
            let in_length: u32 = match in_length.trim().parse() {
                Ok(num) => num, 
                Err(_) => continue
            }; 
            let sq1 = Rectangle::square(in_length);
            let mut choice = String::new();

            'sq_inner: loop {

                println!("What would you like to do...");
                println!("\t1.) Find the area of your square");
                println!("\t2.) Find the perimeter of your square");
                println!("\t3.) Find the length of your square's diagonals");
                println!("\t4.) View the size of your square");
                println!("\t5.) Make a different shape");

                choice.clear();
                io::stdin().read_line(&mut choice).expect("Readln failure at ln 143 in src/main.rs");      
                // let choice = choice.;

                if choice.trim() == "1" {
                    println!(
                        "The area of your square is {}",
                        sq1.area()
                    );
                } else if choice.trim() == "2" {
                    println!(
                        "The perimeter of your square is {}",
                        sq1.perimeter()
                    );
                } else if choice.trim() == "3" {
                    println!(
                        "The length of your square's diagonals is {}",
                        sq1.diagonal()
                    );
                } else if choice.trim() == "4" {
                    println!("The side length of your square is {in_length}");
                } else if choice.trim() == "5" {
                    break 'sq_inner; 
                } else {
                    println!("Sorry, I don't understand that");
                }
                
            }
        } else if option.trim() == "3" {
            break 'outer;
        } else {
            println!("Sorry, I don't understand that");
        }
        
    } // 'outer: loop
    println!("Have a nice day!");
} // fn main
