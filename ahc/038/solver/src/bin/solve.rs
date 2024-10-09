use solver::{
    io::IO,
    solver::{
        bulk_arm::BulkArmSolver, multi_op::MultiOPSolver, one_arm_tree::OneArmTreeSolver,
        search_arm::SearchArmSolver, Solver,
    },
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
    let output2 = if input.v >= 7 {
        SearchArmSolver::new(io.clone(), input.clone()).solve()
    } else {
        OneArmTreeSolver::new(io.clone(), input.clone()).solve()
    };
    let res2 = compute_score(&input, &output2);
    if input.v >= 7 {
        eprintln!("[SearchArm Solver]: {:?}", res2);
    } else {
        eprintln!("[OneArmTree Solver]: {:?}", res2);
    }
    let score2 = if !res2.1.is_empty() { i64::MAX } else { res2.0 };

    io.write(if score1 < score2 { &output1 } else { &output2 });
}
