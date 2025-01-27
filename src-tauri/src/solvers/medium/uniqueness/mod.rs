use crate::{
    game_board::GameBoard,
    impl_with_id,
    utils::{BitMap, Coord, House, HouseType},
};
#[derive(Copy, Clone)]
struct BiValueCell {
    x: usize,
    y: usize,
    bi_value: BitMap,
}
impl BiValueCell {
    pub fn new(x: usize, y: usize, bi_value: BitMap) -> Self {
        Self { x, y, bi_value }
    }
}
/// Iter through the whole
fn iter_valid_bi_value(game_board: &GameBoard) -> impl Iterator<Item = BiValueCell> + '_ {
    Coord::all_cells().filter_map(|(px, py)| {
        game_board.get_candidates(px, py).and_then(|candidates| {
            (candidates.count() == 2).then_some(BiValueCell::new(px, py, candidates))
        })
    })
}
/// Verify if a given cell could contain both candidates in bi_value
/// returns None if it's not
/// returns two bitmaps : bi_value candidates which appears in the cell, and extra candidates in the cell
fn valid_unique_rectangle_cell(
    game_board: &GameBoard,
    x: usize,
    y: usize,
    bi_value: BitMap,
) -> Option<(BitMap, BitMap)> {
    bi_value
        .iter_ones()
        .all(|candidate| game_board.could_have_been(x, y, candidate))
        .then_some(game_board.get_candidates(x, y).map(|candidate| {
            (
                candidate.intersect(&bi_value),
                candidate.difference(&bi_value),
            )
        }))
        .flatten()
}
/// Used in test type 2 and 3
/// find a line with two bi-value cells
/// returns the line as a House, indices of these two cells in the house, and the bi_value
fn find_base_line(
    game_board: &GameBoard,
) -> impl Iterator<Item = (House, usize, usize, BitMap)> + '_ {
    (0..2)
        .flat_map(|dim| (0..9).map(move |house_index| HouseType::from_dim(dim).house(house_index)))
        .flat_map(move |house| {
            (0..9).filter_map(move |first| {
                let (fx, fy) = Coord::from_house_and_index(&house, first);
                game_board
                    .get_candidates(fx, fy)
                    .and_then(|candidates| (candidates.count() == 2).then_some((first, candidates)))
                    .and_then(|(first, bi_value)| {
                        (first + 1..9).find_map(|second| {
                            let (sx, sy) = Coord::from_house_and_index(&house, second);
                            game_board.get_candidates(sx, sy).and_then(|candidates| {
                                (candidates == bi_value).then_some((house, first, second, bi_value))
                            })
                        })
                    })
            })
        })
}

struct SemiPossibleUR {
    base_house: House,
    base_bi_value: BitMap,
    first_index: usize,
    second_index: usize,
    span_house: House,
    first_span_candidates: BitMap,
    second_span_candidates: BitMap,
}
/// This functions is for Uniqueness Test 2 and 3
/// The side with two cells only contains UR candidates is called Base House
/// and the side with two cells contains extra candidates is called Span House
///
/// This function returns an iterator to
fn semi_possible_ur(game_board: &GameBoard) -> impl Iterator<Item = SemiPossibleUR> + '_ {
    find_base_line(game_board).flat_map(move |(base_house, first, second, bi_value)| {
        let same_house_flag = first / 3 == second / 3;
        (0..9)
            .filter(move |&x| {
                x != base_house.get_index()
                    && (x / 3 == base_house.get_index() / 3) != same_house_flag
            })
            .filter_map(move |x| {
                let span_house = base_house.get_parallel(x);
                let (fx, fy) = Coord::from_house_and_index(&span_house, first);
                let (sx, sy) = Coord::from_house_and_index(&span_house, second);
                game_board
                    .get_candidates(fx, fy)
                    .and_then(|first_span_candidates| {
                        game_board
                            .get_candidates(sx, sy)
                            .and_then(|second_span_candidates| {
                                bi_value
                                    .iter_ones()
                                    .all(|candidate| game_board.could_have_been(fx, fy, candidate))
                                    .then(|| {
                                        bi_value
                                            .iter_ones()
                                            .all(|candidate| {
                                                game_board.could_have_been(sx, sy, candidate)
                                            })
                                            .then_some(SemiPossibleUR {
                                                base_house,
                                                base_bi_value: bi_value,
                                                first_index: first,
                                                second_index: second,
                                                span_house,
                                                first_span_candidates,
                                                second_span_candidates,
                                            })
                                    })
                            })
                            .flatten()
                    })
            })
    })
}

