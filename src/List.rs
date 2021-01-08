#[derive(Debug, PartialEq, Clone)]
pub struct List<T: PartialEq + Clone> {
    value: T,
    next: Option<Box<List<T>>>,
}

impl<T: PartialEq + Clone> List<T> {
    pub fn new(value: T) -> List<T> {
        List { value, next: None }
    }

    pub fn prepend(&self, value: T) -> List<T> {
        List {
            value,
            next: Some(Box::new(self.clone())),
        }
    }

    pub fn append(&mut self, value: T) {
        match self.next.as_mut() {
            Some(n) => n.append(value),
            None => self.next = Some(Box::new(List::new(value))),
        }
    }

    pub fn find(&self, value: &T) -> bool {
        match (&self.value, &self.next) {
            (v, _) if v == value => true,
            (v, Some(ref next)) => next.find(v),
            (_, _) => false,
        }
    }
}
