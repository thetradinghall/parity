// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;
use std::net::SocketAddr;
use io::PanicHandler;
use rpc_apis;
use helpers::replace_home;

#[cfg(feature = "dapps")]
pub use ethcore_dapps::Server as WebappServer;
#[cfg(not(feature = "dapps"))]
pub struct WebappServer;

#[derive(Debug, PartialEq, Clone)]
pub struct Configuration {
	pub enabled: bool,
	pub interface: String,
	pub port: u16,
	pub user: Option<String>,
	pub pass: Option<String>,
	pub dapps_path: String,
}

impl Default for Configuration {
	fn default() -> Self {
		Configuration {
			enabled: true,
			interface: "127.0.0.1".into(),
			port: 8080,
			user: None,
			pass: None,
			dapps_path: replace_home("$HOME/.parity/dapps"),
		}
	}
}

pub struct Dependencies {
	pub panic_handler: Arc<PanicHandler>,
	pub apis: Arc<rpc_apis::Dependencies>,
}

pub fn new(configuration: Configuration, deps: Dependencies) -> Result<Option<WebappServer>, String> {
	if !configuration.enabled {
		return Ok(None);
	}

	let url = format!("{}:{}", configuration.interface, configuration.port);
	let addr = try!(url.parse().map_err(|_| format!("Invalid Webapps listen host/port given: {}", url)));

	let auth = configuration.user.as_ref().map(|username| {
		let password = configuration.pass.as_ref().map_or_else(|| {
			use rpassword::read_password;
			println!("Type password for WebApps server (user: {}): ", username);
			let pass = read_password().unwrap();
			println!("OK, got it. Starting server...");
			pass
		}, |pass| pass.to_owned());
		(username.to_owned(), password)
	});

	Ok(Some(try!(setup_dapps_server(deps, configuration.dapps_path, &addr, auth))))
}

#[cfg(not(feature = "dapps"))]
pub fn setup_dapps_server(
	_deps: Dependencies,
	_dapps_path: String,
	_url: &SocketAddr,
	_auth: Option<(String, String)>,
) -> Result<WebappServer, String> {
	Err("Your Parity version has been compiled without WebApps support.".into())
}

#[cfg(feature = "dapps")]
pub fn setup_dapps_server(
	deps: Dependencies,
	dapps_path: String,
	url: &SocketAddr,
	auth: Option<(String, String)>
) -> Result<WebappServer, String> {
	use ethcore_dapps as dapps;

	let server = dapps::ServerBuilder::new(dapps_path);
	let server = rpc_apis::setup_rpc(server, deps.apis.clone(), rpc_apis::ApiSet::UnsafeContext);
	let start_result = match auth {
		None => {
			server.start_unsecure_http(url)
		},
		Some((username, password)) => {
			server.start_basic_auth_http(url, &username, &password)
		},
	};

	match start_result {
		Err(dapps::ServerError::IoError(err)) => Err(format!("WebApps io error: {}", err)),
		Err(e) => Err(format!("WebApps error: {:?}", e)),
		Ok(server) => {
			server.set_panic_handler(move || {
				deps.panic_handler.notify_all("Panic in WebApp thread.".to_owned());
			});
			Ok(server)
		},
	}
}

