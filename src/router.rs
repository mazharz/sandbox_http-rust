use std::{error::Error, net::TcpStream};

use crate::constants::HttpMethod;

pub struct Route {
    method: HttpMethod,
    path: String,
    pub callback: Box<dyn Fn(&TcpStream) -> ()>,
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: vec![] }
    }

    pub fn add_route(
        &mut self,
        method: HttpMethod,
        path: &str,
        callback: Box<dyn Fn(&TcpStream) -> ()>,
    ) -> Result<(), Box<dyn Error>> {
        let existing = self.get_route(method.clone(), path);
        match existing {
            Some(_) => Err("This route exits".into()),
            None => {
                self._add_route(method, path, callback);
                return Ok(());
            }
        }
    }

    fn _add_route(
        &mut self,
        method: HttpMethod,
        path: &str,
        callback: Box<dyn Fn(&TcpStream) -> ()>,
    ) {
        self.routes.push(Route {
            method,
            path: path.to_string(),
            callback,
        });
    }

    pub fn get_route(&mut self, method: HttpMethod, path: &str) -> Option<&Route> {
        self.routes
            .iter()
            .find(|&x| x.method == method && x.path == path)
    }
}
