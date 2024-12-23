use std::{collections::HashSet, fmt::Write, io::Write as iW, time::Instant};

use clap::Parser;

use crate::{
    board_parser::Tetfu,
    caches::Caches,
    data::kick::Kickset,
    grid::Grid,
    input::{DropType, Key},
    pattern::{Pattern, Queue},
    piece::{Piece, Rotation},
    ranged::Ranged,
    text::Text,
};

#[derive(Debug)]
pub struct Sfce {
    pub program: Program,
    pub caches: Caches,
    pub buf: String,
}

#[derive(clap::Parser, Clone, Debug)]
#[clap(disable_help_flag = true)]
pub struct Program {
    #[command(subcommand)]
    sub: SfceCommand,
    #[clap(flatten)]
    pub args: Options,
}

#[derive(clap::Args, Clone, Debug)]
pub struct Options {
    #[arg(short = 'o', long = "output")]
    /// Where to output the result. If left blank, prints to stdout.
    pub output: Option<String>,
    #[arg(short = 'l', long = "link-type")]
    /// The link type for outputs. Capitalizing this prefixes the link with <https://fumen.zui.jp/>.
    pub link_type: Option<char>,
    #[arg(short = 'w', long = "width")]
    /// The assumed width of the given board.
    pub board_width: Option<usize>,
    #[arg(short = 'h', long = "height")]
    /// The assumed height of the given board.
    pub board_height: Option<usize>,
    #[arg(short = 's', long = "stopwatch", default_value = "true")]
    /// Whether or not to output timing results.
    pub stopwatch: bool,
    #[clap(flatten)]
    pub handling: Handling,
    #[arg(short = 'm', long = "margin", default_value = "2")]
    /// The amount of rows that a piece is allowed to spawn in
    pub board_margin: usize,
    #[arg(long = "no-cache")]
    /// Whether or not to store and use commonly determined results into/from a file.
    pub no_cache: bool,
    #[arg(long = "pw", default_value = "7")]
    /// For commands that output many patterns, the amount of patterns to be shown on one line before separating.
    pub pw: usize,
    #[arg(long = "no-hold", default_value = "false")]
    /// Whether or not the engine is allowed to use hold.
    pub no_hold: bool,
}

#[derive(clap::Args, Clone, Debug, PartialEq, Eq, Copy)]
pub struct Handling {
    #[arg(short = 'k', long = "kickset", default_value = "srs")]
    /// Which kickset to use.
    pub kickset: Kickset<'static>,
    #[arg(short = 'y', long = "use-180")]
    /// Whether or not the engine is allowed to perform 180-degree rotations
    pub use_180: bool,
    #[arg(short = 'd', long = "drop-type", default_value = "soft")]
    /// The allowed drop type. "none" enforces hard drops, "sonic" is similar to max gravity, and "soft" is regular dropping.
    pub drop_type: DropType,
    #[arg(short = 'i', long = "input-count", default_value = "4")]
    /// The maximum amount of inputs you can do for a single piece.
    pub max: usize,
    #[arg(long = "das")]
    /// Whether or not DAS is utilized, which allows you to move the piece all the way to one side in 1 input.
    pub das: bool,
    #[arg(short = 'f', long = "finesse")]
    /// Whether or not to care about 100% finesse.
    pub finesse: bool,
    #[arg(long = "ignore")]
    /// Whether or not to ignore the use of inputs for a placement. This may generate some impossible placements.
    pub ignore: bool,
}

