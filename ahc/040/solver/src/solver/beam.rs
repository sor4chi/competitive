use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::{fmt::Display, io::Write};

use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;

use crate::state::Pos;
use crate::{
    io::{Direction, Input, Operation, Query, Rotation, IO},
    state::State,
};

use std::fs::File;

use super::Solver;

pub struct BeamSolver<'a> {
    input: &'a Input,
    io: &'a IO,
}

impl BeamSolver<'_> {
    pub fn new<'a>(input: &'a Input, io: &'a IO) -> BeamSolver<'a> {
        BeamSolver { input, io }
    }
}

impl Solver for BeamSolver<'_> {
    fn solve(&mut self) {
        #[derive(Clone, PartialEq, Eq)]
        struct BeamState {
            score: i32,
            operations: Vec<Operation>,
        }

        let mut beams = vec![BeamState {
            score: i32::MAX,
            operations: vec![],
        }];

        let mut rng = Pcg64Mcg::new(42);
        let beam_width = 100;
        let input = self.input;

        // HashSet to track visited states
        let mut visited_states: HashSet<Vec<Pos>> = HashSet::new();

        for t in 0..self.input.N {
            let mut next_beams = vec![];
            for beam in beams {
                for r in &[Rotation::Stay, Rotation::Rotate] {
                    for d in &[Direction::Up, Direction::Left] {
                        for b in -1..t as isize {
                            let next_op = Operation {
                                p: t,
                                r: r.clone(),
                                d: d.clone(),
                                b,
                            };
                            let mut next_beam = beam.clone();
                            next_beam.operations.push(next_op);

                            // Create a state to check if it's visited
                            let mut state = State::new(input);
                            if let Err(err) = state.query(input, &next_beam.operations) {
                                panic!("{}", err);
                            }

                            let beam_state = BeamState {
                                score: state.score + rng.gen_range(0..1000),
                                operations: next_beam.operations,
                            };

                            // Skip if the state has been visited
                            if visited_states.contains(&state.pos) {
                                continue;
                            }

                            visited_states.insert(state.pos.clone());
                            next_beams.push(beam_state);
                        }
                    }
                }
            }
            next_beams.sort_by_key(|beam| beam.score);
            next_beams.truncate(beam_width);
            beams = next_beams;
            eprintln!("t: {} best score: {}", t, beams[0].score);
        }

        eprintln!("beams.len(): {}", beams.len());

        for beam in beams.iter().take(self.input.T) {
            self.io.measure(&Query {
                operations: beam.operations.clone(),
            });
        }
    }
}
