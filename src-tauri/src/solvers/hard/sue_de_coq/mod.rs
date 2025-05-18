use std::iter::once;

use crate::{
    game_board::GameBoard,
    solvers::{
        Solver, SolverIdentifier,
        solution::{Action, Candidate, EliminationDetails, Solution},
    },
    utils::{BitMap, Coord, House, HouseType},
};

///[HoDoKu explanations on Sue De Coq ](https://hodoku.sourceforge.net/en/tech_misc.php#sdc)
/// ## Terminology
/// - The intersection cell is called the **Yoke**.
/// - The bi-value or ALS in the box is called the **Orion**.
/// - The Bi-value or ALS in the line is called the **Scorpius**.
/// - If the Yoke contains 3 cells and an extra candidate doesn't appear in nither Orion nor
///   Scorpius, the extra candidate is locked in the Yoke and shouldn't be anywher else in the box
///   nor the line. It's called rotten yoke.
///
/// ## Return Format
/// - **Actions**: Contains variable number of elements,representing the all the candidates to be
///   eliminated in each cell, box cells first and followed by line cells.
/// - **House Clues**: Contains 2 elements, representing the box and the line.
/// - **Candidate Clues**: Contains 4 or 5 groups of elements, seperated by the Candidate::SEPARATOR:
///     - Orion candidates
///     - Yoke candidates which overlap with the Orion
///     - Scorpius candidates
///     - Yoke candidates which overlap with the Scorpius
///     - (optional) rotten yoke candidates in each yoke cell
pub struct SueDeCoq;
mod yoke;
use scorpius::Scorpius;
use yoke::Yoke;
mod orion;
use orion::Orion;
mod scorpius;
impl Solver for SueDeCoq {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        Self::iter_possible_yokes(game_board)
            .flat_map(|yoke| Self::iter_valid_orion(game_board, yoke))
            .flat_map(|(orion, yoke)| Self::iter_valid_scorpius(game_board, yoke, orion))
            .find_map(|(yoke, orion, scorpius)| {
                self.generate_solution(game_board, yoke, orion, scorpius)
            })
    }

    fn solver_id(&self) -> crate::solvers::SolverIdentifier {
        SolverIdentifier::SueDeCoq
    }
}

