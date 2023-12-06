mod option_enum;
mod pattern_matching;
mod if_let;
fn main() {
    // enums should be used in such a scenario where you dont need a class type/ struct type and have limited set of values that 
    // still needs typing example IpAddress(v4 and v6)
    // we can define the data type of each variant along with declaring enum inside tuple with datatype, and it supports multiple 
    #[derive(Debug)]
    enum IpAddress {
        // the attribute types inside enums are called variants
        V4(u8, u8, u8, u8, String),
        V6(String)
    }

    // we can created instances of different variants by using the scope on enum namespace
    let home = IpAddress::V4(127, 0, 0, 1, String::from("home"));
    let loopback = IpAddress::V6(String::from("::1"));
    // with this we have two instances of same enum type IpAddress, which can used to implement functionality on IpAddress type

    fn route(ip_type: IpAddress) {
        println!("{:?}", ip_type);
    }
    
    route(home);
    route(loopback);
    // there is enum type for ipaddr in std lib, which stores the enum variants data in a special struct types  
    // we can also have another enum type as variant type 
    println!("Hello, world!");


    enum Message {
        Quit,
        Move {x: i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32)
    }
    // defining enum like Message is similar to defining different structs but enums doesnt use struct keyword and all the variants are grouped in 
    // Message type, so enums help in grouping different kind of structs and can be used if all the structs grouped together and 
    // need to be passed around

    // similar to structs we can implement methods on enums as well
    impl Message {
        fn call(&self){
            // mehtod
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
    option_enum::option_ex();
    pattern_matching::match_pattern();
    if_let::iflet_ex();
}
