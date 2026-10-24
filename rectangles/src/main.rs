/*
    A program to calculate the area of a rectangle. Topic: Structs
*/


#[derive(Debug)]

struct Rectangle {
    length: u32,
    width: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn fits_in(&self, other: &Rectangle) -> bool {
        let fits_in = (self.length < other.length) && (self.width < other.width);
        fits_in 
    }
    fn square(size: u32) -> Self {
        Self {
            length: size,
            width: size
        }
    }
} 

fn main() {
    
    let rect1 = Rectangle {
        length: 30,
        width: 50
    };
    let sq1 = Rectangle::square(20);
    println!("The dimensions of rect1 are {rect1:#?}");
    println!(
        "The area of the rectangle is {} square units",
        rect1.area()
    );
    println!("The dimensions of sq1 are {sq1:?}");
    println!(
        "Does sq1 fit in rect1? {}",
        sq1.fits_in(&rect1)
    );
}
/*
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}   
*/