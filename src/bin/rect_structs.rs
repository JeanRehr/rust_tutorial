#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // needs & to not take ownership and be moved
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }

    fn rect_builder(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/* old builder
fn rect_builder1(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}
*/

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the rect1 is {} square pixels", rect1.area());

    //let rect2: Rectangle = rect_builder1(10, 10);

    //don't take self as parameter so isn't a method, only an associated fn
    //needs to use this syntax
    let rect2: Rectangle = Rectangle::rect_builder(10, 15);
    println!("the area of tthe rect2 is {} square pixels", rect2.area());

    //needs #[derive(Debug)] before the struct definition
    println!("rect1 is: {:?}", rect1);

    println!("rect2 is: {:#?}", rect2);

    //Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to
    //println!, which prints to the standard output console stream (stdout)
    let scale: u32 = 2;
    let rect3: Rectangle = Rectangle {
        width: dbg!(50 * scale),
        height: 60,
    };

    dbg!(&rect3);

    if rect1.width() {
        println!("rect1 has a nonzero width of {}", rect1.width);
    }

    if rect2.height() {
        println!("rect2 has a nonzero height of {}", rect2.height);
    }

    //wihtout & in the function declaration and here, rect2 would be moved
    //into the can_hold function and be taken ownership, println
    //wouldn't work as it needs to borrow it
    //println!("can rect1 hold rect2? {}", rect1.can_hold(rect2));
    //println!("can rect1 hold rect3? {}", rect1.can_hold(rect3));
    //println!("the area of tthe rect2 is {} square pixels", rect2.area());
    //println!("the area of tthe rect3 is {} square pixels", rect3.area());

    //borrows (takes a reference) and not ownership
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1: Rectangle = Rectangle::square(20);

    println!(
        "square1 width and height: {} {}",
        square1.width, square1.height
    )
}

/* possible BUILDER PATTERN
// Step 1: Define the Rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

// Step 2: Define the RectangleBuilder struct
struct RectangleBuilder {
    width: Option<u32>,
    height: Option<u32>,
}

impl RectangleBuilder {
    // Step 3: Implement a method to create a new builder
    fn new() -> Self {
        RectangleBuilder {
            width: None,
            height: None,
        }
    }

    // Step 4: Implement methods to set width and height
    fn width(&mut self, width: u32) -> &mut Self {
        self.width = Some(width);
        self
    }

    fn height(&mut self, height: u32) -> &mut Self {
        self.height = Some(height);
        self
    }

    // Step 5: Implement the build method
    fn build(&self) -> Result<Rectangle, &'static str> {
        if let (Some(width), Some(height)) = (self.width, self.height) {
            Ok(Rectangle { width, height })
        } else {
            Err("Width and height must be set")
        }
    }
}

fn main() {
    // Using the builder
    let rect = RectangleBuilder::new()
        .width(10)
        .height(20)
        .build();

    match rect {
        Ok(r) => println!("Rectangle: width = {}, height = {}", r.width, r.height),
        Err(e) => println!("Error: {}", e),
    }
}
*/
