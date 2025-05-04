use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{
    board::Board,
    grid::Grid,
    piece::{Piece, Rotation},
    placement::Placement,
    program::Handling,
    traits::{contiguous_cut_seqs, do_until_same},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input<'a> {
    pub piece: Piece,
    pub board: &'a Board,
    pub location: (usize, usize),
    pub rotation: Rotation,
    pub handling: Handling,
}

impl<'a> Input<'a> {
    #[must_use]
    pub fn new(
        board: &'a Board,
        piece: Piece,
        location: (usize, usize),
        rotation: Rotation,
        handling: Handling,
    ) -> Self {
        Self {
            piece,
            board,
            location,
            rotation,
            handling,
        }
    }
    #[must_use]
    pub fn placement(&self) -> Placement {
        Placement::new(self.piece, self.location.0, self.location.1, self.rotation)
    }

    #[must_use]
    pub fn is_valid(&self, placement: Placement) -> bool {
        self.board.is_valid_placement(placement, true)
    }

    pub fn move_left(&mut self) {
        let mut np = self.placement();
        if let Some(t) = np.x().checked_sub(1) {
            np.set_x(t);
            if self.is_valid(np) {
                self.location.0 = np.x();
            }
        }
    }

    pub fn move_right(&mut self) {
        let mut np = self.placement();
        // println!("{:?}", self.placement());
        if let Some(t) = np.x().checked_add(1) {
            np.set_x(t);
            if self.is_valid(np) {
                self.location.0 = np.x();
            }
        }
    }

    pub fn soft_drop(&mut self) {
        let mut np = self.placement();
        if let Some(t) = np.y().checked_sub(1) {
            np.set_y(t);

            if self.is_valid(np) {
                self.location.1 = np.y();
            }
        }
    }

    pub fn fly(&mut self) {
        let mut np = self.placement();
        if let Some(t) = np.y().checked_add(1) {
            np.set_y(t);

            if self.is_valid(np) {
                self.location.1 = np.y();
            }
        }
    }

    #[must_use]
    pub fn can_move(&self, direction: Rotation) -> bool {
        let mut z = self.clone();
        match direction {
            Rotation::North => {
                let old = z.location.1;
                z.fly();
                old != z.location.1
            }
            Rotation::East => {
                let old = z.location.0;
                z.move_right();
                old != z.location.0
            }
            Rotation::South => {
                let old = z.location.1;
                z.soft_drop();
                old != z.location.1
            }
            Rotation::West => {
                let old = z.location.0;
                z.move_left();
                old != z.location.0
            }
        }
    }

    pub fn sonic_drop(&mut self) {
        while self.can(Key::SoftDrop) {
            self.soft_drop();
        }
    }

    pub fn das_left(&mut self) {
        while self.can_move(Rotation::West) {
            self.move_left();
        }
    }

    pub fn das_right(&mut self) {
        while self.can_move(Rotation::East) {
            self.move_right();
        }
    }

    pub fn cw(&mut self) {
        let p = self.placement();
        let ro = p.rotation();
        let rn = p.rotation().cw();

        let tests = self.handling.kickset.get(self.piece, ro, rn);

        for (tx, ty) in tests {
            if let Some(dx) = p.x().checked_add_signed(tx) {
                if let Some(mut dy) = p.y().checked_add_signed(ty) {
                    let mut np = p;
                    while self.board.is_cleared(dy) {
                        if ty > 0 {
                            dy += 1;
                        } else {
                            if dy == 0 { break; }
                            dy -= 1;
                        }
                    }
                    np.move_to((dx, dy));
                    np.set_rotation(rn);
                    if self.is_valid(np) {
                        self.location.0 = dx;
                        self.location.1 = dy;
                        self.rotation = rn;
                        return;
                    }
                }
            }
        }
    }

    pub fn ccw(&mut self) {
        let p = self.placement();
        let ro = p.rotation();
        let rn = p.rotation().ccw();

        let tests = self.handling.kickset.get(self.piece, ro, rn);

        for (tx, ty) in tests {
            if let Some(dx) = p.x().checked_add_signed(tx) {
                if let Some(mut dy) = p.y().checked_add_signed(ty) {
                    let mut np = p;
                    while self.board.is_cleared(dy) {
                        if ty > 0 {
                            dy += 1;
                        } else {
                            if dy == 0 { break; }
                            dy -= 1;
                        }
                    }
                    np.move_to((dx, dy));
                    np.set_rotation(rn);
                    if self.is_valid(np) {
                        self.location.0 = dx;
                        self.location.1 = dy;
                        self.rotation = rn;
                        return;
                    }
                }
            }
        }
    }

