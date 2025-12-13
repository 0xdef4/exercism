use std::fmt::Debug;

#[derive(Debug, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    pub value: T,
    pub children: Vec<Tree<T>>,
}

impl<T: Debug + Ord> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.value != other.value || self.children.len() != other.children.len() {
            return false;
        }

        let mut matched = vec![false; other.children.len()];

        for child1 in &self.children {
            let mut found = false;

            for (i, child2) in other.children.iter().enumerate() {
                if !matched[i] && child1 == child2 {
                    matched[i] = true;
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        true
    }
}


impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            value: label,
            children: vec![],
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        let path = match self.find_path(from) {
            Some(p) => p,
            None => return false,
        };

        if path.is_empty() {
            return true;
        }

        // Start from the current root (mutable reference)
        let node: &mut Tree<T> = self;

        for idx in path {
            let mut child = node.children.remove(idx);
            std::mem::swap(node, &mut child);
            node.children.push(child);
        }

        true
    }

    fn find_path(&self, target: &T) -> Option<Vec<usize>> {
        let mut path = vec![];
        if dfs(self, target, &mut path) {
            return Some(path);
        } else {
            return None;
        }
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if !self.pov_from(from) {
            return None;
        }
        let path = self.find_path(to)?;
        Some(self.collect_path(&path))
    }

    /// Convert index path into &T path.
    fn collect_path<'a>(&'a self, idxs: &[usize]) -> Vec<&'a T> {
        let mut res = vec![&self.value];
        let mut node = self;
        for &i in idxs {
            node = &node.children[i];
            res.push(&node.value);
        }
        res
    }
}

pub fn dfs<T: Debug + Ord>(node: &Tree<T>, target: &T, path: &mut Vec<usize>) -> bool {
    if &node.value == target {
        return true;
    }
    for (i, child) in node.children.iter().enumerate() {
        path.push(i);
        if dfs(child, target, path) {
            return true;
        }
        path.pop();
    }
    false
}
