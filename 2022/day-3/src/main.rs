mod item;

use common::functions::{get_input_file_arg, read_input};
use item::Item;
use std::{collections::HashSet, hash::Hash, io};

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input(&f)?;

    let output1 = input
        .iter()
        .map(|bag| {
            let items = bag.chars().collect::<Vec<char>>();
            let (c1, c2) = items.split_at(items.len() / 2);

            let common = find_common(c1, c2);
            common
                .iter()
                .map(|e| Item::from_char(e.clone()).priority)
                .sum::<usize>()
        })
        .sum::<usize>();

    let output2 = input
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .map(|chunk| {
            if let [e1, e2, e3] = chunk {
                let common1 = find_common(e1, e2);
                let common = find_common(&common1, e3);
                return common
                    .iter()
                    .map(|e| Item::from_char(e.clone()).priority)
                    .sum::<usize>();
            } else {
                panic!("invalid input")
            };
        })
        .sum::<usize>();

    println!("Sum of priorities: {}", output1);
    println!("Sum of priorities (Groups of 3): {}", output2);
    Ok(())
}

fn find_common<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Eq + Ord + Hash + Clone,
{
    let mut common = Vec::new();

    let ahs = a
        .to_vec()
        .into_iter()
        .fold(HashSet::<T>::new(), |mut m, x| {
            m.insert(x);
            m
        });
    let bhs = b
        .to_vec()
        .into_iter()
        .fold(HashSet::<T>::new(), |mut m, x| {
            m.insert(x);
            m
        });

    ahs.iter().for_each(|e| match bhs.get(e) {
        Some(_) => {
            common.push(e.clone());
        }
        None => {}
    });
    common
}
