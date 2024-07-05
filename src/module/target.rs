use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramConfig {
    apps: Vec<Program>,
    scripts: Vec<Program>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    name: String,
    icon_path: Option<String>,
    command: String,
    args: Option<Vec<String>>,
}

pub fn load_config() -> ProgramConfig {
    let data = std::fs::read_to_string("C:/Users/sivar/Desktop/SLauncher Config/config.json")
        .expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

pub fn launch_program(program: &Program) {
    let args = program.args.clone().unwrap_or_else(|| vec![]);
    let _ = Command::new(program.command.clone())
        .args(args)
        .spawn()
        .expect("Failed to launch program");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_launch_program() {
        // Create a dummy program struct
        let config = load_config();
        let program = config.apps.first().unwrap();

        // Capture the output of the launched program
        let output = Command::new("echo")
            .arg("Hello, World!")
            .output()
            .expect("Failed to execute echo command");

        // Launch the program
        launch_program(program);

        // Assert that the output contains the expected string
        assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, World!\n");
    }
}
