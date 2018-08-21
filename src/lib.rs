extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate iron_cors;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate ws;

mod middleware;
pub mod server;
pub mod multivp;