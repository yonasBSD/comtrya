use crate::atoms::Outcome;

use super::super::Atom;
//use crate::utilities;
use anyhow::anyhow;
use tracing::debug;

use english_to_cron::str_cron_syntax;

#[derive(Default)]
pub struct Remove {
    pub name: Option<String>,
    pub schedule: String,
    pub user: Option<String>,
    pub privileged: Option<bool>,
}

#[allow(dead_code)]
/*
pub fn new_run_command(command: String) -> Remove {
    Remove {
        command,
        ..Default::default()
    }
}
*/

impl Remove {
    fn elevate_if_required(&self) -> (String, Vec<String>) {
        (String::from(""), vec![String::from("")])
        /*
        // Depending on the priviledged flag and who who the current user is
        // we can determine if we need to prepend sudo to the command
        match (self.privileged, whoami::username().as_str()) {
            // Hasn't requested priviledged, so never try to elevate
            (false, _) => (self.command.clone(), self.arguments.clone()),

            // Requested priviledged, but is already root
            (true, "root") => (self.command.clone(), self.arguments.clone()),

            // Requested priviledged, but is not root
            (true, _) => (
                String::from("sudo"),
                [vec![self.command.clone()], self.arguments.clone()].concat(),
            ),
        }
        */
    }

    fn elevate(&mut self) -> anyhow::Result<()> {
        Ok(())
        /*
        tracing::info!(
            "Sudo required for privilege elevation to run `{} {}`. Validating sudo ...",
            &self.command,
            &self.arguments.join(" ")
        );

        match std::process::Command::new("sudo")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg("--validate")
            .output()
        {
            Ok(std::process::Output { status, .. }) if status.success() => Ok(()),

            Ok(std::process::Output { stderr, .. }) => Err(anyhow!(
                "Command requires sudo, but couldn't elevate privileges: {}",
                String::from_utf8(stderr)?
            )),

            Err(err) => Err(anyhow!(
                "Command requires sudo, but couldn't elevate privileges: {}",
                err
            )),
        }
        */
    }
}

impl std::fmt::Display for Remove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CommandRemove with: privileged={:#?}: {:#?} {}",
            self.privileged, self.name, self.schedule
        )
    }
}

impl Atom for Remove {
    fn plan(&self) -> anyhow::Result<Outcome> {
        Ok(Outcome {
            // Commands may have side-effects, but none that can be "known"
            // without some sandboxed operations to detect filesystem and network
            // affects.
            // Maybe we'll look into this one day?
            side_effects: vec![],
            // Commands should always run, we have no cache-key based
            // determinism atm the moment.
            should_run: true,
        })
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        let _schedule = match str_cron_syntax(&self.schedule) {
            Ok(schedule) => {
                debug!("{}", schedule);
                schedule
            },
            Err(err) => {
                return Err(anyhow!(
                        "Invalid schedule {}",
                        err
                ))
            }
        };

        Ok(())
        /*
        let (command, arguments) = self.elevate_if_required();

        let command = utilities::get_binary_path(&command)
            .or_else(|_| Err(anyhow!("Command `{}` not found in path", command)))?;

        // If we require root, we need to use sudo with inherited IO
        // to ensure the user can respond if prompted for a password
        if command.eq("sudo") {
            match self.elevate() {
                Ok(_) => (),
                Err(err) => {
                    return Err(anyhow!(err));
                }
            }
        }

        match std::process::Command::new(&command)
            .envs(self.environment.clone())
            .args(&arguments)
            .current_dir(&self.working_dir.clone().unwrap_or_else(|| {
                std::env::current_dir()
                    .map(|current_dir| current_dir.display().to_string())
                    .expect("Failed to get current directory")
            }))
            .output()
        {
            Ok(output) if output.status.success() => {
                self.status.stdout = String::from_utf8(output.stdout)?;
                self.status.stderr = String::from_utf8(output.stderr)?;

                debug!("stdout: {}", &self.status.stdout);

                Ok(())
            }

            Ok(output) => {
                self.status.stdout = String::from_utf8(output.stdout)?;
                self.status.stderr = String::from_utf8(output.stderr)?;

                debug!("exit code: {}", &self.status.code);
                debug!("stdout: {}", &self.status.stdout);
                debug!("stderr: {}", &self.status.stderr);

                Err(anyhow!(
                    "Command failed with exit code: {}",
                    output.status.code().unwrap_or(1)
                ))
            }

            Err(err) => Err(anyhow!(err)),
        }
        */
    }

    /*
    fn output_string(&self) -> String {
        self.status.stdout.clone()
    }

    fn error_message(&self) -> String {
        self.status.stderr.clone()
    }
    */
}

#[cfg(test)]
mod tests {
    //use super::*;
    //use pretty_assertions::assert_eq;

    #[test]
    fn defaults() {
        /*
        let command_run = Remove {
            ..Default::default()
        };

        assert_eq!(String::from(""), command_run.command);
        assert_eq!(0, command_run.arguments.len());
        assert_eq!(None, command_run.working_dir);
        assert_eq!(0, command_run.environment.len());
        assert_eq!(false, command_run.privileged);

        let command_run = new_run_command(String::from("echo"));

        assert_eq!(String::from("echo"), command_run.command);
        assert_eq!(0, command_run.arguments.len());
        assert_eq!(None, command_run.working_dir);
        assert_eq!(0, command_run.environment.len());
        assert_eq!(false, command_run.privileged);
        */
    }

    #[test]
    fn elevate() {
        /*
        let mut command_run = new_run_command(String::from("echo"));
        command_run.arguments = vec![String::from("Hello, world!")];
        let (command, args) = command_run.elevate_if_required();

        assert_eq!(String::from("echo"), command);
        assert_eq!(vec![String::from("Hello, world!")], args);

        let mut command_run = new_run_command(String::from("echo"));
        command_run.arguments = vec![String::from("Hello, world!")];
        command_run.privileged = true;
        let (command, args) = command_run.elevate_if_required();

        assert_eq!(String::from("sudo"), command);
        assert_eq!(
            vec![String::from("echo"), String::from("Hello, world!")],
            args
        );
        */
    }

    #[test]
    fn error_propagation() {
        /*
        let mut command_run = new_run_command(String::from("non-existant-command"));
        command_run.execute().expect_err("Command should fail");
        */
    }
}
