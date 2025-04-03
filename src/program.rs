use std::{collections::HashSet, fmt::Write, io::Write as iW, time::Instant};

use clap::Parser;
use itertools::Itertools;

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
    #[arg(short = 's', long = "stopwatch", default_value = "false")]
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
    #[arg(long = "no-comments", default_value = "false")]
    /// Whether or not to include page comments in the output. This significantly shortens fumen URLs.
    pub no_comments: bool,
    #[arg(long = "psep", default_value = ";")]
    pub page_sep: String,
    #[arg(long = "rsep", default_value = "|")]
    pub row_sep: String,
    #[arg(long = "raw", default_value = "false")]
    pub raw: bool,
}

#[derive(clap::Args, Clone, Debug, PartialEq, Eq)]
pub struct Handling {
    #[arg(short = 'k', long = "kickset", default_value = "srs")]
    /// Which kickset to use.
    pub kickset: Kickset,
    #[arg(short = 'y', long = "use-180")]
    /// Whether or not the engine is allowed to perform 180-degree rotations
    pub use_180: bool,
    #[arg(short = 'd', long = "drop-type", default_value = "soft")]
    /// The allowed drop type. "none" enforces hard drops, "sonic" is similar to max gravity, and "soft" is regular dropping.
    pub drop_type: DropType,
    #[arg(short = 'i', long = "input-count", default_value = "6")]
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
        #[arg(short = 'q', default_value = "..")]
        continuous_clears: Ranged<usize>,
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
    Inputs {
        #[arg(short = 't')]
        tetfu: Text<Tetfu>,
        #[arg(short = 'p')]
        piece: Piece,
        #[arg(short = 'x')]
        x: usize,
        #[arg(short = 'y')]
        y: usize,
        #[arg(short = 'r')]
        r: Rotation,
    },

    Possible {
        #[arg(short = 't')]
        tetfu: Text<Tetfu>,
        #[arg(short = 'p')]
        piece: Piece,
        #[arg(short = 'r')]
        rotation: Rotation,
    },

    Congruent {
        #[arg(short = 't')]
        tetfu: Text<Tetfu>,
        #[arg(short = 'p')]
        pattern: Text<Pattern>,
        #[arg(short = 'c', default_value = "g")]
        color: Piece,
        #[arg(short = 'm')]
        minimal: bool,
    },

    Send {
        #[arg(short = 't')]
        tetfu: Text<Tetfu>,
        #[arg(short = 'p')]
        piece: Piece,
        #[arg(short = 'k', value_delimiter = ',')]
        keys: Vec<Key>,
    },
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum FumenCli {
    #[command(name = "encode")]
    Encode {
        #[arg(short = 't', long = "grid")]
        grid: Text<Tetfu>,
    },

    #[command(name = "decode")]
    Decode {
        #[arg(short = 't', long = "fumen")]
        fumen: Text<Tetfu>,
    },

    #[command(name = "glue")]
    Glue {
        #[arg(short = 't', long = "fumen")]
        fumen: Text<String>,
    },

    #[command(name = "optimize")]
    Optimize {
        #[arg(short = 't', long = "fumen")]
        fumen: Text<Tetfu>,
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
        self.program.args.handling.clone()
    }

    #[must_use]
    pub fn new() -> Self {
        let mut program = Program::parse();
        if program.args.page_sep == "\\n" {
            program.args.page_sep = "\n".to_string();
        }
        if program.args.row_sep == "\\n" {
            program.args.row_sep = "\n".to_string();
        }

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
            SfceCommand::Fumen(l) => self.fumen_command(l)?,
            SfceCommand::Pattern(l) => self.pattern_command(l)?,
            SfceCommand::Move {
                tetfu,
                pattern,
                clears,
                minimal,
                continuous_clears
            } => self.move_command(&tetfu.contents(), &pattern.contents(), clears, minimal, continuous_clears)?,
            SfceCommand::Finesse { tetfu } => self.finesse_command(&tetfu.contents())?,

            SfceCommand::Inputs {
                tetfu,
                piece,
                x,
                y,
                r,
            } => self.inputs(&tetfu.contents(), piece, x, y, r)?,
            SfceCommand::Grid { tetfu } => write!(self.buf, "{}", tetfu.grid().as_deoptimized())?,
            SfceCommand::Test => self.test_command()?,
            SfceCommand::Possible {
                tetfu,
                piece,
                rotation,
            } => self.possible(&tetfu, piece, rotation),
            SfceCommand::Congruent {
                tetfu,
                pattern,
                color,
                minimal,
            } => self.congruent_command(&tetfu, &pattern, color, minimal)?,
            SfceCommand::Send { tetfu, piece, keys } => self.send_command(&tetfu, piece, &keys)?,
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
        let g = if self.program.args.no_comments {
            Grid::from_pages(
                f.0.clone()
                    .iter_mut()
                    .map(|x| {
                        x.comment = None;
                        x.clone()
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            f.clone()
        };
        if let Some(t) = self.program.args.link_type {
            if t.is_lowercase() {
                format!("{t}{}", &g.fumen().encode()[1..])
            } else if t == 'Q' {
                format!("https://qv.rqft.workers.dev/view?{}", g.fumen().encode())
            } else if t == 'D' {
                format!("https://fumen.zui.jp/?D{}", &g.fumen().encode()[1..])
            } else if t == 'T' {
                let mut z = String::new();
                z += "\n";
                for page in g.pages() {
                    for (i, r) in page.data.iter().rev().enumerate() {
                        for c in r {
                            z += "\x1b[48;2;";
                            z += match c {
                                Piece::I => "66;175;275",
                                Piece::J => "17;101;181",
                                Piece::O => "246;208;60",
                                Piece::L => "243;137;39",
                                Piece::Z => "235;79;101",
                                Piece::S => "81;184;77",
                                Piece::T => "151;57;162",
                                Piece::E => "40;40;40",
                                Piece::G => "134;134;134",
                                Piece::D => "134;134;134",
                            };

                            z += "m  \x1b[0m"
                        }

                        if i == 0 {
                            if let Some(s) = &page.comment {
                                z += "\t";
                                z += &s;
                            }
                        }

                        z += "\n";
                    }
                    z += "\n";
                }

                z += "https://qv.rqft.workers.dev/view?";
                z += &g.fumen().encode();

                z
            } else {
                format!(
                    "https://harddrop.com/fumen?{}{}",
                    t.to_lowercase(),
                    &g.fumen().encode()[1..]
                )
            }
        } else {
            f.pages()
                .iter()
                .map(|x| {
                    x.rows()
                        .iter()
                        .map(|x| x.iter().join(""))
                        .join(&self.program.args.row_sep)
                })
                .join(&self.program.args.page_sep)
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
    pub fn hold_queues(&self, queue: &Queue) -> HashSet<Queue> {
        if self.program.args.no_hold {
            HashSet::from([queue.clone()])
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
