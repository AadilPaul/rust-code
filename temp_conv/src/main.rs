/*
    A program that converts temperatures between Fahrenheit and Celsius
*/

use std::io;

fn main() {
    println!("Welcome to the temperature converter!");

    'temp_conv: loop {
        let mut direction = String::new(); 

        println!("What would you like to do?");
        println!("1.) Go from Fahrenheit to Celsius");
        println!("2.) Go from Celsius to Fahrenheit");
        println!("3.) Exit the program");

        io::stdin()
            .read_line(&mut direction)
            .expect("Failed in reading line");

        if direction.trim() == "1" {

            println!("Enter the temperature in Fahrenheit: ");

            let mut in_temp = String::new();

            io::stdin()
                .read_line(&mut in_temp)
                .expect("Failed in reading line");
            
            let in_temp: f32 = in_temp.trim().parse().expect("Error: NaN");

            let out_temp: f32 = (in_temp - 32.0) / 1.8;

            println!("{in_temp} degrees Fahrenheit is {out_temp} degrees Celsius");
            
        } else if direction.trim() == "2" {

            println!("Enter the temperature in Celsius : ");

            let mut in_temp = String::new();

            io::stdin() 
                .read_line(&mut in_temp)
                .expect("\nFailed in reading line");
            
            let in_temp: f32 = in_temp.trim().parse().expect("Error: NaN");

            let out_temp: f32 = (in_temp * 1.8) + 32.0;

            println!("{in_temp} degrees Celsius is {out_temp} degrees Fahrenheit");

        } else if direction.trim() == "3" {

            break 'temp_conv;

        } else {

            println!("Unknown input. Please try again");

        }
    }
    
    println!("Have a nice day!")
}
