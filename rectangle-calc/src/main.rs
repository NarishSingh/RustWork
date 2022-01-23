#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //variables
    let width1: u32 = 30;
    let height1: u32 = 50;

    println!("Area = {} px", area(width1, height1));

    //tuples -> but problem is what if we mix up the order
    let rect1: (u32, u32) = (30, 50);

    println!("Area = {} px", area_tuple(rect1));

    //struct -> clear labeling of fields
    let rect2: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area = {} px", area_struct(&rect2)); //want main to retain this for future use -> hence use of ref

    //print rect info, after making it derive the Debug trait
    println!("Rectangle struct: {:?}", rect2); //indicates to use the Debug print format
    println!("Rectangle struct: {:#?}", rect2); //prettified Debug print format -> useful for larger structs

    //use of the dbg macro
    let scale: u32 = 2;
    let rect_d: Rectangle = Rectangle{
        width: dbg!(30 * scale), //the macro will print and evaluate the expression within it
        height: 50
    };

    dbg!(rect_d);
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