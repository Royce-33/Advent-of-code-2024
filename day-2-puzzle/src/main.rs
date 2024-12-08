use std::fs::read_to_string;

const INPUT_FILE: &str = "/home/radams/Projects/Advent-of-code-2024/day-2-puzzle/input.txt";
const SAMPLE_INPUT: &str = "/home/radams/Projects/Advent-of-code-2024/day-2-puzzle/sample_input.txt";
// Day 2 Challenge
// Given an input file of the format
// 7 6 4 2 1
// 1 2 7 8 9
// ...
// Each line is a report:
// Acceptable reports must be:
// - Either all increasing or decreasing
// - Any two adjacent numbers differ by at least one and at most three

// To do this I need to:
// Read in the input from the file, could make use of the line structure when parsing the file
// Iterate through each line, set prev number to first number
// continue looping through the line until we reach the end or violate one of our rules

fn main() {
    
    count_valid_reports(INPUT_FILE);

}

fn count_valid_reports( file_name: &str ) {
    let mut valid_count: i32 = 0;

    for line in read_to_string( file_name ).unwrap().lines() {
        let line_str: String = line.to_string();
        let mut increasing: bool = false;
        let mut element_count: i32 = 0;
        let mut prev_element: i32 = 0;
        let mut valid_report: bool = true;

        for element_str in line_str.split_whitespace() {
            println!("element_str: {}", element_str);
            let element: i32 = element_str.parse::<i32>().expect("Unable to convert string to i32");
            
             // calculate prev element - curr element, if positive - we are decreasing, if negative - we are increasing, the difference must be at least 1 but no greater than 3
            if element_count != 0 {
                let diff: i32 = prev_element - element;
                
                if element_count == 1 {
                    if diff < 0  {
                        increasing = true;
                        println!("Increasing");
                    }

                    else {
                        println!("Decreasing");
                    }
                }
                
                if increasing && diff > 0  {
                    valid_report = false;
                }

                if !increasing && diff < 0 {
                    valid_report = false;
                }


                if !( diff.abs() >= 1 && diff.abs() <= 3 ) {
                    valid_report = false;
                }
                
            }
            
            if !valid_report {
                break;
            }
            prev_element = element;
            element_count += 1; 
        }
        println!("Finished processing report!");

        if valid_report {
            valid_count += 1;
            println!("Report is valid!");
        } 
    }

    println!("Valid report count: {}", valid_count);
}