impl SueDeCoq {
    fn iter_possible_yokes(game_board: &GameBoard) -> impl Iterator<Item = Yoke> + '_ {
        (0..2).flat_map(move |line_dim| {
            (0..9).flat_map(move |line_id| {
                (0..9).flat_map(move |box_id| {
                    Yoke::get_all_yokes(game_board, line_dim, line_id, box_id)
                })
            })
        })
    }

    fn iter_valid_orion(
        game_board: &GameBoard,
        yoke: Yoke,
    ) -> impl Iterator<Item = (Orion, Yoke)> + '_ {
        game_board
            .get_als_by_house(HouseType::Box.as_dim(), yoke.box_id())
            .iter()
            .filter_map(move |als| Orion::try_new(yoke, als).zip(Some(yoke)))
    }

    fn iter_valid_scorpius(
        game_board: &GameBoard,
        yoke: Yoke,
        orion: Orion,
    ) -> impl Iterator<Item = (Yoke, Orion, Scorpius)> + '_ {
        game_board
            .get_als_by_house(yoke.line_dim(), yoke.line_id())
            .iter()
            .filter_map(move |als| {
                Scorpius::try_new(yoke, orion, als).map(|scorpius| (yoke, orion, scorpius))
            })
    }

    fn valid_target(game_board: &GameBoard, x: usize, y: usize, mask: BitMap) -> Option<Action> {
        let target = game_board.get_candidates(x, y)?.intersect(mask);
        (target.count() > 0).then_some(Action::Elimination(EliminationDetails { x, y, target }))
    }
    fn box_targets(
        game_board: &GameBoard,
        yoke: Yoke,
        orion: Orion,
        rotten_yoke: BitMap,
    ) -> impl Iterator<Item = Action> + '_ {
        (0..9)
            .filter(move |&i| !orion.indices().union(yoke.indices_in_box()).contains(i))
            .map(move |i| Coord::from_house_and_index(&House::Box(yoke.box_id()), i))
            .filter_map(move |(x, y)| {
                Self::valid_target(game_board, x, y, orion.candidates().union(rotten_yoke))
            })
    }

    fn line_targets(
        game_board: &GameBoard,
        yoke: Yoke,
        scorpius: Scorpius,
        rotten_yoke: BitMap,
    ) -> impl Iterator<Item = Action> + '_ {
        (0..9)
            .filter(move |&i| !scorpius.indices().union(yoke.indices_in_line()).contains(i))
            .map(move |i| {
                Coord::from_house_and_index(&House::from_dim_id(yoke.line_dim(), yoke.line_id()), i)
            })
            .filter_map(move |(x, y)| {
                Self::valid_target(game_board, x, y, scorpius.candidates().union(rotten_yoke))
            })
    }
    fn cell_to_candidate_clue(
        game_board: &GameBoard,
        x: usize,
        y: usize,
        mask: BitMap,
    ) -> Option<Candidate> {
        let candidates = game_board.get_candidates(x, y)?.intersect(mask);
        if candidates.count() > 0 {
            Some(Candidate::from_coord((x, y), candidates))
        } else {
            None
        }
    }
    fn orion_clues(
        game_board: &GameBoard,
        yoke: Yoke,
        orion: Orion,
    ) -> impl Iterator<Item = Candidate> + '_ {
        (0..9)
            .filter(move |&i| orion.indices().contains(i))
            .map(move |i| Coord::from_house_and_index(&House::Box(yoke.box_id()), i))
            .filter_map(move |(x, y)| {
                Self::cell_to_candidate_clue(game_board, x, y, orion.candidates())
            })
    }
    fn yoke_orion_clues(
        game_board: &GameBoard,
        yoke: Yoke,
        orion: Orion,
    ) -> impl Iterator<Item = Candidate> {
        (0..9)
            .filter(move |&i| yoke.indices_in_box().contains(i))
            .map(move |i| Coord::from_house_and_index(&House::Box(yoke.box_id()), i))
            .filter_map(move |(x, y)| {
                Self::cell_to_candidate_clue(game_board, x, y, orion.candidates())
            })
    }
    fn scorpius_clues(
        game_board: &GameBoard,
        yoke: Yoke,
        scorpius: Scorpius,
    ) -> impl Iterator<Item = Candidate> + '_ {
        (0..9)
            .filter(move |&i| scorpius.indices().contains(i))
            .map(move |i| {
                Coord::from_house_and_index(&House::from_dim_id(yoke.line_dim(), yoke.line_id()), i)
            })
            .filter_map(move |(x, y)| {
                Self::cell_to_candidate_clue(game_board, x, y, scorpius.candidates())
            })
    }
    fn yoke_scorpius_clues(
        game_board: &GameBoard,
        yoke: Yoke,
        scorpius: Scorpius,
    ) -> impl Iterator<Item = Candidate> {
        (0..9)
            .filter(move |&i| yoke.indices_in_box().contains(i))
            .map(move |i| Coord::from_house_and_index(&House::Box(yoke.box_id()), i))
            .filter_map(move |(x, y)| {
                Self::cell_to_candidate_clue(game_board, x, y, scorpius.candidates())
            })
    }
    fn rotten_yoke_clues(
        game_board: &GameBoard,
        yoke: Yoke,
        orion: Orion,
        scorpius: Scorpius,
    ) -> impl Iterator<Item = Candidate> {
        (0..9)
            .filter(move |&i| yoke.indices_in_box().contains(i))
            .map(move |i| Coord::from_house_and_index(&House::Box(yoke.box_id()), i))
            .filter_map(move |(x, y)| {
                Self::cell_to_candidate_clue(
                    game_board,
                    x,
                    y,
                    yoke.candidates()
                        .difference(orion.candidates())
                        .difference(scorpius.candidates()),
                )
            })
    }

    fn generate_solution(
        &self,
        game_board: &GameBoard,
        yoke: Yoke,
        orion: Orion,
        scorpius: Scorpius,
    ) -> Option<Solution> {
        //TODO:
        //1. find all the candidates in the Box which locks Orion
        //2. find all the candidates in the line which locks Scorpius
        //3. check the occourence of rotten yoke.

        let rotten_yoke = yoke
            .candidates()
            .difference(orion.candidates())
            .difference(scorpius.candidates());
        let actions: Vec<Action> = Self::box_targets(game_board, yoke, orion, rotten_yoke)
            .chain(Self::line_targets(game_board, yoke, scorpius, rotten_yoke))
            .collect();
        if actions.is_empty() {
            None
        } else {
            let candidate_clues: Vec<Candidate> = Self::orion_clues(game_board, yoke, orion)
                .chain(once(Candidate::SEPARATOR))
                .chain(Self::yoke_orion_clues(game_board, yoke, orion))
                .chain(once(Candidate::SEPARATOR))
                .chain(Self::scorpius_clues(game_board, yoke, scorpius))
                .chain(once(Candidate::SEPARATOR))
                .chain(Self::yoke_scorpius_clues(game_board, yoke, scorpius))
                .chain(once(Candidate::SEPARATOR))
                .chain(Self::rotten_yoke_clues(game_board, yoke, orion, scorpius))
                .collect();

            Some(Solution {
                actions,
                candidate_clues,
                house_clues: vec![
                    House::Box(yoke.box_id()),
                    House::from_dim_id(yoke.line_dim(), yoke.line_id()),
                ],
                solver_id: self.solver_id(),
            })
        }
    }
}

