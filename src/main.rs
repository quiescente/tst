use chrono::{DateTime, Utc};
use std::{env, time::SystemTime, usize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let now = SystemTime::now();
    let mut fmt = false;

    if &args.len().wrapping_add_signed(-1) > &usize::MIN {
        if &args[1] == "--fmt" {
            fmt = true
        }
    }

    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            if fmt {
                let now: DateTime<Utc> = Utc::now();

                println!("{}", now.format("%Y%m%d%H%M%S%3f"))
            } else {
                println!("{}", n.as_millis())
            }
        }
        Err(err) => panic!("{:?}", err),
    }
}
