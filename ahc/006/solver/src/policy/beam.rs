use std::collections::HashSet;

use crate::util::manhattan_16;

use super::super::{
    game::{Game, N},
    util::{manhattan, tsp},
};

use super::Policy;

pub struct BeamPolicy {
    game: Game,
}

impl BeamPolicy {
    pub fn new(game: Game) -> Self {
        Self { game }
    }
}

struct BeamNode {
    cur: (u16, u16),
    cost: usize,
    history: Vec<(u16, u16)>,
    visited: HashSet<u16>,
    releases: Vec<u16>,
}

impl BeamPolicy {
    fn find_good_limit(&self) -> u16 {
        // 中心(400,400)から距離Mのrowが50個以上ある最小のMを求める
        let mut min = 0;
        let mut max = 1000;
        while max - min > 1 {
            let mid = (max + min) / 2;
            let mut cnt = 0;
            for i in 0..N {
                if manhattan_16(
                    (400, 400),
                    (self.game.input.a[i] as u16, self.game.input.b[i] as u16),
                ) <= mid
                    && manhattan_16(
                        (400, 400),
                        (self.game.input.c[i] as u16, self.game.input.d[i] as u16),
                    ) <= mid
                {
                    cnt += 1;
                }
            }
            if cnt >= 75 {
                max = mid;
            } else {
                min = mid;
            }
        }
        max
    }
}

impl Policy for BeamPolicy {
    fn solve(&self) -> (Vec<usize>, Vec<(usize, usize)>) {
        let mut cur = (400, 400);
        let mut beams = vec![BeamNode {
            cur,
            cost: 0,
            history: vec![],
            visited: HashSet::new(),
            releases: vec![],
        }];
        let beam_width = 500;
        let mut i = 0;
        let limit = self.find_good_limit();
        eprintln!("limit: {}", limit);
        let mut bans = HashSet::new();
        for i in 0..N {
            if manhattan_16(
                (400, 400),
                (self.game.input.a[i] as u16, self.game.input.b[i] as u16),
            ) > limit as u16
                || manhattan_16(
                    (400, 400),
                    (self.game.input.c[i] as u16, self.game.input.d[i] as u16),
                ) > limit as u16
            {
                bans.insert(i as u16);
            }
        }
        loop {
            i += 1;
            eprintln!("i: {}", i);
            let mut next_beams = vec![];
            for beam in beams {
                if beam.visited.len() < 50 {
                    for j in 0..N {
                        if bans.contains(&(j as u16)) {
                            continue;
                        }
                        if beam.visited.contains(&(j as u16)) {
                            continue;
                        }
                        let restaurant = (self.game.input.a[j] as u16, self.game.input.b[j] as u16);
                        let mut dist = manhattan_16(beam.cur, restaurant) as usize;
                        let mut next_history = beam.history.clone();
                        next_history.push(restaurant);
                        let mut next_visited = beam.visited.clone();
                        next_visited.insert(j as u16);
                        let mut next_releases = beam.releases.clone();
                        next_releases.push(j as u16);
                        // releaseを抱えすぎると減点
                        // dist += next_releases.len().pow(2);
                        next_beams.push(BeamNode {
                            cur: restaurant,
                            cost: beam.cost + dist,
                            history: next_history,
                            visited: next_visited,
                            releases: next_releases,
                        });
                    }
                }
                for release in &beam.releases {
                    let release_pos = (
                        self.game.input.c[*release as usize] as u16,
                        self.game.input.d[*release as usize] as u16,
                    );
                    let mut dist = manhattan_16(beam.cur, release_pos) as usize;
                    let mut next_history = beam.history.clone();
                    next_history.push(release_pos);
                    let mut next_releases = beam.releases.clone();
                    next_releases.retain(|&x| x != *release);
                    // releaseを抱えすぎると減点
                    // dist += next_releases.len().pow(2);
                    next_beams.push(BeamNode {
                        cur: release_pos,
                        cost: beam.cost + dist,
                        history: next_history,
                        visited: beam.visited.clone(),
                        releases: next_releases,
                    });
                }
            }
            next_beams.sort_by_key(|beam| beam.cost);
            next_beams.truncate(beam_width);
            beams = next_beams;
            if beams[0].visited.len() == 50 && beams[0].releases.len() == 0 {
                break;
            }
        }

        let mut ops = vec![];
        ops.push((400, 400));
        for i in 0..beams[0].history.len() {
            ops.push((
                beams[0].history[i].0 as usize,
                beams[0].history[i].1 as usize,
            ));
        }
        ops.push((400, 400));
        (
            beams[0]
                .visited
                .clone()
                .into_iter()
                .map(|x| x as usize)
                .collect::<Vec<usize>>(),
            ops,
        )
    }
}
