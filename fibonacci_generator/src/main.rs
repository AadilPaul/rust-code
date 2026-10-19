/*
    A program that generates the nth fibonacci number
*/

use std::io;

fn main() {
    'outer: loop {
        // input (str)
        let mut n = String::new();
        println!("Enter the position of the fibonacci number you would like (starting at 0, up to 92)");
        println!("If you would like to exit the program, enter a negative number:");

        io::stdin()
            .read_line(&mut n)
            .expect("Error in reading line");


        let n: i64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // calculation variables.
        let mut temp_var1: i64 = 1;
        let mut temp_var2: i64 = 1; 
        let mut temp_var3: i64 = 0;
        // result
        let fib_num: i64;

        if n > 2 {
            for _i in 1..=(n - 2) {
                temp_var3 = temp_var1 + temp_var2;
                temp_var1 = temp_var2;
                temp_var2 = temp_var3;
            }
        } else if n == 1 || n == 2 {
            temp_var3 = 1;
        } else if n < 0 {
            break 'outer;
        }

        fib_num = temp_var3;
        println!("The {n}th Fibonacci number is {fib_num}.");
    }
    println!("Have a nice day!");
}
