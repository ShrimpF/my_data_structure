use my_data_structure::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut bst: bst::BST<i32> = bst::new(0);
    let mut sum = 0;

    let file = File::open("./median.txt").unwrap();
    let reader = BufReader::new(file);
    for tuple in reader.lines().enumerate() {
        match tuple {
            (0, result) => {
                let value = result.unwrap().parse::<i32>().unwrap();
                bst.value = value;
                sum = value;
            }
            (k, result) => {
                let value = result.unwrap().parse::<i32>().unwrap();
                bst.insert(value);
                let mid = (k + 2) / 2;
                sum += bst.select(mid).unwrap();
            }
        }
    }

    println!("{}", sum);
    println!("{}", sum % 10000);
}
