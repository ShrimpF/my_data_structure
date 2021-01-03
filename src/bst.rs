use std::fmt::Debug;

/*
    Binary Search Tree
    - value: T
    - left: Box of BST, left.value < parent's value
    - right: Box of BST, right.value > parent's value
*/
#[derive(Debug, PartialEq)]
pub struct BST<T: Ord + Clone + Debug> {
    pub value: T,
    size: usize,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>,
}

pub fn new<T: Ord + Clone + Debug>(root: T) -> BST<T> {
    BST::<T> {
        value: root,
        size: 1,
        left: None,
        right: None,
    }
}

impl<T: Ord + Clone + Debug> BST<T> {
    pub fn search(&self, key: T) -> Option<T> {
        match (self.value.clone(), self.left.as_ref(), self.right.as_ref()) {
            (v, _, _) if v == key => Some(self.value.clone()),
            (v, Some(l), _) if v > key => l.search(key),
            (v, _, Some(r)) if v < key => r.search(key),
            _ => None,
        }
    }

    pub fn insert(&mut self, key: T) {
        self.size += 1;
        match (self.value.clone(), self.left.as_mut(), self.right.as_mut()) {
            (v, Some(l), _) if v >= key => l.insert(key),
            (v, None, _) if v >= key => self.left = Some(Box::new(new(key))),
            (v, _, Some(r)) if v < key => r.insert(key),
            (v, _, None) if v < key => self.right = Some(Box::new(new(key))),
            _ => return,
        }
    }

    pub fn min(&self) -> T {
        match self.left.as_ref() {
            Some(l) => l.min(),
            None => self.value.clone(),
        }
    }

    pub fn max(&self) -> T {
        match self.right.as_ref() {
            Some(r) => r.max(),
            None => self.value.clone(),
        }
    }

    pub fn pred(&self, key: T) -> Option<T> {
        self._pred(key, self, true)
    }

    fn _pred<'a>(&'a self, key: T, parent: &'a BST<T>, is_first: bool) -> Option<T> {
        match (self.value.clone(), self.left.as_ref(), self.right.as_ref()) {
            (v, Some(l), _) if v == key => Some(l.max()),
            (v, Some(l), _) if v > key => {
                if is_first {
                    l._pred(key, self, false)
                } else {
                    l._pred(key, parent, is_first)
                }
            }
            (v, None, _) if v == key => {
                if parent.value < key {
                    Some(parent.value.clone())
                } else {
                    None
                }
            }
            (v, _, Some(r)) if v < key => r._pred(key, self, is_first),
            _ => None,
        }
    }

    pub fn succ(&self, key: T) -> Option<T> {
        self._succ(key, self, true)
    }

    fn _succ<'a>(&'a self, key: T, parent: &'a BST<T>, is_first: bool) -> Option<T> {
        match (self.value.clone(), self.left.as_ref(), self.right.as_ref()) {
            (v, _, Some(r)) if v == key => Some(r.min()),
            (v, _, Some(r)) if v < key => {
                if is_first {
                    r._succ(key, self, false)
                } else {
                    r._succ(key, parent, is_first)
                }
            }
            (v, _, None) if v == key => {
                if parent.value > key {
                    Some(parent.value.clone())
                } else {
                    None
                }
            }
            (v, Some(l), _) if v > key => l._succ(key, self, is_first),
            _ => None,
        }
    }

    pub fn inorder(&self) -> Vec<T> {
        let mut v = Vec::<T>::new();
        self._inorder(&mut v);
        return v;
    }

    fn _inorder(&self, vec: &mut Vec<T>) {
        if let Some(l) = &self.left {
            l._inorder(vec)
        };
        vec.push(self.value.clone());
        if let Some(r) = &self.right {
            r._inorder(vec)
        };
    }

    pub fn select(&self, i: usize) -> Option<T> {
        Some(self._select(i)?.value.clone())
    }

    fn _select(&self, i: usize) -> Option<&BST<T>> {
        if i < 1 || i > self.size {
            return None;
        }

        let a = if let Some(l) = &self.left { l.size } else { 0 };
        match a {
            v if v == i - 1 => Some(self),
            v if v > i - 1 => self.left.as_ref()?._select(i),
            v if v < i - 1 => self.right.as_ref()?._select(i - 1 - v),
            _ => None,
        }
    }
}
