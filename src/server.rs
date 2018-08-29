use std::time::Duration;
use std::path::Path;

use iron;
use iron::prelude::*;
use iron::{Timeouts};
use mount::Mount;
use staticfile::Static;
use iron_cors::CorsMiddleware;

pub struct Route {
	pub mount_path: String,
	pub relative_path: String,
}

pub struct SimbolServer {
	pub address: String,
	pub port: u16,
	pub path: String,
	pub routes: Vec<Route>,
}

impl SimbolServer {
	pub fn new(address: String, port: u16, path: String, routes: Vec<Route>) -> SimbolServer {
		SimbolServer {
			address: address,
			port: port,
			path: path,
			routes: routes,
		}
	}

	fn chain(&self) -> iron::Chain {
    	let mut asset_mount = Mount::new();

		asset_mount.mount("/", Static::new(Path::new(&self.path)));
		for route in &self.routes {
			let path = format!("{}{}", &self.path, &route.relative_path);
			let static_content = Static::new(Path::new(path.as_str()));
			asset_mount.mount(&route.mount_path, static_content);
		}

		iron::Chain::new(asset_mount)
	}

	pub fn run_server(&self) -> iron::Listening {
		let cors_middleware = CorsMiddleware::with_allow_any();
		let mut chain = self.chain();
		chain.link_after(::middleware::DefaultContentTypeMiddleware);
		chain.link_after(::middleware::Custom404Middleware);
		chain.link_around(cors_middleware);

		let mut iron = Iron::new(chain);

		iron.threads = 8;
		iron.timeouts = Timeouts {
			keep_alive: Some(Duration::from_secs(10)),
			read: Some(Duration::from_secs(10)),
			write: Some(Duration::from_secs(10))
		};
		iron.http(format!("{}:{}", self.address, self.port)).unwrap()
	}
}