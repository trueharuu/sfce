use std::fmt::Write;

use crate::{
    program::{PatternCli, Sfce},
    traits::CollectVec,
};

impl Sfce {
    pub fn pattern_command(&mut self, l: PatternCli) -> anyhow::Result<()> {
        match l {
            PatternCli::Expand { pattern } => {
                // println!("{:?}", pat);
                let list = pattern.contents().queues();
                for l in &list {
                    writeln!(
                        self.buf,
                        "{}",
                        l.iter()
                            .map(std::string::ToString::to_string)
                            .vec()
                            .join("")
                    )?;
                }

                writeln!(self.buf, "{} queues total", list.len())?;
            }
        }

        Ok(())
    }
}
