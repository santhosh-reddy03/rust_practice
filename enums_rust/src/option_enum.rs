// its one of the most used enum in rust, becuase of this its direclty include in prelude or all scopes by default
// and we can access its variants directly without using namespace scoping

pub fn option_ex() {
    let some_number = Some(5);
    let some_str = Some(String::from("something"));
    let absent_item: Option<i32> = None;
    // prints or extracts the value T
    println!("{}", some_number.expect("some value"));
    // prints or extracts the value T
    println!("{}", some_str.expect("some string"));
    // for none it creates code ended with panic and print the message
    println!("{}", absent_item.expect("this is none, code panics here"));
    // rust provides nulll safety through this implemenatation as we have to convert option(T) to type T to do any operation and compiler will force us to do so
    // rust forces you to decalre type for none value and make us handle the cases explicitly
    // match expression is control that does converting some(T) to T and None and execute code accordingly to variants
}