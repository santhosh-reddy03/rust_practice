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


    // so far we have been using fully qualified name when referring to a function in submodules even its deeply nested
    // instead we can make use of 'use' key word
    of::nested_modules();
    // use bring what we have specified into our scope, so we still need to refer the sub things inside the namespace ex of::nested_modules()
    // we can also use 'use' on enums to bring enum variants into scope, if we are bringing more than one item from a scope into namespace
    // we should use curly braces ex as show below for TrafficLight
    // we can bring all together by using * from a namespace, and we should be careful while using * as it will bring all into our namespace
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}




enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

