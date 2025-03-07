use std::{fmt::Display, str::FromStr};

use regex::Regex;

use crate::piece::{
    Piece::{self},
    Rotation::{self},
};

// TODO: add kicktables for SRS, SRS+, SRS-X, SRS-jstris
pub type RawKickset = Vec<(Piece, Rotation, Rotation, Vec<(isize, isize)>)>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kickset {
    kick: RawKickset,
}

impl Kickset {
    #[must_use]
    pub fn fetch(path: impl Display) -> Option<Self> {
        let mr = Regex::new(r"\(.*?([+\-0-9]+).*?,.*?([+\-0-9]+).*?\)").unwrap();
        let m = std::fs::read_to_string(path.to_string()).ok()?;
        let mut kset: RawKickset = Vec::new();
        for l in m.lines().filter(|x| !x.is_empty() && !x.starts_with("#")) {
            let (key, val) = l.split_once('=').unwrap();
            let piece = Piece::from_str(&key[0..=0]).unwrap();
            let ir = Rotation::from_str(&key[2..=2]).unwrap();
            let fr = Rotation::from_str(&key[3..=3]).unwrap();

            let os = if let Some(z) = val.strip_prefix("&") {
                let piece = Piece::from_str(&z[0..=0]).unwrap();
                let ir = Rotation::from_str(&z[2..=2]).unwrap();
                let fr = Rotation::from_str(&z[3..=3]).unwrap();

                kset.iter()
                    .find(|(p, i, f, _)| piece == *p && ir == *i && fr == *f)
                    .map(|x| x.3.clone())
            } else {
                let mut os = Vec::new();

                for pair in mr.captures_iter(val) {
                    // println!("{}", pair.get(1).unwrap().as_str());
                    // println!("{}", pair.get(2).unwrap().as_str());
                    os.push((
                        pair.get(1).unwrap().as_str().parse().unwrap(),
                        pair.get(2).unwrap().as_str().parse().unwrap(),
                    ));
                }

                Some(os)
            };

            kset.push((piece, ir, fr, os.unwrap()))
        }

        Some(Self { kick: kset })
    }
    #[must_use]
    pub fn get(
        &self,
        piece: Piece,
        initial_rotation: Rotation,
        final_rotation: Rotation,
    ) -> Vec<(isize, isize)> {
        self.kick
            .iter()
            .find(|(p, i, f, _)| *p == piece && *i == initial_rotation && *f == final_rotation)
            .map(|x| x.3.clone())
            .unwrap_or(vec![(0, 0)])
            .to_vec()
    }
}

impl FromStr for Kickset {
    type Err = String;
    fn from_str(z: &str) -> Result<Self, Self::Err> {
        Self::fetch(format!("kick/{z}.kick")).ok_or(format!("invalid kick table {z}"))
    }
}
