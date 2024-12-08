use std::collections::HashMap;

use crate::{
    board::{Bits, Board},
    input::Key,
    placement::Placement,
    program::Sfce,
};

#[derive(Debug, Default)]
pub struct Caches {
    pub is_placement_possible_store: HashMap<(Bits, Placement), bool>,
    pub finesse_store: HashMap<(Placement, Bits), Vec<Key>>,
}

impl Sfce {
    pub fn is_placement_possible(&mut self, board: &Board, placement: Placement) -> bool {
        let f = board.fast();
        if let Some(s) = self
            .caches
            .is_placement_possible_store
            .get(&(f.clone(), placement))
        {
            *s
        } else if self.is_valid_placement(board, placement, false) {
            if self.handling().ignore {
                return true;
            }
            let possible = placement.is_doable(board, board.spawn(), self.handling());

            self.caches
                .is_placement_possible_store
                .insert((f, placement), possible);
            possible
        } else {
            false
        }
    }

    pub fn is_valid_placement(
        &mut self,
        board: &Board,
        placement: Placement,
        allow_floating: bool,
    ) -> bool {
        // this previously was cached but it took up 200mb of storage
        board.is_valid_placement(placement, allow_floating)
    }

    pub fn finesse(&mut self, placement: Placement, board: &Board) -> Option<Vec<Key>> {
        let f = board.fast();

        if let Some(s) = self
            .caches
            .is_placement_possible_store
            .get(&(f.clone(), placement))
        {
            if !*s {
                return None;
            }
        }

        if let Some(s) = self.caches.finesse_store.get(&(placement, f.clone())) {
            // println!("cache hit!");
            Some(s.clone())
        } else {
            let inputs = placement.inputs(board, board.spawn(), self.handling());
            if let Some(i) = inputs {
                self.caches
                    .is_placement_possible_store
                    .insert((f.clone(), placement), true);
                // if it's accessible with raw inputs, it's a real placement
                self.caches.finesse_store.insert((placement, f), i.clone());
                Some(i)
            } else {
                // we can safely assume no sequence of inputs exists if no short sequence exists
                self.caches
                    .is_placement_possible_store
                    .insert((f, placement), false);
                None
            }
        }
    }

    pub fn save_state(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(".caches")?;
        {
            let d = bincode::serialize(&self.caches.finesse_store)?;
            std::fs::write(".caches/finesse_store.bin", d)?;
        }

        {
            let d = bincode::serialize(&self.caches.is_placement_possible_store)?;
            std::fs::write(".caches/is_placement_possible.bin", d)?;
        }

        // {
        //     let d = bincode::serialize(&self.caches.is_valid_placement_store)?;
        //     std::fs::write(".caches/is_valid_placement.bin", d)?;
        // }
        Ok(())
    }

    pub fn load_state(&mut self) -> anyhow::Result<()> {
        {
            let x = std::fs::read(".caches/finesse_store.bin");

            if let Ok(t) = x {
                self.caches.finesse_store = bincode::deserialize(&t)?;
            }
        }

        {
            let x = std::fs::read(".caches/is_placement_possible.bin");

            if let Ok(t) = x {
                self.caches.is_placement_possible_store = bincode::deserialize(&t)?;
            }
        }

        // {
        //     let x = std::fs::read(".caches/is_valid_placement.bin");

        //     if let Ok(t) = x {
        //         self.caches.is_valid_placement_store = bincode::deserialize(&t)?;
        //     }
        // }

        Ok(())
    }
}
