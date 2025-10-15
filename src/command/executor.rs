use std::collections::HashMap;

type CommandFn = fn(&[&str]) -> Result<String, String>;

pub struct CommandExecutor {
    commands: HashMap<String, CommandFn>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        let mut executor = Self {
            commands: HashMap::new(),
        };
        executor.register_default_commands();
        executor
    }

    fn register_default_commands(&mut self) {
        // Commands will be registered here
    }

    pub fn register_command(&mut self, name: &str, func: CommandFn) {
        self.commands.insert(name.to_string(), func);
    }

    pub fn execute(&self, name: &str, args: &[&str]) -> Result<String, String> {
        if let Some(cmd) = self.commands.get(name) {
            cmd(args)
        } else {
            Err(format!("Unknown command: {}", name))
        }
    }
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

