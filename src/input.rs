use strum::EnumIter;

use crate::{
    board::Board,
    grid::Grid,
    piece::{Piece, Rotation}, placement::Placement,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub piece: Piece,
    pub board: Board,
    pub location: (usize, usize),
    pub rotation: Rotation,
}

impl Input {
    pub fn new(board: Board, piece: Piece, spawn_at: (usize, usize)) -> Self {
        Self {
            board,
            piece,
            location: spawn_at,
            rotation: Rotation::North,
        }
    }
    pub fn placement(&self) -> Placement {
        Placement {
            x: self.location.0,
            y: self.location.1,
            rotation: self.rotation,
            piece: self.piece,
        }
    }

    pub fn is_valid(&self, placement: Placement) -> bool {
        self.board.is_valid_placement(placement, true)
    }

    pub fn move_left(&mut self) {
        let mut np = self.placement();
        if let Some(t) = np.x.checked_sub(1) {
            np.x = t;
            if self.is_valid(np) {
                self.location.0 = np.x;
            }
        }
    }

    pub fn move_right(&mut self) {
        let mut np = self.placement();
        // println!("{:?}", self.placement());
        if let Some(t) = np.x.checked_add(1) {
            np.x = t;
            if self.is_valid(np) {
                self.location.0 = np.x;
            }
        }
    }

    pub fn soft_drop(&mut self) {
        let mut np = self.placement();
        if let Some(t) = np.y.checked_sub(1) {
            np.y = t;

            if self.is_valid(np) {
                self.location.1 = np.y;
            }
        }
    }

    pub fn fly(&mut self) {
        let mut np = self.placement();
        if let Some(t) = np.y.checked_add(1) {
            np.y = t;

            if self.is_valid(np) {
                self.location.1 = np.y;
            }
        }
    }

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
        while self.can_move(Rotation::South) {
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

    // TODO: use kicksets
    pub fn cw(&mut self) {
        let mut np = self.placement();
        np.rotation = np.rotation.cw();

        if self.is_valid(np) {
            self.rotation = np.rotation;
        }
    }

    pub fn ccw(&mut self) {
        let mut np = self.placement();
        np.rotation = np.rotation.ccw();

        if self.is_valid(np) {
            self.rotation = np.rotation;
        }
    }

    pub fn flip(&mut self) {
        let mut np = self.placement();
        np.rotation = np.rotation.flip();
        if self.is_valid(np) {
            self.rotation = np.rotation;
        }
    }

    pub fn place(&self) -> Board {
        self.board.with_placement(self.placement())
    }

    pub fn send_key(&mut self, key: Key) {
        match key {
            Key::MoveLeft => self.move_left(),
            Key::MoveRight => self.move_right(),
            Key::DasLeft => self.das_left(),
            Key::DasRight => self.das_right(),
            Key::RotateCW => self.cw(),
            Key::RotateCCW => self.ccw(),
            Key::Flip => self.flip(),
            Key::SoftDrop => self.soft_drop(),
            Key::HardDrop | Key::SonicDrop => self.sonic_drop(),
        };
    }

    

    pub fn is_useful(&self, key: Key) -> bool {
      let mut c = self.clone();
      c.send_key(key);
      &c != self
    }

    pub fn send_keys(&mut self, keys: &[Key]) {
        for key in keys {
            self.send_key(*key);
        }
    }

    pub fn can(&self, key: Key) -> bool {
        let mut c = self.clone();
        c.send_key(key);
        self == &c
    }

    pub fn show_inputs(&mut self, keys: &[Key]) -> Grid {
        let mut g = Grid::default();
        g.add_page(self.board.clone());
        for key in keys {
            self.send_key(*key);
            g.add_page(self.place());
        }

        g
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumIter)]
pub enum Key {
    MoveLeft,
    MoveRight,
    DasLeft,
    DasRight,
    RotateCW,
    RotateCCW,
    Flip,
    SoftDrop,
    HardDrop,
    SonicDrop,
}
