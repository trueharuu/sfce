use crate::{
    board::Board,
    piece::{Piece, Placement, Rotation},
};

#[derive(Clone, Debug)]
pub struct Input {
    pub piece: Piece,
    pub board: Board,
    pub location: (usize, usize),
    pub rotation: Rotation,
}

impl Input {
    pub fn placement(&self) -> Placement {
        Placement {
            x: self.location.0,
            y: self.location.1,
            rotation: self.rotation,
            piece: self.piece,
        }
    }

    pub fn move_left(mut self) -> Option<Self> {
        let mut np = self.placement();
        np.x = np.x.checked_sub(1)?;
        if self.board.is_valid_placement(np) {
            self.location.0 -= 1;
            Some(self)
        } else {
            None
        }
    }

    pub fn move_right(mut self) -> Option<Self> {
        let mut np = self.placement();
        np.x = np.x.checked_add(1)?;
        if self.board.is_valid_placement(np) {
            self.location.0 += 1;
            Some(self)
        } else {
            None
        }
    }

    pub fn soft_drop(mut self) -> Option<Self> {
        let mut np = self.placement();
        np.y = np.y.checked_sub(1)?;
        if self.board.is_valid_placement(np) {
            self.location.1 -= 1;
            Some(self)
        } else {
            None
        }
    }

    pub fn hard_drop(mut self) -> Option<Self> {
        while let Some(z) = self.clone().soft_drop() {
            self = z;
        }

        Some(self)
    }

    // TODO: use kicksets
    pub fn cw(mut self) -> Option<Self> {
      let mut np = self.placement();
      np.rotation = np.rotation.cw();
      if self.board.is_valid_placement(np) {
        self.rotation = np.rotation;
        Some(self)
      } else {
        None
      }
    }

    pub fn ccw(mut self) -> Option<Self> {
      let mut np = self.placement();
      np.rotation = np.rotation.ccw();
      if self.board.is_valid_placement(np) {
        self.rotation = np.rotation;
        Some(self)
      } else {
        None
      }
    }
    
    pub fn flip(mut self) -> Option<Self> {
      let mut np = self.placement();
      np.rotation = np.rotation.flip();
      if self.board.is_valid_placement(np) {
        self.rotation = np.rotation;
        Some(self)
      } else {
        None
      }
    }

    pub fn lock(self) -> Board {
      self.board.with_placement(self.placement())
    }
}
