#[derive(Default)]
pub struct CommandBuilder {
    executable: Option<String>,
    args: Vec<String>,
    current_dir: Option<String>,
}

#[derive(Debug)]
pub struct Command {
    executable: String, 
    args: Vec<String>,
    current_dir: String,
}

impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder::default()
    }
}

impl CommandBuilder {
    pub fn executable(mut self, exec: String) -> Self {
        self.executable = Some(exec);
        self
    }
    pub fn args(mut self, arg: String) -> Self {
        self.args.push(arg);
        self
    }
    pub fn current_dir(mut self, dir: String) -> Self {
        self.current_dir = Some(dir);
        self
    }

    pub fn build(self) -> Result<Command, ()> {
        let executable = self.executable.ok_or(())?;
        let current_dir = self.current_dir.ok_or(())?;
        Ok(Command { executable: executable, args: self.args, current_dir: current_dir })
    }
}

fn main() {
    // let cb = CommandBuilder::default();
    // println!("hello~");

    let command = Command::builder()
                .executable("cargo".to_owned())
                .args("--release".to_owned())
                .current_dir("./".to_owned())
                .build()
                .unwrap();

    println!("{:?}", command);
}
