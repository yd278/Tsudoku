use crate::{
    game_board::GameBoard,
    solvers::{solution::Solution, Solver, SolverIdentifier},
};

mod colorizer;
use colorizer::Colorizer;

/// This solver assigns colors to single digits based on hard links and identifies one candidate that conflicts with the assigned coloring pattern.
///
/// ## Return Format
/// - **Actions**: Contains a single element— the candidate that conflicts with the coloring pattern and should be removed.
/// - **House Clues**: An empty vector.
/// - **Candidate Clues**: Contains a variable number of elements. Each candidate clue is represented as a bitmap, which is either single (only one `1` bit set) or inverted single (only one `0` bit set).
///     - These bitmaps are organized into 2n segments, with a segment of single bitmaps appearing first, followed by a segment of inverted single bitmaps, and so on.
///     - Each pair of segments represents a pair of light-dark colorings.
pub struct Coloring;
mod analyzer;
use analyzer::Analyzer;

impl Solver for Coloring {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        for target in 0..9 {
            let mut colorizer = Colorizer::new(target);
            let res = colorizer.colorize(game_board);
            if res.is_err() {
                return None;
            }
            let mut analyzer = Analyzer::new(colorizer);
            analyzer.calculate_exclusions();
            if let Some(solution) = analyzer.try_get_solution(game_board, self.solver_id()) {
                return Some(solution);
            }
        }
        None
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::Coloring
    }
}

#[cfg(test)]
mod coloring_test {
    use super::*;
    use crate::tests::common::test_function_e;
    #[test]
    fn test_multi_color() {
        test_function_e(
            Coloring,
            [
                112, 8, 112, 2, 96, 128, 256, 1, 4, 96, 128, 256, 1, 4, 96, 2, 24, 24, 5, 5, 2, 8,
                256, 16, 64, 32, 128, 128, 256, 80, 96, 8, 4, 48, 2, 1, 84, 2, 1, 352, 128, 320, 8,
                84, 112, 8, 32, 68, 16, 2, 1, 128, 68, 256, 2, 64, 128, 4, 16, 296, 1, 264, 40, 33,
                16, 8, 384, 97, 2, 4, 384, 96, 256, 5, 36, 192, 97, 72, 48, 216, 2,
            ],
            vec![(8, 6)], //exp_actions
            vec![32],     //exp_action_targets
            vec![],       //exp_house_clues
            vec![
                (0, 2),
                (7, 0),
                (8, 2),
                (0, 4),
                (1, 0),
                (6, 5),
                (1, 5),
                (6, 8),
                (3, 3),
                (4, 8),
                (8, 6),
                (3, 6),
                (4, 4),
            ], //exp_candi_clues
            vec![32, 32, 479, 32, 32, 32, 479, 479, 32, 32, 32, 479, 479], //exp_candi_masks
        )
    }
    #[test]
    fn test_single_color() {
        test_function_e(
            Coloring,
            [
                32, 4, 128, 10, 1, 256, 16, 64, 10, 19, 259, 272, 136, 64, 132, 32, 258, 140, 8,
                258, 64, 34, 176, 148, 384, 1, 6, 401, 32, 4, 129, 384, 8, 2, 400, 64, 400, 8, 272,
                64, 2, 32, 4, 400, 1, 64, 385, 2, 4, 400, 17, 384, 8, 32, 4, 16, 8, 161, 160, 131,
                64, 130, 256, 258, 386, 1, 16, 4, 64, 8, 32, 130, 130, 64, 32, 256, 8, 130, 1, 4,
                16,
            ],
            vec![(2, 5)], //exp_actions
            vec![128],    //exp_action_targets
            vec![],       //exp_house_clues
            vec![
                (1, 8),
                (5, 6),
                (6, 7),
                (7, 1),
                (8, 5),
                (2, 6),
                (5, 1),
                (7, 8),
                (8, 0),
                (4, 0),
                (4, 7),
            ], //exp_candi_clues
            vec![128, 128, 128, 128, 128, 383, 383, 383, 383, 128, 383], //exp_candi_masks
        )
    }
}
