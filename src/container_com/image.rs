use std::collections::HashMap;
use std::process::{Command, ExitStatus, Output};
use serde_json::{json, Value};

use super::{CommandStruct, ContainerCommand};

pub struct DockerListAllImages {

}

impl ContainerCommand for DockerListAllImages {
    fn execute(args: Vec<String>) -> Result<CommandStruct, String> {
        let command_name = String::from("docker");
        let command_args = vec![String::from("image"), String::from("ls"), String::from("--all"), String::from("--format"), String::from("json")];
        let mut command_struct = CommandStruct::new(command_name, command_args);
        let command_result = Self::execute_os_command(&command_struct)?;
        if !command_result.status.success() {
            return Err(String::from_utf8(command_result.stderr).unwrap());
        }
        let stdout_string = String::from_utf8(command_result.stdout).unwrap();
        let stdout_lines = stdout_string.lines();
        command_struct.is_ok = command_result.status.success();
        command_struct.exit_code = command_result.status.code().unwrap();
        command_struct.output.clear();
        for line in stdout_lines {
            let x = serde_json::from_str(line);
            match x {
                Ok(serde_map) => command_struct.output.push(serde_map),
                Err(e) => {
                    println!("Error: {}", e);
                    command_struct.output = vec![HashMap::from([(String::from("ran"), Value::Bool(true))])];
                }
            }
        }
        Ok(command_struct)
    }
}