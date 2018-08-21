# Simbol server

A simple HTTP and WebSocket server built on top of [Iron]() and [ws](). Easily get started with a Simbol app

## Quick start

In the root directory of you web project, create a server directory for the [Rust]() server:

```bash
mkdir server
mkdir server/src
cd server
```

Create a `Cargo.toml` and add the dependency to it:

```toml
[dependencies]
simbol-server = "1.0.0"
```

Create a `main.rs` file in the `server/src` directory with the following content:

```rust
extern crate simbol;

use std::thread;
use simbol::server::*;
use simbol::multivp::*;

fn main() {
    let mut routes: Vec<Route> = Vec::new();
    routes.push(Route {
            id: String::from("build"), // How to identify this route
            url_path: String::from("/build/*"), // What part of the URL will this route be used for
            mount_path: String::from("/build/"), // Mount path
            relative_path: String::from("build"), // The actual relative path to the content from your project's root directory
    });
    routes.push(Route {
        id: String::from("assets"),
        url_path: String::from("/assets/*"),
        prefix: String::from("/assets/"),
        relative_path: String::from("assets"),
    });
    let server = SimbolServer::new(String::from("../"), 3000, routes);
    let http_handle = thread::spawn(move || {
        server.run_server();
    });

    let multivp_server = MultiVP::new(String::from("localhost"), 8091);
    let ws_handle = thread::spawn(move || {
        multivp_server.run_server();
    });

    http_handle.join().unwrap();
    ws_handle.join().unwrap();
}
```

This will create and run an HTTP server with the default roots to load:

- `index.html` from `/index.html`
- The different files in `/*`
- Your assets, such as GLTF files, from `/assets/*`
- Your built files, such as your JS and CSS, from `/build/*`

It will also create and run a WebSocket server for the multiVP (social) component of Simbol

Then run it:

`cargo run`

## License

This program is free software and is distributed under an [MIT License](https://github.com/wearesimbol/simbol-server/blob/master/LICENSE).