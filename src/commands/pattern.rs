use std::fmt::Write;

use crate::{
    program::{PatternCli,Sfce},
    traits::CollectVec,
};

pub fn pattern_command(f: &mut Sfce, l: PatternCli) -> anyhow::Result<()> {
    match l {
        PatternCli::Expand { pattern } => {
            // println!("{:?}", pat);
            let list = pattern.contents().queues();
            for l in &list {
                writeln!(f.buf, "{}", l.iter().map(|x| x.to_string()).vec().join(""))?
            }

            writeln!(f.buf, "{} queues total", list.len())?;
        }
    }

    Ok(())
}
