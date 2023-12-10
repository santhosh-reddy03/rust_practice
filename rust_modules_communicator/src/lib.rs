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
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
