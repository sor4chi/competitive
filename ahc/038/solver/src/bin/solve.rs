use std::time::Instant;

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
    unsafe {
        let mut io = IO::default();
        let mut input = io.read();
        let mut results = vec![];
        let start = Instant::now();

        {
            let output = MultiOPSolver::new(io.clone(), input.clone()).solve();
            let res = compute_score(&input, &output);
            let score = if !res.1.is_empty() { i64::MAX } else { res.0 };
            results.push((score, output, "MultiOP Solver"));
        }

        {
            input.tl = 2950;
            let output = SearchArmSolver::new(io.clone(), input.clone(), &start).solve();
            let res = compute_score(&input, &output);
            let score = if !res.1.is_empty() { i64::MAX } else { res.0 };
            results.push((score, output, "SearchArm Solver"));
        }

        for (score, _, solver_name) in &results {
            eprintln!(
                "[{}]: {}",
                solver_name,
                if *score == i64::MAX {
                    "Error".to_string()
                } else {
                    score.to_string()
                }
            );
        }

        results.sort_by_key(|x| x.0);
        let (score, output, solver_name) = &results[0];
        eprintln!("[Best]: {} ({})", score, solver_name);
        io.write(output);
    }
}
