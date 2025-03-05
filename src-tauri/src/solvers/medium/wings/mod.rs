/// XY-Wing Solver
/// ## Terminology
/// - The pivot is denoted by P
/// - Two pincers are denoted by Q and R
/// - The candidate shared by P and Q is x, and the candidate shared by P and R is y
/// ## Return Format
/// - **Actions**: Contains variable number of elements, representing candidates pinched by two pincers.
/// - **House Clues**: An empty vector.
/// - **Candidate Clues**: Contains 6 elements, representing corresponding x in P, x in Q, y in P, y in R, z in Q and z in R.
pub struct XYWing;

mod xy_wing;

/// XYZ-Wing Solver
/// ## Terminology
/// - The pivot is denoted by P.
/// - Two pincers are denoted by Q and R.
/// - The candidates shared by P and Q are x and z, and the candidates shared by P and R are y and z
/// ## Return Format
/// - **Actions**: Contains variable number of elements, representing candidates pinched by two pincers and seeable by P.
/// - **House Clues**: An empty vector.
/// - **Candidate Clues**: Contains 7 elements, representing corresponding x in P, x in Q, y in P, y in R,z in P, z in Q and z in R.
pub struct XYZWing;
mod xyz_wing;
/// W-Wing Solver
/// ## Pre-request
/// - Pointing
/// - Claiming
/// ## Terminology
/// - The hard-linked two candidates are called **Bridge**, the one at top-left side is denoted by U and the other is denoted by V
/// - Two pincers are denoted by P and Q, where P is seeable by U and Q is seeable by V.
/// - The candidates shared by P and Q are x and z, and the candidates shared by P and R are y and z
/// ## Return Format
/// - **Actions**: Contains variable number of elements, representing candidates pinched by two pincers and seeable by P.
/// - **House Clues**: An empty vector.
/// - **Candidate Clues**: Contains 6 elements, representing corresponding x in P, x in Q, y in P, y in R,z in P, z in Q and z in R.
pub struct WWing;
mod w_wing;

#[cfg(test)]
mod wings_test {
    use super::*;
    use crate::tests::common::test_function_e;

    #[test]
    fn test_xy_wing() {
        test_function_e(
            XYWing,
            [
                8, 592, 608, 800, 128, 1, 2, 820, 820, 530, 256, 546, 4, 520, 576, 640, 513, 560,
                517, 517, 640, 802, 770, 528, 8, 800, 64, 515, 552, 16, 64, 515, 4, 256, 552, 640,
                128, 586, 834, 770, 16, 544, 580, 590, 1, 835, 611, 4, 8, 771, 128, 16, 610, 546,
                32, 598, 8, 640, 580, 770, 513, 854, 790, 854, 598, 834, 513, 580, 8, 544, 128,
                790, 838, 640, 1, 16, 32, 770, 580, 838, 8,
            ],
            vec![(5, 0)],                                         //exp_actions
            vec![2],                                              //exp_action_targets
            vec![],                                               //exp_house_clues
            vec![(1, 8), (1, 0), (1, 8), (5, 8), (1, 0), (5, 8)], //exp_candi_clues
            vec![16, 16, 32, 32, 2, 2],                           //exp_candi_masks
        );
    }

    #[test]
    fn test_xyz_wing() {
        test_function_e(
            XYZWing,
            [
                4, 793, 593, 872, 2, 777, 625, 569, 128, 592, 913, 849, 808, 992, 4, 2, 569, 536,
                32, 649, 2, 16, 704, 521, 577, 256, 516, 594, 534, 628, 1, 548, 522, 640, 586, 256,
                256, 578, 8, 640, 16, 546, 4, 608, 1, 128, 517, 549, 810, 804, 64, 560, 570, 538,
                530, 32, 784, 770, 513, 128, 8, 516, 64, 520, 832, 128, 4, 608, 816, 785, 531, 530,
                1, 774, 836, 834, 8, 786, 784, 640, 32,
            ],
            vec![(8, 3)],                                                 //exp_actions
            vec![256],                                                    //exp_action_targets
            vec![],                                                       //exp_house_clues
            vec![(8, 5), (6, 3), (8, 5), (8, 6), (8, 5), (8, 6), (6, 3)], //exp_candi_clues
            vec![2, 2, 16, 16, 256, 256, 256],                            //exp_candi_masks
        );
    }

    #[test]
    fn test_w_wing() {
        test_function_e(
            WWing,
            [
                128, 545, 16, 576, 518, 526, 522, 768, 545, 64, 768, 520, 32, 530, 1, 516, 640,
                530, 514, 4, 545, 536, 128, 256, 64, 545, 536, 528, 545, 805, 2, 613, 608, 769, 8,
                128, 773, 640, 64, 776, 525, 524, 16, 546, 546, 8, 2, 801, 784, 561, 128, 769, 576,
                516, 545, 584, 2, 4, 256, 608, 640, 16, 521, 548, 584, 640, 1, 608, 16, 522, 518,
                256, 769, 528, 773, 640, 522, 522, 32, 517, 64,
            ],
            vec![(4, 0)],                                         //exp_actions
            vec![1],                                              //exp_action_targets
            vec![],                                               //exp_house_clues
            vec![(3, 5), (6, 5), (3, 1), (3, 1), (6, 0), (6, 0)], //exp_candi_clues
            vec![32, 32, 32, 1, 32, 1],                           //exp_candi_masks
        );
    }
}
