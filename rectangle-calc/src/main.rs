#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//this is a method on rectangle struct, not a function
impl Rectangle {
    /// Calculate the area
    ///
    /// Returns the area as u32
    fn area(&self) -> u32 {
        self.width * self.width
    }

    /// Verify if this Rectangle can hold another Rectangle
    ///
    /// other {Rectangle} the rectangle to verify against
    /// Returns true if self is big enough to contain the other, false otherwise
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// Create a square
    /// This is an *associated function* note that is has no `&self` as its first param
    ///
    /// size {u32} length of a side of the new rectangle
    /// Returns a Rectangle obj that is a square
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    //variables
    let width1: u32 = 30;
    let height1: u32 = 50;

    println!("Area = {} px", area(width1, height1));

    //tuples -> but problem is what if we mix up the order
    let rect_tup: (u32, u32) = (30, 50);

    println!("Area = {} px", area_tuple(rect_tup));

    //struct -> clear labeling of fields
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area = {} px", area_struct(&rect1)); //want main to retain this for future use -> hence use of ref

    //print rect info, after making it derive the Debug trait
    println!("Rectangle struct: {rect1:?}"); //indicates to use the Debug print format
    println!("Rectangle struct: {rect1:#?}"); //prettified Debug print format -> useful for larger structs

    //use of the dbg macro
    let scale: u32 = 2;
    let rect_d: Rectangle = Rectangle {
        width: dbg!(30 * scale), //the macro will print and evaluate the expression within it
        height: 50,
    };

    dbg!(rect_d);

    //Methods
    println!("Area of rect: {} sq. px.", rect1.area());

    let rect2: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4?: {}", rect1.can_hold(&rect4));

    //associated functions
    let sq1: Rectangle = Rectangle::square(3);
    println!(
        "sq1 is a square of {} width and {} height.",
        sq1.width, sq1.height
    )
}

/// Calculate the area of a rectangle
///
/// * `width` - u32 for width
/// * `height` - u32 for height
///
/// Returns a u32 for area
fn area(width: u32, height: u32) -> u32 {
    width * height
}

/// Calculate the area of a rectangle
///
/// * `dimensions` - (u32, u32) for width and height
///
/// Returns a u32 for area
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

/// Calculate the area of a rectangle
///
/// * `rect` - &Rectangle containing dimension info
///
/// Returns a u32 for area
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
