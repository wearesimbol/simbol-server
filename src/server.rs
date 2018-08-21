use std::time::Duration;
use std::path::Path;

use iron;
use iron::prelude::*;
use iron::{Timeouts};
use router::{Router};
use mount::Mount;
use staticfile::Static;
use iron_cors::CorsMiddleware;

pub struct Route {
	pub mount_path: String,
	pub relative_path: String,
}

pub struct SimbolServer {
	pub path: String,
	pub port: u16,
	pub routes: Vec<Route>,
}

impl SimbolServer {
	pub fn new(path: String, port: u16, routes: Vec<Route>) -> SimbolServer {
		SimbolServer {
			path: path,
			port: port,
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
		iron.http(format!("localhost:{}", self.port)).unwrap()
	}
}