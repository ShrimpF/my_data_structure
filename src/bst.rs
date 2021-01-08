use std::fmt::Debug;
use std::*;
/*
    Binary Search Tree
    - value: T
    - left: Box of BST, left.value < parent's value
    - right: Box of BST, right.value > parent's value
*/
#[derive(Debug, PartialEq, Clone)]
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

    pub fn delete(&mut self, key: T) {
        if self.size == 1 {
            return;
        }
        self._delete(key);
    }

    fn _delete(&mut self, key: T) {
        self.size -= 1;
        match (self.value.clone(), self.left.as_mut(), self.right.as_mut()) {
            (v, Some(l), _) if v > key => {
                match (l.value.clone(), l.left.as_mut(), l.right.as_mut()) {
                    (k, None, None) if k == key => self.left = None,
                    (k, Some(ll), None) if k == key => self.left = Some(ll.clone()),
                    (k, None, Some(r)) if k == key => self.left = Some(r.clone()),
                    (k, Some(ll), Some(_r)) if k == key => {
                        // l と lのpred を swap
                        mem::swap(&mut l.value, &mut ll.max_tree().value);
                        l.size -= 1;
                        if ll.value == key {
                            l.left = None;
                        } else {
                            ll._delete(key);
                        }
                    }
                    _ => l._delete(key),
                }
            }
            (v, _, Some(r)) if v < key => {
                match (r.value.clone(), r.left.as_mut(), r.right.as_mut()) {
                    (k, None, None) if k == key => self.right = None,
                    (k, Some(l), None) if k == key => self.right = Some(l.clone()),
                    (k, None, Some(rr)) if k == key => self.right = Some(rr.clone()),
                    (k, Some(l), Some(_r)) if k == key => {
                        // rと rのpred を swap
                        mem::swap(&mut r.value, &mut l.max_tree().value);
                        if l.value == key {
                            r.size -= 1;
                            r.left = None;
                            return;
                        }

                        l._delete(key);
                    }
                    _ => r._delete(key),
                }
            }
            _ => return,
        }
    }

    fn max_tree(&mut self) -> &mut BST<T> {
        match self.right.as_ref() {
            Some(_) => self.right.as_mut().unwrap().max_tree(),
            _ => self,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_test1() {
        let mut bst = new(10);
        bst.insert(9);
        bst.delete(9);
        assert_eq!(bst, new(10));
    }

    #[test]
    fn delete_test2() {
        let mut bst = new(10);
        bst.insert(11);
        bst.delete(11);
        assert_eq!(bst, new(10));
    }

    #[test]
    fn delete_test3() {
        let mut bst = new(10);
        bst.insert(9);
        bst.insert(8);
        bst.delete(9);

        let expected = || {
            let mut bst = new(10);
            bst.insert(8);
            bst
        };

        assert_eq!(bst, expected());
    }

    #[test]
    fn delete_test4() {
        let mut bst = new(10);
        bst.insert(11);
        bst.insert(12);
        bst.delete(11);

        let expected = || {
            let mut bst = new(10);
            bst.insert(12);
            bst
        };

        assert_eq!(bst, expected());
    }

    #[test]
    fn delete_test5() {
        let mut bst = new(10);
        bst.insert(5);
        bst.insert(7);
        bst.insert(3);
        bst.delete(5);

        let expected = || {
            let mut bst = new(10);
            bst.insert(3);
            bst.insert(7);
            bst
        };

        assert_eq!(bst, expected());
    }

    #[test]
    fn delete_test6() {
        let mut bst = new(10);
        bst.insert(5);
        bst.insert(4);
        bst.insert(7);
        bst.insert(8);
        bst.delete(5);

        let expected = || {
            let mut bst = new(10);
            bst.insert(4);
            bst.insert(7);
            bst.insert(8);
            bst
        };

        assert_eq!(bst, expected());
    }

    #[test]
    fn delete_test7() {
        let mut bst = new(10);
        bst.insert(5);
        bst.insert(6);
        bst.insert(2);
        bst.insert(3);
        bst.insert(4);
        bst.delete(5);

        let expected = || {
            let mut bst = new(10);
            bst.insert(4);
            bst.insert(6);
            bst.insert(2);
            bst.insert(3);
            bst
        };

        assert_eq!(bst, expected());
    }
}