#[cfg(test)]
mod sue_de_coq_test {

    use super::*;
    use crate::tests::common::test_function_e;
    use crate::utils::House::{Box, Col, Row};
    #[test]
    fn sue_de_coq_test_1() {
        test_function_e(
            SueDeCoq,
            [
                1, 8, 128, 4, 2, 64, 48, 48, 256, 2, 32, 64, 16, 256, 8, 4, 128, 1, 272, 4, 272,
                128, 32, 1, 64, 8, 2, 4, 256, 32, 8, 16, 2, 1, 64, 128, 64, 1, 8, 32, 4, 128, 256,
                2, 16, 128, 16, 2, 65, 65, 256, 40, 36, 44, 280, 128, 276, 2, 9, 32, 24, 277, 64,
                296, 64, 260, 257, 128, 16, 2, 293, 44, 312, 2, 1, 320, 72, 4, 128, 304, 40,
            ],
            vec![(7, 0), (8, 0), (6, 4), (6, 7)], //exp_actions
            vec![256, 256, 8, 16],                //exp_action_targets
            vec![Box(6), Row(6)],                 //exp_house_clues
            vec![
                (7, 2),
                (0, 0),
                (6, 0),
                (6, 2),
                (0, 0),
                (6, 6),
                (0, 0),
                (6, 0),
                (6, 2),
            ], //exp_candi_clues
            vec![260, 65535, 256, 260, 65535, 24, 65535, 24, 16], //exp_candi_masks
        );
    }

    #[test]
    fn sue_de_coq_test_2() {
        test_function_e(
            SueDeCoq,
            [
                128, 4, 256, 8, 82, 67, 114, 3, 96, 16, 2, 8, 32, 256, 65, 128, 65, 4, 64, 1, 32,
                18, 128, 4, 24, 256, 10, 1, 16, 66, 128, 74, 98, 40, 4, 256, 42, 256, 4, 80, 88,
                98, 1, 74, 128, 42, 72, 128, 1, 4, 256, 98, 16, 104, 10, 32, 66, 4, 1, 128, 256,
                72, 16, 256, 72, 1, 66, 32, 16, 4, 128, 74, 4, 128, 16, 256, 66, 8, 66, 32, 1,
            ],
            vec![(0, 4), (0, 6)],
            vec![2, 2],
            vec![Row(1), Row(0), Col(5), Col(7)],
            vec![(1, 5), (1, 7), (0, 5), (0, 7), (0, 5), (0, 7)],
            vec![65, 65, 65, 1, 2, 2],
        );
    }

    #[test]
    fn sue_de_coq_test_3() {
        test_function_e(
            SueDeCoq,
            [
                128, 9, 16, 256, 96, 9, 4, 96, 2, 4, 265, 64, 19, 40, 43, 24, 288, 128, 32, 264, 2,
                212, 204, 140, 24, 320, 1, 64, 2, 4, 8, 1, 256, 32, 128, 16, 16, 32, 8, 132, 2,
                132, 64, 1, 256, 1, 128, 256, 32, 16, 64, 2, 4, 8, 2, 80, 160, 193, 232, 169, 256,
                24, 4, 8, 84, 160, 198, 256, 166, 1, 18, 96, 256, 68, 1, 70, 108, 16, 128, 10, 96,
            ],
            vec![(2, 4)],
            vec![72],
            vec![Row(4), Row(2), Col(3), Col(5), Row(2)],
            vec![
                (4, 3),
                (4, 5),
                (2, 3),
                (2, 5),
                (2, 3),
                (2, 5),
                (2, 1),
                (2, 6),
                (2, 7),
            ],
            vec![132, 132, 132, 132, 80, 8, 264, 24, 320],
        );
    }
}
