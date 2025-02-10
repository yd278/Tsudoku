use crate::impl_with_id;

impl_with_id!(XYWing);

/// XY-Wing Solver
/// ## Terminology
/// - The pivot is denoted by P
/// - Two pincers are denoted by Q and R
/// - The candidate shared by P and Q is x, and the candidate shared by P and R is y
/// ## Return Format
/// - **Actions**: Contains variable number of elements, representing candidates pinched by two pincers.
/// - **House Clues**: An empty vector.
/// - **Candidate Clues**: Contains 6 elements, representing corresponding x in P, x in Q, y in P, y in R, z in Q and z in R.
pub struct XYWing {
    id: usize,
}

mod xy_wing;

/// XYZ-Wing Solver
/// ## Terminology
/// - The pivot is denoted by P.
/// - Two pincers are denoted by Q and R.
/// - The candidates shared by P and Q are x and z, and the candidates shared by P and R are y and z
/// ## Return Format
/// - **Actions**: Contains variable number of elements, representing candidates pinched by two pincers and seeable by P.
/// - **House Clues**: An empty vector.
/// - **Candidate Clues**: Contains 6 elements, representing corresponding x in P, x in Q, y in P, y in R,z in P, z in Q and z in R.
pub struct XYZWing{
    id: usize,
}
mod xyz_wing;

pub struct WWing{
    id : usize,
}
mod w_wing;