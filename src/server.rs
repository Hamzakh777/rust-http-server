use std::io::read;
use std::net::TcpListener;

// Server is a struct:
// structs in Rust are like object in Js, but it holds just props, no  methods
pub struct Server {
    addr: String,
}

// implementation is how we add methods/functions to a struct.
// Associated function don't need an instance of the struct
impl Server {
    // associated function
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // method
    // self points to the instance of the struct - self is much like this
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        // if we fail to bind we want this to be an unrecoverable error
        // hense we use unwrap
        // unwrap will terminate the programe if the result is an error
        let listener = TcpListener::bind(&self.addr).unwrap();

        // loop is an infinite loop like doing while true
        loop {
            // match is like a switch statement
            match listener.accept() {
                Ok((stream, _)) => {
                    stream.read();
                }
                Err(error) => println!("Failed to establish a connection: {}", error),
            }
        }
    }
}
