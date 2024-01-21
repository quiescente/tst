use std::time::SystemTime;

fn main() {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("{}", n.as_millis()),
        Err(err) => panic!("{:?}", err),
    }
}
