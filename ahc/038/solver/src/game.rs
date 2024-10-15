use std::collections::{HashMap, HashSet, VecDeque};

use crate::{io::Rotate, original_lib::id::IncrementalIDGenerator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_d(&self) -> (i32, i32) {
        match self {
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    pub fn idx(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

    pub fn from_idx(idx: usize) -> Self {
        match idx {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Invalid idx: {}", idx),
        }
    }

    pub fn diff(&self, other: Direction) -> usize {
        let diff = (self.idx() as i32 - other.idx() as i32).abs();
        if diff > 2 {
            4 - diff as usize
        } else {
            diff as usize
        }
    }

    // 目標にあわせるための回転操作列を返す
    pub fn align(&self, other: Direction) -> Vec<Rotate> {
        let real_diff = other.idx() as i32 - self.idx() as i32;
        match real_diff {
            0 => vec![],
            1 | -3 => vec![Rotate::Right],
            -1 | 3 => vec![Rotate::Left],
            2 => vec![Rotate::Right, Rotate::Right],
            -2 => vec![Rotate::Left, Rotate::Left],
            _ => panic!("Invalid diff: {}", real_diff),
        }
    }

    // ベースの方向から見た相対方向から、絶対方向を返す
    pub fn from_relative(&self, relative: Direction) -> Direction {
        let base_idx = self.idx();
        let relative_idx = relative.idx();
        let new_idx = (base_idx + relative_idx) % 4;
        match new_idx {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Invalid idx: {}", new_idx),
        }
    }

    // 絶対方向から、ベースの方向から見た相対方向を返す
    pub fn to_relative(&self, absolute: Direction) -> Direction {
        let base_idx = self.idx();
        let absolute_idx = absolute.idx();
        let new_idx = (absolute_idx + 4 - base_idx) % 4;
        match new_idx {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Invalid idx: {}", new_idx),
        }
    }
}

#[test]
fn test_direction_diff() {
    assert_eq!(Direction::Right.diff(Direction::Left), 2);
    assert_eq!(Direction::Right.diff(Direction::Right), 0);
    assert_eq!(Direction::Right.diff(Direction::Up), 1);
    assert_eq!(Direction::Right.diff(Direction::Down), 1);
}

#[test]
fn test_direction_align() {
    assert_eq!(
        Direction::Right.align(Direction::Left),
        vec![Rotate::Right, Rotate::Right]
    );
    assert_eq!(Direction::Right.align(Direction::Right), vec![]);
    assert_eq!(Direction::Right.align(Direction::Up), vec![Rotate::Left]);
    assert_eq!(Direction::Right.align(Direction::Down), vec![Rotate::Right]);
}

#[test]
fn test_direction_from_relative() {
    assert_eq!(
        Direction::Right.from_relative(Direction::Right),
        Direction::Down
    );
    assert_eq!(
        Direction::Right.from_relative(Direction::Left),
        Direction::Up
    );
    assert_eq!(
        Direction::Right.from_relative(Direction::Up),
        Direction::Right
    );
    assert_eq!(
        Direction::Right.from_relative(Direction::Down),
        Direction::Left
    );
}

#[test]
fn test_direction_to_relative() {
    assert_eq!(
        Direction::Right.to_relative(Direction::Down),
        Direction::Right
    );
    assert_eq!(Direction::Right.to_relative(Direction::Up), Direction::Left);
    assert_eq!(
        Direction::Right.to_relative(Direction::Right),
        Direction::Up
    );
    assert_eq!(
        Direction::Right.to_relative(Direction::Left),
        Direction::Down
    );
}

pub const DIRS: [Direction; 4] = [
    Direction::Right,
    Direction::Up,
    Direction::Left,
    Direction::Down,
];

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct ArmNodeID(pub usize);
impl From<usize> for ArmNodeID {
    fn from(x: usize) -> Self {
        ArmNodeID(x)
    }
}

pub const ROOT_ID: ArmNodeID = ArmNodeID(0);

#[derive(Clone)]
pub struct ArmTree {
    idg: IncrementalIDGenerator<ArmNodeID>,
    // [parent] -> [(child, arm_length)]
    pub tree: HashMap<ArmNodeID, Vec<(ArmNodeID, usize)>>,
    // [child] -> [parent]
    tree_rev: HashMap<ArmNodeID, ArmNodeID>,
    // [node] -> (x, y)
    pub tree_pos: HashMap<ArmNodeID, (i32, i32)>,
    pub leaves: HashSet<ArmNodeID>,
}

impl ArmTree {
    pub fn new(initial_pos: (usize, usize)) -> Self {
        let tree = HashMap::new();
        let mut tree_pos = HashMap::new();
        let mut idg = IncrementalIDGenerator::new();
        let id = idg.generate();
        tree_pos.insert(id, (initial_pos.0 as i32, initial_pos.1 as i32));
        let mut leaves = HashSet::new();
        leaves.insert(id);
        Self {
            idg,
            tree,
            tree_rev: HashMap::new(),
            tree_pos,
            leaves,
        }
    }

    pub fn add_arm(&mut self, parent: ArmNodeID, length: usize) -> ArmNodeID {
        let mut pos = self.tree_pos[&parent];
        pos.1 += length as i32;
        let id = self.idg.generate();
        self.tree.entry(parent).or_insert(vec![]).push((id, length));
        self.tree_rev.insert(id, parent);
        self.tree_pos.insert(id, pos);
        self.leaves.remove(&parent);
        self.leaves.insert(id);
        id
    }

    pub fn remove_arm(&mut self, arm_id: ArmNodeID) {
        let parent = self.tree_rev[&arm_id];
        let mut children = self.tree.remove(&parent).unwrap();
        children.retain(|(child, _)| *child != arm_id);
        self.tree.insert(parent, children);
        self.tree_rev.remove(&arm_id);
        self.tree_pos.remove(&arm_id);
        self.leaves.remove(&arm_id);
        if self.tree[&parent].is_empty() {
            self.leaves.insert(parent);
        }
    }

    pub fn rotate(&mut self, rotate_id: ArmNodeID, rotate: Rotate) {
        // rotate_idよりも深い部分を、rotate_idを中心に回転する
        let parent_id = self.tree_rev[&rotate_id];
        let center_pos = self.tree_pos[&parent_id];
        // BFSをして、出てきた順に回転させる
        // 新しい座標はcenter_posを(a,b)とした時、(x,y) -> (x-a, y-b)としてから回転行列をかけ、(a,b)を足す
        let mut q = VecDeque::new();
        q.push_back(rotate_id);
        while let Some(node_id) = q.pop_front() {
            let pos = self.tree_pos[&node_id];
            let centered_pos = (pos.0 - center_pos.0, pos.1 - center_pos.1);
            let rotated_pos = match rotate {
                Rotate::Left => (-centered_pos.1, centered_pos.0),
                Rotate::Right => (centered_pos.1, -centered_pos.0),
                Rotate::Stay => panic!("Stay is not allowed in rotate"),
            };
            let new_pos = (rotated_pos.0 + center_pos.0, rotated_pos.1 + center_pos.1);
            self.tree_pos.insert(node_id, new_pos);
            if let Some(children) = self.tree.get(&node_id) {
                for (child, _) in children {
                    q.push_back(*child);
                }
            }
        }
    }

    pub fn all_shift(&mut self, shift: (i32, i32)) {
        for (_, pos) in self.tree_pos.iter_mut() {
            pos.0 += shift.0;
            pos.1 += shift.1;
        }
    }

    pub fn flatten(&self) -> Vec<(ArmNodeID, usize)> {
        let tree = self.tree.iter().collect::<Vec<_>>();
        let mut flat_tree = vec![];
        for (parent, children) in tree {
            for (child, length) in children {
                flat_tree.push((*parent, *child, *length));
            }
        }
        flat_tree.sort_by_key(|x| x.1);
        flat_tree
            .into_iter()
            .map(|(parent, _, length)| (parent, length))
            .collect()
    }

    // 指定したノードがどの方向から繋がっているかを返す
    pub fn direction(&self, node_id: ArmNodeID) -> Direction {
        let parent = self.tree_rev[&node_id];
        let parent_pos = self.tree_pos[&parent];
        let node_pos = self.tree_pos[&node_id];
        let dx = node_pos.0 - parent_pos.0;
        let dy = node_pos.1 - parent_pos.1;
        if dx == 0 && dy > 0 {
            Direction::Right
        } else if dx == 0 && dy < 0 {
            Direction::Left
        } else if dx > 0 && dy == 0 {
            Direction::Down
        } else if dx < 0 && dy == 0 {
            Direction::Up
        } else {
            panic!("Invalid direction: {:?}", (dx, dy));
        }
    }

    pub fn show_info(&self) {
        let tab = "    ";
        let child_middle = "├── ";
        let child_end = "└── ";
        let mut q = VecDeque::new();
        q.push_back((ROOT_ID, 0));
        eprintln!("{}", ROOT_ID.0);
        while let Some((node_id, depth)) = q.pop_front() {
            let children = self.tree.get(&node_id);
            if let Some(children) = children {
                for (i, (child, length)) in children.iter().enumerate() {
                    let child_str = if i == children.len() - 1 {
                        child_end
                    } else {
                        child_middle
                    };
                    eprintln!("{}{}{}: {}", tab.repeat(depth), child_str, child.0, *length);
                    q.push_back((*child, depth + 1));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arm_tree() {
        let mut arm_tree = ArmTree::new((0, 0));
        let a = arm_tree.add_arm(ArmNodeID(0), 1);
        assert_eq!(a, ArmNodeID(1));
        assert_eq!(arm_tree.tree_pos[&a], (0, 1));
        let b = arm_tree.add_arm(ArmNodeID(1), 2);
        assert_eq!(b, ArmNodeID(2));
        assert_eq!(arm_tree.tree_pos[&b], (0, 3));

        arm_tree.rotate(ArmNodeID(2), Rotate::Left);
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(0)], (0, 0));
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(1)], (0, 1));
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(2)], (-2, 1));

        arm_tree.rotate(ArmNodeID(1), Rotate::Right);
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(0)], (0, 0));
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(1)], (1, 0));
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(2)], (1, 2));

        let tree = arm_tree.flatten();
        assert_eq!(tree, vec![(ArmNodeID(0), 1), (ArmNodeID(1), 2)]);

        arm_tree.all_shift((1, 2));
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(0)], (1, 2));
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(1)], (2, 2));
        assert_eq!(arm_tree.tree_pos[&ArmNodeID(2)], (2, 4));
    }
}
