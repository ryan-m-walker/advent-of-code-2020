use std::fs;

const TREE: char = '#';

type Map = Vec<Vec<char>>;
type Slope = (usize, usize);

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let map: Map = input.split("\n").map(|x| x.chars().collect()).collect();

    let answer1 = get_trees_encountered(&map, (3, 1));
    println!("Answer 1: {}", answer1);

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let answer2 = get_trees_encountered_product(&map, slopes);
    println!("Answer 2: {}", answer2);
}

fn get_trees_encountered(map: &Map, slope: Slope) -> u64 {
    let (x_slope, y_slope) = slope;
    let mut trees_encountered_count = 0;
    let mut i: usize = 0;

    loop {
        let x = (i * x_slope as usize) % map[0].len();
        let y = i * y_slope as usize;

        if y >= map.len() {
            return trees_encountered_count;
        }

        let space = map[y][x];
        if space == TREE {
            trees_encountered_count += 1;
        }

        i += 1;
    }
}

fn get_trees_encountered_product(map: &Map, slopes: Vec<Slope>) -> u64 {
    let mut tree_encounters = vec![];

    for slope in slopes {
        tree_encounters.push(get_trees_encountered(&map, slope));
    }

    Iterator::product(tree_encounters.iter())
}
