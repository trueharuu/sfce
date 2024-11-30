use std::{fmt::Write, io::Write as iW};

use crate::{
    board_parser::Tetfu,
    commands::{
        fumen::fumen_command, movec::move_command, pattern::pattern_command, test::test_command,
    },
    grid::Grid,
    text::Text,
    pattern::Pattern, ranged::Ranged,
};

#[derive(clap::Parser, Clone, Debug)]
#[clap(disable_help_flag = true)]
pub struct Sfce {
    #[command(subcommand)]
    sub: SfceCommand,
    #[clap(flatten)]
    pub args: Options,
    #[clap(skip)]
    pub buf: String,
}

#[derive(clap::Args, Clone, Debug)]
pub struct Options {
    #[arg(short = 'o')]
    pub output: Option<String>,
    #[arg(short = 'l')]
    pub link_type: Option<char>,
    #[arg(short = 'w')]
    pub board_width: Option<usize>,
    #[arg(short = 'h')]
    pub board_height: Option<usize>,
    #[arg(short = '!')]
    pub open_browser: bool,
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum SfceCommand {
    #[command(subcommand)]
    Fumen(FumenCli),
    #[command(subcommand)]
    Pattern(PatternCli),
    Move {
        #[arg(short = 't', long = "tetfu")]
        tetfu: Text<Tetfu>,
        #[arg(short = 'p', long = "pattern")]
        pattern: Text<Pattern>,
        #[arg(short = 'c', default_value = "0")]
        clears: Ranged<usize>,
    },
    Test,
    Grid {
        #[arg(short = 't')]
        tetfu: Text<Tetfu>,
    },
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum FumenCli {
    #[command(name = "encode")]
    Encode {
        #[arg(short = 't', long = "grid")]
        grid: Text<String>,
    },

    #[command(name = "decode")]
    Decode {
        #[arg(short = 't', long = "fumen")]
        fumen: Text<String>,
    },

    #[command(name = "glue")]
    Glue {
        #[arg(short = 't', long = "fumen")]
        fumen: Text<String>,
    },
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum PatternCli {
    #[command(name = "expand")]
    Expand {
        #[arg(short = 'p', long = "pattern")]
        pattern: Text<Pattern>,
    },
}

impl Sfce {
    pub fn run(&mut self) -> anyhow::Result<()> {
        // dbg!(&self);
        match self.sub.clone() {
            SfceCommand::Fumen(l) => fumen_command(self, l)?,
            SfceCommand::Pattern(l) => pattern_command(self, l)?,
            SfceCommand::Move { tetfu, pattern, clears } => {
                move_command(self, tetfu.contents(), pattern.contents(), clears)?
            }
            SfceCommand::Grid { tetfu } => write!(self.buf, "{}", tetfu.grid())?,
            SfceCommand::Test => test_command(self)?,
        }

        if let Some(s) = &self.args.output {
            println!("wrote {} bytes to path", self.buf.as_bytes().len());
            std::fs::write(s, self.buf.clone())?;
        } else {
            write!(std::io::stdout(), "{}", self.buf)?;
        }

        if self.args.open_browser {
            open::that(&self.buf);
        }

        Ok(())
    }

    pub fn tetfu(&self, f: Grid) -> String {
        if let Some(t) = self.args.link_type {
            if t == 'q' {
                format!(
                    "https://qv.rqft.workers.dev/tools/board-editor?{}",
                    f.fumen().encode()
                )
            } else {
                format!("https://harddrop.com/fumen?{t}{}", &f.fumen().encode()[1..])
            }
        } else {
            f.to_string()
        }
    }

    pub fn resize(&self, mut f: Grid) -> Grid {
        if let Some(w) = self.args.board_width {
            f.set_width(w);
        }

        if let Some(h) = self.args.board_height {
            f.set_height(h);
        }

        f
    }
}
