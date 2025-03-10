use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        SolverIdentifier,
    },
    utils::{BitMap, Coord},
};

use super::{colorizer::Color, Colorizer};
#[derive(Clone)]
struct ExclusiveMap {
    pub map: Vec<bool>,
}

impl ExclusiveMap {
    pub fn new(color_cnt: usize) -> Self {
        Self {
            map: vec![false; 2 * color_cnt],
        }
    }

    pub fn set(&mut self, color: Color) {
        self.map[color.as_index()] = true
    }
    pub fn union_with(&mut self, other: &ExclusiveMap) {
        for i in 0..self.map.len() {
            self.map[i] |= other.map[i]
        }
    }

    pub fn check(&self, color: Color) -> bool {
        self.map[color.as_index()]
    }
}
pub struct Analyzer {
    exclusions: Vec<ExclusiveMap>,
    colorizer: Colorizer,
}

impl Analyzer {
    fn get_exclusion(&self, color: Color) -> &ExclusiveMap {
        &self.exclusions[color.as_index()]
    }
    pub fn new(colorizer: Colorizer) -> Self {
        let cnt = colorizer.color_cnt;
        let mut res = Self {
            exclusions: vec![ExclusiveMap::new(cnt); cnt * 2],
            colorizer,
        };
        for i in 0..cnt {
            res.exclusions[i << 1].map[i << 1 | 1] = true;
            res.exclusions[i << 1 | 1].map[i << 1] = true;
        }
        res
    }

    pub fn calculate_exclusions(&mut self) {
        for (x, y) in Coord::all_cells() {
            if self.colorizer.color[x][y].colored() {
                let cur_color_flip = self.colorizer.color[x][y]
                    .other()
                    .expect("this cell is guaranteed to be colored");
                for (sx, sy) in Coord::seeable_cells(x, y) {
                    if self.colorizer.color[sx][sy].colored() {
                        let see_color_flip = self.colorizer.color[sx][sy]
                            .other()
                            .expect("this cell is guaranteed to be colored");
                        self.exclusions[cur_color_flip.as_index()].set(see_color_flip);
                    }
                }
            }
        }
    }

    pub fn try_find_contradiction(&self, game_board: &GameBoard) -> Option<(usize, usize)> {
        for (x, y) in Coord::all_cells() {
            if game_board.contains_candidate(x, y, self.colorizer.target) {
                let mut exclusive = ExclusiveMap::new(self.colorizer.color_cnt);
                for (sx, sy) in Coord::seeable_cells(x, y) {
                    let color = self.colorizer.color[sx][sy];
                    if color.colored() {
                        if exclusive.check(color) {
                            return Some((x, y));
                        }
                        exclusive.union_with(self.get_exclusion(color));
                    }
                }
            }
        }
        None
    }

    pub fn get_coloring_vector(&self) -> Vec<Candidate> {
        let candidates = BitMap::from(self.colorizer.target);
        let mut collection: Vec<Vec<Candidate>> = vec![vec![]; self.colorizer.color_cnt * 2];
        for (x, y) in Coord::all_cells() {
            let color = self.colorizer.color[x][y];
            match color {
                Color::Light(_) => {
                    collection[color.as_index()].push(Candidate::new(x, y, candidates))
                }
                Color::Dark(_) => {
                    collection[color.as_index()].push(Candidate::new(x, y, candidates.complement()))
                }
                Color::Uncolored => (),
            }
        }
        collection.into_iter().flatten().collect()
    }

    pub fn try_get_solution(
        &self,
        game_board: &GameBoard,
        solver_id: SolverIdentifier,
    ) -> Option<Solution> {
        let (ax, ay) = self.try_find_contradiction(game_board)?;
        Some(Solution {
            actions: vec![Action::Elimination(EliminationDetails {
                x: ax,
                y: ay,
                target: BitMap::from(self.colorizer.target),
            })],
            house_clues: vec![],
            candidate_clues: self.get_coloring_vector(),
            solver_id,
        })
    }
}