impl Handling {
    #[must_use]
    pub fn possible_moves(&self) -> Vec<Key> {
        let mut possible_moves = vec![Key::MoveLeft, Key::MoveRight, Key::CW, Key::CCW];

        if self.das {
            possible_moves.insert(0, Key::DasRight);
            possible_moves.insert(0, Key::DasLeft);
        }

        if self.use_180 {
            possible_moves.push(Key::Flip);
        }

        if self.drop_type == DropType::Sonic || self.drop_type == DropType::Soft {
            possible_moves.push(Key::SonicDrop);
        }

        if self.drop_type == DropType::Soft {
            possible_moves.push(Key::SoftDrop);
        }

        possible_moves
    }
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
        #[arg(short = 'c', default_value = "..")]
        clears: Ranged<usize>,
        #[arg(short = 'm')]
        minimal: bool,
    },
    Percent {
        #[arg(short = 't', long = "tetfu")]
        tetfu: Text<Tetfu>,
        #[arg(short = 'p', long = "pattern")]
        pattern: Text<Pattern>,
        #[arg(short = 'c', default_value = "..")]
        clears: Ranged<usize>,
    },
    Test,
    Grid {
        #[arg(short = 't')]
        tetfu: Text<Tetfu>,
    },
    Finesse {
        #[arg(short = 't')]
        tetfu: Text<Tetfu>,
    },

    Possible {
        #[arg(short = 't')]
        tetfu: Text<Tetfu>,
        #[arg(short = 'p')]
        piece: Piece,
        #[arg(short = 'r')]
        rotation: Rotation,
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
    #[command(name = "hold")]
    Hold {
        #[arg(short = 'p')]
        pattern: Text<Pattern>,
    },
}

impl Sfce {
    #[must_use]
    pub fn handling(&self) -> Handling {
        self.program.args.handling
    }

    #[must_use]
    pub fn new() -> Self {
        let program = Program::parse();
        Self {
            program,
            caches: Caches::default(),
            buf: String::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        if !self.program.args.no_cache {
            self.load_state()?;
        }
        let i = Instant::now();
        // dbg!(&self);
        match self.program.sub.clone() {
            SfceCommand::Fumen(l) => self.fumen_commnad(l)?,
            SfceCommand::Pattern(l) => self.pattern_command(l)?,
            SfceCommand::Move {
                tetfu,
                pattern,
                clears,
                minimal,
            } => self.move_command(&tetfu.contents(), &pattern.contents(), clears, minimal)?,
            SfceCommand::Percent {
                tetfu,
                pattern,
                clears,
            } => self.percent_command(&tetfu.contents(), &pattern.contents(), clears)?,
            SfceCommand::Finesse { tetfu } => self.finesse_command(&tetfu.contents())?,
            SfceCommand::Grid { tetfu } => write!(self.buf, "{}", tetfu.grid().as_deoptimized())?,
            SfceCommand::Test => self.test_command()?,
            SfceCommand::Possible {
                tetfu,
                piece,
                rotation,
            } => self.possible(&tetfu, piece, rotation),
        }

        if let Some(s) = &self.program.args.output {
            println!("--> wrote {} bytes to path", self.buf.len());
            std::fs::write(s, self.buf.clone())?;
        } else {
            writeln!(std::io::stdout(), "{}", self.buf)?;
        }

        if self.program.args.stopwatch {
            println!("--> took {:.3}s", i.elapsed().as_secs_f64());
        }

        if !self.program.args.no_cache {
            self.save_state()?;
        }

        Ok(())
    }

    #[must_use]
    pub fn tetfu(&self, f: &Grid) -> String {
        if let Some(t) = self.program.args.link_type {
            if t.is_lowercase() {
                format!("{t}{}", &f.fumen().encode()[1..])
            } else if t == 'Q' {
                format!("https://qv.rqft.workers.dev/board?{}", f.fumen().encode())
            } else if t == 'D' {
                format!("https://fumen.zui.jp/?D{}", &f.fumen().encode()[1..])
            } else {
                format!(
                    "https://harddrop.com/fumen?{}{}",
                    t.to_lowercase(),
                    &f.fumen().encode()[1..]
                )
            }
        } else {
            f.to_string()
        }
    }

    #[must_use]
    pub fn resize(&self, mut f: Grid) -> Grid {
        if let Some(w) = self.program.args.board_width {
            f.set_width(w);
        }

        if let Some(h) = self.program.args.board_height {
            f.set_height(h);
        }

        f.set_margin(self.program.args.board_margin);

        f
    }

    #[must_use]
    pub fn hold_queues(&self, queue: Queue) -> HashSet<Queue> {
        if self.program.args.no_hold {
            HashSet::from([queue])
        } else {
            queue.hold_queues()
        }
    }
}

impl Default for Sfce {
    fn default() -> Self {
        Self::new()
    }
}
