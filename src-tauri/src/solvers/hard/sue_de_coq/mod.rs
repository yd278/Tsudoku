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
                (0, 0),
            ], //exp_candi_clues
            vec![260, 65535, 256, 260, 65535, 24, 65535, 24, 16, 65535], //exp_candi_masks
        );
    }

    #[test]
    fn sue_de_coq_test_2() {
        test_function_e(
            SueDeCoq,
            [
                260, 308, 276, 64, 2, 128, 8, 1, 52, 1, 56, 2, 256, 4, 56, 128, 64, 48, 140, 124,
                220, 1, 40, 56, 18, 54, 256, 32, 128, 348, 6, 73, 79, 338, 30, 94, 14, 94, 92, 134,
                200, 256, 32, 30, 1, 270, 1, 332, 38, 16, 110, 322, 14, 128, 64, 262, 388, 8, 416,
                38, 1, 178, 50, 138, 10, 32, 16, 193, 67, 4, 256, 74, 16, 270, 1, 166, 480, 102,
                66, 170, 106,
            ],
            vec![(6, 7), (8, 7), (0, 8), (3, 8)], //exp_actions
            vec![2, 10, 48, 24],                  //exp_action_targets
            vec![Box(8), Col(8)],                 //house_clues
            vec![
                (8, 6),
                (0, 0),
                (6, 8),
                (7, 8),
                (8, 8),
                (0, 0),
                (1, 8),
                (0, 0),
                (6, 8),
                (8, 8),
                (0, 0),
                (7, 8),
                (8, 8),
            ], //exp_candidate_clues
            vec![66, 65535, 2, 66, 66, 65535, 48, 65535, 48, 32, 65535, 8, 8], //exp candidate_clue masks
        )
    }

    #[test]
    fn sue_de_coq_test_3() {
        test_function_e(
            SueDeCoq,
            [
                4, 2, 8, 128, 32, 1, 16, 256, 64, 32, 16, 64, 256, 8, 4, 128, 2, 1, 256, 1, 128,
                16, 2, 64, 4, 32, 8, 2, 480, 1, 8, 4, 400, 320, 208, 432, 16, 448, 4, 2, 129, 32,
                329, 200, 384, 8, 416, 288, 64, 145, 400, 259, 148, 438, 1, 268, 274, 32, 144, 146,
                330, 220, 406, 128, 260, 274, 1, 64, 8, 32, 20, 278, 64, 40, 50, 4, 256, 146, 10,
                1, 146,
            ],
            vec![(3, 8), (4, 6), (4, 7), (5, 6), (5, 8), (6, 7)], //exp_actions
            vec![384, 320, 192, 256, 384, 20],                    //exp_action_targets
            vec![Box(5), Col(7)],                                 //house_clues
            vec![
                (3, 6),
                (4, 8),
                (0, 0),
                (3, 7),
                (5, 7),
                (0, 0),
                (7, 7),
                (0, 0),
                (3, 7),
                (5, 7),
                (0, 0),
            ], //exp_candidate_clues
            vec![320, 384, 65535, 192, 128, 65535, 20, 65535, 16, 20, 65535], //exp candidate_clue masks
        )
    }

    #[test]
    fn sue_de_coq_test_4() {
        test_function_e(
            SueDeCoq,
            [
                32, 8, 1, 80, 80, 128, 4, 2, 256, 128, 64, 4, 2, 256, 1, 32, 8, 16, 16, 256, 2, 36,
                36, 8, 1, 128, 64, 324, 128, 304, 116, 2, 116, 8, 336, 1, 73, 17, 56, 240, 240,
                256, 2, 80, 4, 326, 22, 272, 8, 1, 84, 128, 336, 32, 267, 19, 408, 176, 176, 50,
                64, 4, 10, 14, 32, 136, 256, 196, 70, 16, 1, 10, 6, 22, 64, 1, 8, 22, 256, 32, 128,
            ],
            vec![(3, 2), (4, 2), (5, 1), (6, 0)], //exp_actions
            vec![270, 14, 14, 10],                //exp_action_targets
            vec![Box(3), Col(0)],                 //house_clues
            vec![
                (4, 1),
                (5, 2),
                (0, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (0, 0),
                (7, 0),
                (8, 0),
                (0, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (0, 0),
                (3, 0),
                (4, 0),
                (5, 0),
            ], //exp_candidate_clues
            vec![
                17, 272, 65535, 256, 1, 256, 65535, 14, 6, 65535, 4, 8, 6, 65535, 64, 64, 64,
            ], //exp candidate_clue masks
        )
    }

    #[test]
    fn sue_de_coq_test_5() {
        test_function_e(
            SueDeCoq,
            [
                128, 2, 24, 337, 32, 84, 265, 265, 68, 321, 32, 24, 128, 325, 84, 265, 2, 68, 321,
                257, 4, 2, 329, 72, 16, 32, 128, 32, 8, 1, 272, 386, 146, 4, 64, 274, 16, 4, 258,
                353, 323, 98, 387, 385, 8, 258, 128, 64, 281, 271, 30, 32, 273, 275, 8, 16, 32, 4,
                130, 256, 64, 129, 3, 4, 64, 386, 40, 138, 1, 394, 408, 306, 259, 257, 386, 104,
                16, 234, 394, 4, 290,
            ],
            vec![(7, 7), (7, 8), (8, 8), (4, 6)], //exp_actions
            vec![128, 2, 2, 257],                 //exp_action_targets
            vec![Box(8), Col(6)],                 //house_clues
            vec![
                (6, 7),
                (6, 8),
                (0, 0),
                (7, 6),
                (8, 6),
                (0, 0),
                (0, 6),
                (1, 6),
                (0, 0),
                (7, 6),
                (8, 6),
                (0, 0),
            ], //exp_candidate_clues
            vec![
                129, 3, 65535, 130, 130, 65535, 265, 265, 65535, 264, 264, 65535,
            ], //exp candidate_clue masks
        )
    }
    #[test]
    fn sue_de_coq_test_6() {
        test_function_e(
            SueDeCoq,
            [
                4, 2, 256, 161, 201, 225, 16, 9, 233, 152, 136, 65, 165, 221, 225, 161, 2, 256,
                152, 32, 65, 385, 475, 451, 129, 4, 201, 256, 4, 8, 2, 129, 129, 64, 32, 16, 64, 1,
                32, 260, 260, 16, 8, 128, 2, 2, 16, 128, 64, 32, 8, 257, 257, 4, 168, 64, 4, 417,
                387, 419, 387, 16, 137, 1, 256, 16, 8, 130, 4, 162, 64, 160, 168, 136, 2, 16, 449,
                481, 4, 265, 137,
            ],
            vec![(0, 4), (1, 4), (2, 4), (2, 5), (6, 3)], //exp_actions
            vec![193, 193, 193, 193, 256],                //exp_action_targets
            vec![Box(1), Col(3)],                         //house_clues
            vec![
                (0, 3),
                (0, 5),
                (1, 5),
                (0, 0),
                (1, 3),
                (2, 3),
                (0, 0),
                (4, 3),
                (0, 0),
                (1, 3),
                (2, 3),
                (0, 0),
            ], //exp_candidate_clues
            vec![
                161, 225, 225, 65535, 161, 129, 65535, 260, 65535, 4, 256, 65535,
            ], //exp candidate_clue masks
        )
    }
}
