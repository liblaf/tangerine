use color_eyre::eyre;
use color_eyre::eyre::Result;

pub trait ExitStatusError: Sized {
    fn check(self, command: &tokio::process::Command) -> Result<Self>;
}

impl ExitStatusError for std::process::ExitStatus {
    fn check(self, command: &tokio::process::Command) -> Result<Self> {
        if self.success() {
            Ok(self)
        } else {
            Err(eyre::eyre!(
                "Command {:?} failed with status: {}",
                command,
                self
            ))
        }
    }
}
