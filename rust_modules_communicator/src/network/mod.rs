
// mod.rs feels like python __init__.py file to treat entire folder as single module or package
pub fn connect(){

}

// we cannot declare a sub module like main module into seperate files directly
//  to support this we have make a seperate directory for network module and then declare server.rs file inside it and rename network.rs to mod
// this is because rust cant identify root mods as sub module of other module
// below is great example why the above is the way to implement sub module
// now we have to have both mod client and sub module client in src folder which would not be possible and we have to differentiate 
// which code belong to which block incase if we use single client.rs file
//  ├── client
//  └── network
//      └── client

pub mod server;
// mod server{
//     fn connect(){

//     }
// }