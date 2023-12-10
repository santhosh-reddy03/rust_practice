extern crate communicator;
// we should always import external crate in root mod or at top level, so that our submodules can refer to items from 
// external crates as top-level items/modules.

fn main() {
    println!("Hello, world!");
    // rust modules helps us reuse the code in a organised fashion as the project grows big in size
    // a module is a namespace that contains definitions of functions and types like structs and enums
    // and we can choose whether a definition is public or private inside a module for the availability outside module
    // mod keyword declares the module and code immediately after the braces belong to it
    // by default all items are pvt except the ones that are explicitly mentioned to be public
    // we can bring the other modules into the current namespace using 'use' keyword
    // we can also create moduels inside modules to seperate the functionality 
    communicator::client::connect();

    // by default everything is pvt in rust, If you don’t use a private function within your program, 
    // because your program is the only code allowed to use that function, 
    // Rust will warn you that the function has gone unused.
    // we have to make them pub to be used in other parts or by external users 

    mod outermost {
        pub fn middle_function() {}
    
        fn middle_secret_function() {}
    
        pub mod inside {
            pub fn inner_function() {
                // the two colons in the beginning says that we want to refer modules from the root module
                // ::outermost::middle_secret_function()
            }
    
            fn secret_function() {}
        }
    }

    fn try_me() {
        outermost::middle_function();
        // outermost::middle_secret_function();
        outermost::inside::inner_function();
        // outermost::inside::secret_function();
    }
}





