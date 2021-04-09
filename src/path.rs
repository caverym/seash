use std::env;

pub struct Path {
    paths: Vec<String>
}

impl Path {
    pub fn new() -> Self {
        // Read `PATH`, sets to `/bin` if can't read.
        let all = match env::var("PATH") {
            Ok(a) => a,
            Err(e) => {eprintln!("Warning: {}", e); "/bin".into()}
        };

        // Split `PATH` into a vector.
        let v_str: Vec<&str> = all.split(':').collect();
        // Convert the vector in a usable form.
        let v: Vec<String> = v_str.iter().map(|x| x.to_string()).collect();

        Self { paths: v }
    }
}
