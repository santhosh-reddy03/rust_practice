// for rust libraries the lib.rs serves as entry point for library
pub mod client;
pub mod network;
// mod network {
//     // here thw two funcs named connect wont collide because they are in different module namespaces
//     fn connect(){
//     }

//     mod server{
//         fn connect(){

//         }
//     }
// }

// mod client {
//     fn connect() {

//     }
// }
// we can replace the above client mod as mod client; at top of file, that will tell rust to look for code in some other file called client.rs
// the effective change
// mod client {
//     client.rs
// }
// when we add more functionaility the funcs and mods might grow in size and we can use them as seperate files depending on our requirements



// pvt vs pub
// If an item is public, it can be accessed through any of its parent modules.
// If an item is private, it can be accessed only by its immediate parent module and any of the parent’s child modules.


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    // use super::*;
    #[test]
    fn check_client_connect() {
        // if we use client::connect() it fails becuase we are not referring communicator scope before client as we are inside communicator
        // we cant use 'use' because the paths are relative and we need client inside tests module
        // we can overcome this by using super infront of client which tells rust to move one module up from the current one 
        // we can also use this ::client::connect() to check from root of the module, but it seems deprecated
        super::client::connect();

        // instead of specifying the super for every usage we instead use super::import_required_mod at the paretns(tests) top instead of root(lib.rs) 
    }
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
