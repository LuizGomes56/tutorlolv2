use crate::{Tracker, scripts::utils::FmtArgs};
use std::{collections::HashMap, ops::Range};
use tutorlolv2_dev::MayFail;

impl Tracker<'_> {
    pub fn batch(&mut self, fmt: String, fmt_args: &mut HashMap<&str, FmtArgs>) -> MayFail {
        let input = tutorlolv2_fmt::rustfmt(&fmt, None);
        let offset = self.offset();

        // self.record("?");
        // tutorlolv2_dev::write(format!("__build_ir_{offset}.txt"), &input)?;

        Ok(())
    }
}
