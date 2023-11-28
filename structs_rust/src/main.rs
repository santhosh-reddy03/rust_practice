
#[derive(Debug)]
struct Rectangle{
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    // we can also define functions that dont take self param, they are called associated functions
    // these are not methods and are functions since they dont take self param
    // associated functions are often used to create new instances like constructors
    fn square(size: u32) -> Rectangle {
        Rectangle{ length:size, width:size}
    }
}

fn area(rect : &Rectangle) -> u32 {
    rect.length * rect.width
}

fn main() {
    let rect = Rectangle{
        length: 10,
        width: 15
    };
    println!("{}", area(&rect));
    // rust has its reference and dereference operatoion that works when we call methods on reference or a pointer in c++
    // rust can also figure if the method need mutable or immutable self reference or taking ownership
    println!("using method {}", &rect.area());
    // printing the rect using the debugger :? is debug trait(which has its own std::fmt::Display trait)
    println!("rect {:?}", rect);
    // we use :: (namespaced) by struct  and is used to call the associated functions
    let sq = Rectangle::square(5);
    println!("square {:?}", sq);

}
