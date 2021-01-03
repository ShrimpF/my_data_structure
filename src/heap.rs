use std::*;

#[derive(Debug, PartialEq)]
pub struct Heap<T: PartialOrd + Clone> {
    a: Vec<T>,
}

pub fn new<T: PartialOrd + Clone>(a: &mut Vec<T>) -> Heap<T> {
    build(a);
    Heap { a: a.to_vec() }
}

fn build<T: PartialOrd + Clone>(a: &mut Vec<T>) {
    for i in (0..(a.len() / 2)).rev() {
        println!("{}", i);
        heapify(a, i)
    }
}

fn heapify<T: PartialOrd + Clone>(a: &mut Vec<T>, i: usize) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut smallest = i;

    if left < a.len() && a[i] > a[left] {
        smallest = left;
    }

    if right < a.len() && a[left] > a[right] {
        smallest = right;
    }

    if smallest != i {
        a.swap(i, smallest);
        heapify(a, smallest);
    }
}

impl<T: PartialOrd + Clone> Heap<T> {
    pub fn insert(&mut self, v: T) {
        self.a.push(v);
        let mut i = 0;
        while self.a[i / 2] > self.a[i] {
            self.a.swap(i / 2, i);
            i = i / 2;
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.a.len() == 0 {
            return Option::None;
        };
        let res = self.a.first().cloned();
        let last = self.a.len() - 1;

        self.a.swap(0, last);
        self.a.pop();

        heapify(&mut self.a, 0);

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_new {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(new(input), expected);
            }
        )*
        }
    }

    test_new! {
        heap_new_1: (&mut vec![] , Heap::<i32>{a: vec![]}),
        heap_new_2: (&mut vec![1] , Heap::<i32>{a: vec![1]}),
        heap_new_3: (&mut vec![1, 2, 3, 4] , Heap::<i32>{a: vec![1, 2, 3, 4]}),
        heap_new_4: (&mut vec![1, 4, 3, 2] , Heap::<i32>{a: vec![1, 2, 3, 4]}),
        heap_new_5: (&mut vec![5, 4, 3, 2, 1] , Heap::<i32>{a: vec![1, 2, 3, 5, 4]}),
    }
}