    pub fn flip(&mut self) {
        let p = self.placement();
        let ro = p.rotation();
        let rn = p.rotation().flip();

        let tests = self.handling.kickset.get(self.piece, ro, rn);

        for (tx, ty) in tests {
            if let Some(dx) = p.x().checked_add_signed(tx) {
                if let Some(mut dy) = p.y().checked_add_signed(ty) {
                    let mut np = p;
                    while self.board.is_cleared(dy) {
                        if ty > 0 {
                            dy += 1;
                        } else {
                            if dy == 0 { break; }
                            dy -= 1;
                        }
                    }
                    np.move_to((dx, dy));
                    np.set_rotation(rn);
                    if self.is_valid(np) {
                        self.location.0 = dx;
                        self.location.1 = dy;
                        self.rotation = rn;
                        return;
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn place(&self) -> Board {
        self.board.with_placement(self.placement())
    }

    pub fn send_key(&mut self, key: Key) {
        match key {
            Key::MoveLeft => self.move_left(),
            Key::MoveRight => self.move_right(),
            Key::DasLeft => self.das_left(),
            Key::DasRight => self.das_right(),
            Key::CW => self.cw(),
            Key::CCW => self.ccw(),
            Key::Flip => self.flip(),
            Key::SoftDrop => self.soft_drop(),
            Key::HardDrop | Key::SonicDrop => self.sonic_drop(),
        };
    }

    #[must_use]
    pub fn is_useful(&self, key: &[Key]) -> bool {
        let mut c = self.clone();
        c.send_keys(key);
        &c != self
    }

    pub fn send_keys(&mut self, keys: &[Key]) {
        for key in keys {
            self.send_key(*key);
        }
    }

    #[must_use]
    pub fn can(&self, key: Key) -> bool {
        let mut c = self.clone();
        c.send_key(key);
        self != &c
    }

    pub fn show_inputs(&mut self, keys: &[Key]) -> Grid {
        let mut g = Grid::default();
        // g.add_page(self.board.clone());
        // println!("{:?}", self.placement());
        g.add_page(self.place().with_comment("Spawn"));
        for key in keys {
            self.send_key(*key);

            g.add_page(self.place().with_comment(format!("{key:?}")));
        }

        g
    }

    #[must_use]
    pub fn remove_noop(&self, keys: &[Key]) -> Vec<Key> {
        // let output = vec![];
        let mut longest = None;
        let mut nw = None;

        for (before, seq, after) in contiguous_cut_seqs(keys.to_vec()) {
            let mut cpy = self.clone();
            cpy.send_keys(&before);
            if !cpy.is_useful(&seq) && seq.len() > longest.clone().map_or(0, |x: Vec<Key>| x.len())
            {
                longest = Some(seq);
                nw = Some((before, after));
            }
        }

        nw.map(|(x, y)| [x, y].concat()).unwrap_or(keys.to_vec())
    }

    #[must_use]
    pub fn remove_all_noops(&self, keys: &[Key]) -> Vec<Key> {
        do_until_same(keys.to_vec(), |x| self.remove_noop(&x))
    }

    #[must_use]
    pub fn has_noops(&self, keys: &[Key]) -> bool {
        self.remove_all_noops(keys) != keys
    }
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum Key {
    MoveLeft,
    MoveRight,
    DasLeft,
    DasRight,
    CW,
    CCW,
    Flip,
    SoftDrop,
    HardDrop,
    SonicDrop,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DropType {
    Sonic,
    Soft,
    None,
}

impl FromStr for DropType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sonic" => Ok(Self::Sonic),
            "soft" => Ok(Self::Soft),
            "none" => Ok(Self::None),
            _ => Err("unknown drop type".to_string()),
        }
    }
}

impl FromStr for Key {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "hd" => Ok(Self::HardDrop),
            "sf" => Ok(Self::SoftDrop),
            "l" => Ok(Self::MoveLeft),
            "dl" => Ok(Self::DasLeft),
            "r" => Ok(Self::MoveRight),
            "dr" => Ok(Self::DasRight),
            "cw" => Ok(Self::CW),
            "ccw" => Ok(Self::CCW),
            "sd" => Ok(Self::SonicDrop),
            "f" => Ok(Self::Flip),
            _ => Err("unknown key".to_string()),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::HardDrop => "hd",
                Self::SoftDrop => "sf",
                Self::MoveLeft => "l",
                Self::DasLeft => "dl",
                Self::MoveRight => "r",
                Self::DasRight => "dr",
                Self::CW => "cw",
                Self::CCW => "ccw",
                Self::SonicDrop => "sd",
                Self::Flip => "f",
            }
        )
    }
}
