
pub fn match_pattern() {
    #[derive(Debug)]
    enum UsState {
        California,
        Norwalk,
        WashingtonDC
        // etc
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }

    impl Coin {
        fn value_in_cents(&self) -> u32{
            // match tries the match against patterns that are present in arms 
            // each arm will have a pattern and code which will be executed if the pattern is matched
            match self {
                Coin::Penny => {
                    println!("inside matching arm");
                    1
                },
                Coin::Nickel => 5,
                Coin::Dime => 10,
                // the use of variable in pattern will bind the values of the state of data inside enum 
                // then we can use the data inside code 
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}", state);
                    25
                },
            }
        }
    }
    
    let coin = Coin::Penny;
    let quarter = Coin::Quarter(UsState::California);
    println!("{}", quarter.value_in_cents());
    println!("cents in a Penny = {}", coin.value_in_cents());


    // rust is exhaustive when it comes to pattern matching and checks for any patterns that not been implemented and gives run time errors
    // since its exhaustive it forces us to handle every possible value, but to avoid it we can use '_' special pattern as default



    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

}