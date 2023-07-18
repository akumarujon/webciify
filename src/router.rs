// Importing the required libraries
use std::io::Write;
use std::net::TcpStream;

// Defining a public struct `Router` with a field called `routes` which is a vector of `Route` struct
pub struct Router {
    routes: Vec<Route>,
}

// Defining a public struct `Route` with fields method, path and handler
pub struct Route {
    method: String,
    path: String,
    handler: fn(&mut TcpStream, &str),
}

// Implementing the Router struct
impl Router {
    // Defining a new function that returns a new instance of the Router struct
    pub fn new() -> Self {
        Router { routes: vec![] }
    }

    // Defining a function to add a new route to the Router struct
    pub fn add_route(&mut self, method: &str, path: &str, handler: fn(&mut TcpStream, &str)) {
        // Create a new instance of the Route struct with the passed method, path and handler parameters
        let route = Route {
            method: method.to_string(),
            path: path.to_string(),
            handler,
        };

        // Push the new route to the `routes` vector in the Router struct
        self.routes.push(route);
    }

    // Defining a function to handle requests to the server
    pub fn handle_request(&self, stream: &mut TcpStream, method: &str, path: &str) {
        // Iterate through all routes in the `routes` vector in the Router struct
        for route in &self.routes {
            // Check if the method and path of the route matches the method and path of the request
            if route.method == method && route.path == path {
                // If the route matches, call the handler function with the stream and path as parameters
                (route.handler)(stream, path);
                return;
            }
        }

        // If no route matches the request, send a 404 response
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
