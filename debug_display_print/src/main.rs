use std::fmt;

struct Unprintable(i32);

// derive from fmt::Debug to make it printable
#[derive(Debug)]
struct Printable(i32);

#[derive(Debug)]
struct Deep(Printable); // since both derive from debug -> both printable

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// display
struct Structure(i32);

// trait `fmt::Display` must be implemented manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: `f`.
        // Returns `fmt::Result` which indicates whether the operation succeeded or failed.
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//the issue with Display trait on a generic list is that write! generates a Result for every operation
// use the ? operator to only return the error if it hits one, otherwise continue
struct MyList(Vec<i32>);

impl fmt::Display for MyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // create a reference to `vec`.
        let vec: &Vec<i32> = &self.0;

        write!(f, "[")?;

        // Extract the value using tuple indexing,
        // Iterate over `v` in `vec` while enumerating the iteration
        for (i, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return
        write!(f, "]")
    }
}

fn main() {
    // DEBUG PRINT
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Printable(3));

    // The problem with `derive` is there is no control over how the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Printable(7)));

    // prettfied
    let name: &str = "Peter";
    let age: u8 = 27;
    let peter: Person = Person { name, age };
    println!("{peter:?}");
    println!("{peter:#?}");

    //DISPLAY PRINT - manually specify how to print
    let s: Structure = Structure(9);
    println!("{s}");

    // Generic containers have't been implemented with display as yet, fallback on debug
    // Non-generics are fine with Display trait
    let minmax: MinMax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {minmax}");
    println!("Debug: {minmax:?}");

    let big_range: MinMax = MinMax(-300, 300);
    let small_range: MinMax = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point: Point2D = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    // Error. Both `Debug` and `Display` were implemented, but `{:b}` requires `fmt::Binary` to be implemented -> won't work
    // println!("What does Point2D look like in binary: {:b}?", point);

    let v: MyList = MyList(vec![1, 2, 3]);
    println!("{v}");
}
