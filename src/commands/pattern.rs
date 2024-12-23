use std::fmt::Write;

use crate::program::{PatternCli, Sfce};

impl Sfce {
    pub fn pattern_command(&mut self, l: PatternCli) -> anyhow::Result<()> {
        match l {
            PatternCli::Expand { pattern } => {
                // println!("{:?}", pat);
                let list = pattern.contents();
                let mut i = 0;
                for q in list {
                    if i % self.program.args.pw == 0 {
                        writeln!(self.buf)?;
                    }
                    write!(self.buf, "{q} ")?;
                    i += 1;
                }

                writeln!(self.buf)?;

                writeln!(self.buf, "{i} queues total")?;
            }

            PatternCli::Hold { pattern } => {
                // println!("{:?}", pat);
                let list = pattern.contents();
                for q in list {
                    write!(self.buf, "\x1b[1m{q}\x1b[0m")?;

                    for (i, hq) in q.hold_queues().into_iter().enumerate() {
                        if i % self.program.args.pw == 0 {
                            writeln!(self.buf)?;
                        }
                        write!(self.buf, "{hq} ")?;
                    }

                    writeln!(self.buf, "\n")?;
                }
            }
        }

        Ok(())
    }
}
