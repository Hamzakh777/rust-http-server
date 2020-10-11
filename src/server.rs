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
    }
}
