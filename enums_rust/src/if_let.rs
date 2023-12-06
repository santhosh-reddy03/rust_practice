

pub fn iflet_ex(){
    // if let syntax can be used in a situatuon where we execute the match pattern matching code for only a particular match
    // and rest goes into default
    // example as below
    

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // we can do this for the above code (syntactic sugar)
    if let Some(3) = some_u8_value {
        println!("three");
    }else {
        println!("else block");
    }

    // to execute the default pattern we can use else block in if let 
    // we can also do this for patterns that have variables in their enum varaints else we can count the number of coins
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("this is quarter coin from state {:?}", state);
    // } else {
    //     count += 1
    // }

    // the above code is similar to 
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

}