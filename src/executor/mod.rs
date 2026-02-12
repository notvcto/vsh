mod builtin;

use crate::error::Result;
use crate::parser::Command;

/// Execute a parsed command
pub fn execute_command(cmd: Command) -> Result<()> {
    builtin::execute_builtin(cmd)
}
