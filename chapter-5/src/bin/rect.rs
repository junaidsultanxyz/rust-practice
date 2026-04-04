#[derive(Debug)] // add outer attribute of Debug
struct Rectangle {
    width: u32,
    height: u32
}

// everything inside this block will be associated to Rectangle
impl Rectangle {
    /*
        first parameter is always the self. &self is short for self: &Self.
        this method will be called on the instance of Rectangle.
        methods must have the self parameter in the first spot.
        we borrowed the self with &self becase we just wanted to read the data.
        methods can take ownership of self.
    */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // we can also make methods with same name as struct fields.
    fn width(&self) -> bool {
        self.width > 0
    }

    /*
        Rust has automatic referencing and dereferencing.
        we dont have to (&rect1).area(). instead we can just do rect1.area().
        Rust will add the &, &mut or * or anything that is needed to match the
        signature of method.
    */


    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // working like a constructor.
    // function namespaced by struct.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

// we can write multiple impl blocks
impl Rectangle {
    fn print_rect(&self) {
        println!("{self:?}");
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 10,
        height: 20
    };

    let rect2 = Rectangle {
        width: 5,
        height: 10
    };

    let sqr = Rectangle::square(10);

    println!("rect1 is {rect1:?}"); // print the struct with simple formatting
    println!("rect1 is {rect1:#?}"); // print the struct with same better formatting
    /*
        :?  ->  Rectangle { width: 10, height: 20 }
        :#? ->  Rectanglle {
                    width: 10,
                    height: 20,
                }
    */

    /*
        by defauult, the {} inside println! uses formatting Display.
        structs dont have Display.
        So to print structs, we can use different specifier.
        :? uses Debug output format.
        But to actually use it, the struct must have the Debug trait.
    */


    println!("area of {rect1:?} = {}", rect1.area());
    println!("{}", rect1.width());
    println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("area of rect1: {}", Rectangle::area(&rect1));
    println!("area of square {}", sqr.area());
    sqr.print_rect();
}