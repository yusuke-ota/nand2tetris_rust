use crate::{Optimizer, CodeWriter};
use std::mem::swap;
use parser::anyhow;

impl Optimizer for CodeWriter{
    /// run optimize functions.
    fn optimize(&mut self) -> anyhow::Result<()> {
        self.remove_sp_minus_sp_plus()?;

        Ok(())
    }

    fn remove_sp_minus_sp_plus(&mut self) -> anyhow::Result<()> {
        let mut write_buffer = Vec::<u8>::new();
        swap(&mut self.write_buffer, &mut write_buffer);

        let mut trim_string = String::from_utf8(write_buffer)?
            .split("@SP\nM=M+1\n@SP\nAM=M-1\n")
            .collect::<String>();

        swap(&mut self.write_buffer, &mut unsafe { trim_string.as_mut_vec().clone() });
        Ok(())
    }
}
