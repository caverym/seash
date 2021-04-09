pub struct Command {
    pub executable: String,
    pub arguments: Vec<String>,
    // args: bool,
}

impl Command {
    pub fn new(input: String) -> Self {
        let mut _args = true;

        // Split input into a vector.
        let mut arguments: Vec<&str> = input.split(' ').collect();

        // Remove any empty elements from the vector.
        arguments.retain(|x| *x != "");

        // Check if there are any arguments to pass to the command.
        if arguments.len() < 2 {
            _args = false;
        }

        // Get the command.
        let executable: &str = arguments.remove(0);

        Self {
            executable: executable.into(),
            arguments: arguments.iter().map(|x| x.to_string()).collect(),
            // args
        }
    }
}