impl_with_id!(
    UniquenessTest1,
    UniquenessTest2,
    UniquenessTest3,
    UniquenessTest4,
    UniquenessTest5
);

///[HoDoKu explanations on Uniqueness Rectangle Type 1](https://hodoku.sourceforge.net/en/tech_ur.php#u1)
/// ## Terminology
/// - The cell with extra candidates is called the **target cell**.
/// - The cell diagonally opposite to the target cell is called the **pivot**.
/// - The cell in the same row as the pivot is called the **row pincer**.
/// - The cell in the same column as the pivot is called the **column pincer**.
///
/// ## Return Format
/// - **Actions**: Contains 1 element, indicating the candidates to be eliminated in the **target cell**.
/// - **House Clues**: Contains 4 elements, representing the pivot row, target cell row, pivot column, and target cell column, respectively.
/// - **Candidate Clues**: Contains 3 elements, representing the pivot, row pincer, and column pincer, respectively.
struct UniquenessTest1 {
    id: usize,
}

mod test_1;

/// [HoDoKu explanations on Uniqueness Rectangle Type 2](https://hodoku.sourceforge.net/en/tech_ur.php#u2)
///
/// ## Terminology
/// - The side containing two bi-value cells is called the **base house**.
///     - These two cells are called the **first base cell** and **second base cell**, in ascending order of their relative positions in the base house.
/// - The side opposite to the **base house** is called the **span house**.
///     - These two cells are called the **first span cell** and **second span cell**, in ascending order of their relative positions in the span house.
/// - The extra candidate in the two span cells is called the **target**.
///
/// ## Return Format
/// - **Actions**: Contains a variable number of elements representing all candidates visible to the **target**:
///     - The candidates in the span house appear first in ascending order of their relative position in the span house.
///     - They are followed by candidates in the same box as the two span cells, also listed in ascending order of their relative positions within the box.
/// - **House Clues**: Contains 4 elements, representing the base house, the span house, and the other two sides, in ascending order.
/// - **Candidate Clues**: Contains 6 elements, representing the bi-value candidates in the first and second base cells, and the respective bi-value candidates found in the first and second span cells, followed by the targets in both span cells.
pub struct UniquenessTest2 {
    id: usize,
}
mod test_2;

/// [HoDoKu explanations on Uniqueness Rectangle Type 3](https://hodoku.sourceforge.net/en/tech_ur.php#u3)
///
/// ## Terminology
/// - The side containing two bi-value cells is called the **base house**.
///     - These two cells are called the **first base cell** and **second base cell**, in ascending order of their relative positions in the base house.
/// - The side opposite to the **base house** is called the **span house**.
///     - These two cells are called the **first span cell** and **second span cell**, in ascending order of their relative positions in the span house.
/// - The extra candidates in the two span cells is called the **subset candidates**.
/// - The
///
/// ## Return Format
/// - **Actions**: Contains a variable number of elements representing all candidates visible to the **target**:
///     - The candidates in the span house appear first in ascending order of their relative position in the span house.
///     - They are followed by candidates in the same box as the two span cells, also listed in ascending order of their relative positions within the box.
/// - **House Clues**: Contains 5 elements, representing the base house, the span house, and the other two sides, in ascending order, followed by the house of naked subsets.
/// - **Candidate Clues**: Contains 4 elements, representing the bi-value candidates in the first and second base cells, and the respective bi-value candidates found in the first and second span cells.
struct UniquenessTest3 {
    id: usize,
}
mod test_3;
/// [HoDoKu explanations on Uniqueness Rectangle Type 4](https://hodoku.sourceforge.net/en/tech_ur.php#u4)
///
/// ## Terminology
/// - The side containing two bi-value cells is called the **base house**.
///     - These two cells are called the **first base cell** and **second base cell**, in ascending order of their relative positions in the base house.
/// - The side opposite to the **base house** is called the **span house**.
///     - These two cells are called the **first span cell** and **second span cell**, in ascending order of their relative positions in the span house.
///
///
/// ## Return Format
/// - **Actions**: Contains up to 2 elements, representing the candidates eliminated in two span cells
/// - **House Clues**: Contains 5 elements, representing the base house, the span house, and the other two sides, in ascending order, followed by the house of hard link
/// - **Candidate Clues**: Contains 4 elements, representing the bi-value candidates in the first and second base cells, and the bi-value candidate which forms a hard link
struct UniquenessTest4 {
    id: usize,
}
mod test_4;

