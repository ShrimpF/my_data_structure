use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Bound::Included;
use std::path::Path;

use my_data_structure::list::*;
fn main() {
    let mut list = List::new();
    list.push(10);
    list.push(20);
    list.push(5);
    println!("{:?}", list.pop());
}

#[allow(dead_code)]
fn two_sum_solution() {
    let lines = read_lines("./two-sum.txt")
        .expect("fail to unwrap file")
        .map(|line| {
            line.expect("fail to uwrap line")
                .parse::<i64>()
                .expect("fail to parse")
        });

    let mut btree = BTreeSet::new();
    for i in lines {
        btree.insert(i);
    }

    println!("{}", btree.len());

    let mut set = HashSet::new();

    for x in &btree {
        for y in btree.range((Included(-10000 - x), Included(10000 - x))) {
            if x != y {
                let t = *x + *y;
                set.insert(t);
                println!("current = {}", set.len());
            }
        }
    }

    println!("count = {}", set.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
