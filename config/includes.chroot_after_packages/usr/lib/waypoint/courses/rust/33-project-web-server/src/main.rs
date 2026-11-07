use std::net::TcpListener;

use web_server::serve;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Couldn't start the server on port 7878: {error}");
            eprintln!("Something else may already be using it.");
            return;
        }
    };
    println!("Your server is running. Visit http://localhost:7878 in your browser.");
    println!("Press Ctrl+C here to stop it.");
    serve(listener);
}