/// [HoDoKu explanations on Uniqueness Rectangle Type 5](https://hodoku.sourceforge.net/en/tech_ur.php#u5)
///
/// ## Terminology
/// - The bi-value cell is called **pivot**.
/// - The UR cell in the same row as the pivot is called **row pincer**
/// - The UR cell in the same column as the pivot is called **column pincer**
/// - The UR cell diagonally opposite to the pivot is called **target cell** or **third pincer** if it contains extra candidate
///
/// ## Return Format
/// - **Actions**: Contains a variable number of elements representing all candidates visible to the pincers.
/// - **House Clues**: Contains 4 elements, representing the base house, the span house, and the other two sides, in ascending order
/// - **Candidate Clues**: Contains a variable number of elements, the first 4 representing the bi-value candidates in the pivot, row pincer column pincer and target cell,  followed by target candidates in the pincers.
struct UniquenessTest5 {
    id: usize,
}
mod test_5;

#[cfg(test)]
mod uniqueness_test {
    use super::*;
    use crate::solvers::solution::Action::Elimination;
    use crate::solvers::solution::{Candidate, EliminationDetails, Solution};
    use crate::solvers::Solver;
    use crate::utils::House::{Box, Col, Row};
    use crate::{game_board::GameBoard, utils::House};
    fn test_function(
        solver: impl Solver,
        raws: [u16; 81],
        exp_actions: Vec<(usize, usize)>,
        exp_action_targets: Vec<u16>,
        exp_house_clues: Vec<House>,
        exp_candidate_clues: Vec<(usize, usize)>,
        exp_candidate_masks: Vec<u16>,
    ) {
        // raws
        let game_board = GameBoard::from_array(raws);
        // solver type
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        // action data
        let action_len = exp_actions.len();
        let action_std: Vec<_> = exp_actions
            .iter()
            .enumerate()
            .map(|(i, (a, b))| (a, b, exp_action_targets[i]))
            .collect();

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];

            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }
        // // if confirmation
        // assert_eq!(actions.len(), action_len);
        // for i in 0..action_len {
        //     let (x, y, raw) = action_std[i];
        //     let action = &actions[i];
        //     assert_matches!(action, confirmation(ConfirmationDetails{x,y,target:raw});
        // }

        // house_clue data
        let house_clues_len = exp_house_clues.len();

        assert_eq!(house_clues.len(), house_clues_len);
        for i in 0..house_clues_len {
            assert_eq!(house_clues[i], exp_house_clues[i]);
        }

        // candidate_clue data
        let clues_len = exp_candidate_clues.len();
        let clues_std: Vec<_> = exp_candidate_clues
            .iter()
            .enumerate()
            .map(|(i, (a, b))| (a, b, exp_candidate_masks[i]))
            .collect();
        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn uniqueness_test_1() {
        test_function(
            UniquenessTest1::with_id(1),
            [
                64, 256, 136, 24, 2, 1, 32, 144, 4, 4, 10, 32, 24, 256, 128, 83, 83, 17, 16, 130,
                1, 32, 64, 4, 256, 138, 10, 32, 144, 130, 4, 1, 64, 154, 10, 256, 1, 4, 256, 128,
                8, 18, 18, 32, 64, 8, 208, 194, 256, 32, 18, 147, 4, 17, 128, 1, 72, 66, 16, 32, 4,
                256, 10, 2, 72, 4, 1, 128, 256, 88, 88, 32, 256, 32, 16, 66, 4, 8, 67, 65, 128,
            ],
            vec![(5, 6)],
            vec![18],
            vec![Row(4), Row(5), Col(5), Col(6)],
            vec![(4, 5), (4, 6), (5, 5)],
            vec![18, 18, 18],
        );
    }

    #[test]
    fn uniqueness_test_2() {
        test_function(
            UniquenessTest2::with_id(1),
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
    fn uniqueness_test_3() {
        test_function(
            UniquenessTest3::with_id(1),
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
    #[test]
    fn uniqueness_test_3_b() {
        test_function(
            UniquenessTest3::with_id(1),
            [
                272, 2, 336, 4, 32, 128, 8, 336, 1, 280, 364, 376, 1, 282, 26, 400, 336, 388, 128,
                1, 28, 272, 280, 64, 276, 2, 32, 4, 328, 1, 32, 88, 24, 274, 128, 258, 32, 264,
                392, 146, 5, 26, 5, 272, 64, 2, 16, 192, 192, 5, 256, 32, 5, 8, 64, 128, 260, 8,
                258, 32, 263, 5, 16, 1, 296, 312, 338, 338, 4, 450, 40, 386, 280, 300, 2, 336, 128,
                1, 320, 40, 260,
            ],
            vec![(7, 2), (8, 0)],
            vec![256, 256],
            vec![Col(7), Col(1), Row(7), Row(8), Box(6)],
            vec![(7, 7), (8, 7), (7, 1), (8, 1), (7, 1), (8, 1), (6, 2)],
            vec![40, 40, 40, 40, 256, 260, 260],
        );
    }

    #[test]
    fn uniqueness_test_4() {
        test_function(
            UniquenessTest4::with_id(1),
            [
                4, 2, 128, 1, 104, 120, 80, 288, 312, 73, 305, 312, 88, 106, 128, 4, 3, 56, 73, 49,
                56, 92, 256, 126, 113, 131, 184, 32, 20, 2, 84, 69, 256, 128, 8, 65, 384, 8, 260,
                196, 229, 100, 2, 16, 65, 144, 64, 1, 2, 136, 24, 288, 288, 4, 11, 389, 268, 460,
                16, 78, 289, 133, 416, 11, 389, 64, 32, 142, 14, 273, 133, 400, 272, 436, 308, 388,
                132, 1, 8, 64, 2,
            ],
            vec![(3, 4), (4, 4)],
            vec![64, 64],
            vec![Col(8), Col(4), Row(3), Row(4)],
            vec![(3, 8), (4, 8), (3, 4), (4, 4)],
            vec![65, 65, 1, 1],
        );
    }
    #[test]
    fn uniqueness_test_5() {
        test_function(
            UniquenessTest5::with_id(1),
            [
                135, 135, 6, 64, 32, 256, 8, 16, 130, 194, 192, 256, 1, 16, 8, 32, 130, 4, 32, 8,
                16, 128, 2, 4, 64, 257, 257, 16, 5, 36, 2, 64, 128, 256, 41, 9, 131, 256, 34, 8, 4,
                16, 130, 33, 64, 192, 194, 8, 256, 1, 32, 134, 134, 16, 8, 16, 128, 4, 256, 2, 1,
                64, 32, 256, 6, 1, 32, 128, 64, 16, 12, 10, 6, 32, 64, 16, 8, 1, 134, 390, 386,
            ],
            vec![(0, 1), (4, 0)],
            vec![2, 2],
            vec![Row(1), Row(5), Col(1), Col(0)],
            vec![(1, 1), (1, 0), (5, 1), (5, 0), (1, 0), (5, 1)],
            vec![192, 192, 192, 192, 2, 2],
        );
    }
    #[test]
    fn uniqueness_test_5_b() {
        test_function(
            UniquenessTest5::with_id(1),
            [
                408, 32, 392, 64, 272, 257, 393, 4, 2, 412, 1, 268, 2, 276, 32, 392, 64, 392, 260,
                64, 2, 8, 128, 261, 32, 16, 257, 1, 130, 16, 256, 76, 68, 192, 138, 32, 12, 384,
                76, 32, 2, 16, 449, 264, 385, 32, 258, 72, 1, 72, 128, 4, 258, 16, 384, 4, 1, 16,
                32, 8, 2, 384, 64, 2, 8, 32, 128, 320, 320, 16, 1, 4, 64, 16, 384, 4, 1, 2, 392,
                32, 264,
            ],
            vec![(0, 6)],
            vec![128],
            vec![Row(8), Row(1), Col(8), Col(6)],
            vec![(8, 8), (8, 6), (1, 8), (1, 6), (8, 6), (1, 8), (1, 6)],
            vec![264, 264, 264, 264, 128, 128, 128],
        );
    }
}
