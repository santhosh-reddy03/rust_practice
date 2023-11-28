// we can compile rust program using rustc file.rs
// it generates a executable that can be run on any computer without any additional requirements

// cargo is rust package manager and comes with lot of features such as creating new project, build and compile the project as well as run the project
// cargo new project_name --bin (is used to create a new project that creates a basic folder structure with Cargo.toml)
// --bin is used to create executable instead of library/package
// cargo build, cargo run, cargo check(is used to check compilation errors before hand)
// cargo build --release 
// above command generates compiled output with the most optimised version
fn main() {
    println!("Hello, world!");
}

