use std::fs::read_to_string;

const INPUT_FILE: &str = "/home/radams/Projects/Advent-of-code-2024/day-1-puzzle/input.txt";

// Day 1 Challenge
// Given an input file of the format:
// 3  2
// 4 7 ...
// x y ...
// Pair up the smallest number on the left with the smallest and the right
// and continue (i.e second smallest with second smallest, ascending)
// and calculate the difference between each number, and come up with the 
// total distance between both lists.

// To do this, I need to know how:
//     to read from a file
//     basic control structure (if, for-loop)
//     Built-in sorting in Rust? Good practice to make a common sorting algorithm from scratch
//     General Rust Syntax (implicit function returns, etc.)
fn main() {
    let ( mut vector1, mut vector2): (Vec<i32>, Vec<i32>) = read_input_file(INPUT_FILE);

    // sort arrays
    vector1.sort();
    vector2.sort();

    // loop through array to calculate total distance between arrays
    let mut total_dist: i32 = 0;
    for x in 0..vector1.len() {
        let curr_element1: i32 = vector1[x];
        let curr_element2: i32 = vector2[x];

        total_dist += ( curr_element1 - curr_element2 ).abs();
    }

    println!("Total dist between the two lists: {}", total_dist);
}


// Function to read the input file into an array/vector, can it return the arrays/vectors? Implicit return would suggest yes
fn read_input_file(file_name: &str) -> (Vec<i32>, Vec<i32>) {

    let mut vector1: Vec<i32> = Vec::new();
    let mut vector2: Vec<i32> = Vec::new();

    println!("Received file name: {file_name}!");

    // read line-by-line and push to respective vectors
    for line in read_to_string(file_name).unwrap().lines() {
        let line_str: String = line.to_string();
        let mut curr_value = line_str.split_whitespace();
        let value1: i32 = curr_value.next().expect("Invalid string")
        .parse::<i32>().expect("Could not convert String to i32");
        vector1.push( value1 );
        //println!("value1: {}", value1);

        let value2: i32 = curr_value.next().expect("Invalid string")
        .parse::<i32>().expect("Could not convert String to i32");
        vector2.push(value2);
        //println!("value2: {}", value2);
    }

    // implicitly returned
    (vector1, vector2)
}
