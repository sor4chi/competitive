use solver::{
    io::IO,
    solver::{multi_op::MultiOPSolver, one_arm_tree::OneArmTreeSolver, Solver},
    tool::compute_score,
};

extern crate solver;

fn main() {
    let mut io = IO::default();
    let input = io.read();
    let output1 = MultiOPSolver::new(io.clone(), input.clone()).solve();
    let res1 = compute_score(&input, &output1);
    eprintln!("[MultiOP Solver]: {:?}", res1);
    let score1 = if !res1.1.is_empty() { i64::MAX } else { res1.0 };
    let output2 = OneArmTreeSolver::new(io.clone(), input.clone()).solve();
    let res2 = compute_score(&input, &output2);
    eprintln!("[OneArmTree Solver]: {:?}", res2);
    let score2 = if !res2.1.is_empty() { i64::MAX } else { res2.0 };

    // if score1 < score2 {
    //     output1.write();
    //     eprintln!("FINAL SCORE: {}", score1);
    // } else {
    //     output2.write();
    //     eprintln!("FINAL SCORE: {}", score2);
    // }

    output2.write();
    eprintln!("FINAL SCORE: {}", score2);
}
