#[derive(PartialEq, Debug)]

pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }

    pub fn square(size: u32) -> Self {
        // fn square is a constructor shortcut
        Self {
            length: size,
            width: size,
        }
    }
    // Perimeter returns the perimeter
    pub fn perimeter(&self) -> u32 {
        let perimeter = 2 * (self.length + self.width);
        perimeter
    }
    // Diagonal returns the diagonal of the rectangle
    pub fn diagonal(&self) -> f64 {
        let p1: f64 = (self.length as f64) * (self.length as f64);
        let p2: f64 = (self.width as f64) * (self.width as f64);
        // calculate diagonal
        let diag = (p1 + p2).sqrt();
        diag
    }
}
// Tests:
#[cfg(test)]
mod tests {
    use super::*;

    // area test
    #[test]
    fn area_test() {
        let rect1 = Rectangle {
            length: 5,
            width: 3,
        };
        assert_eq!(
            rect1.area(),
            15,
            "Error in area function, expected 15, but got `{}` with tested values.",
            rect1.area()
        );
    }
    #[test]
    fn perimeter_test() {
        let rect1 = Rectangle {
            length: 5,
            width: 3,
        };
        assert_eq!(
            rect1.perimeter(),
            16,
            "Error in perimeter function, expected 16, but got `{}` with tested values.",
            rect1.perimeter()
        );
    }

    #[test]
    fn diagonal_test() {
        let rect1 = Rectangle {
            length: 4,
            width: 3,
        };
        assert_eq!(
            rect1.diagonal(),
            5.0,
            "Error in diagonal function, expected 5.0, but got `{}` with tested values.",
            rect1.diagonal()
        );
    }
}
