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
}

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
