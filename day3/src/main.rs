mod data;
use regex::Regex;

fn main() {
    //println!("input string: {:?}", data::INPUT_STRING);

    let text = data::INPUT_STRING;
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();

    let matches: Vec<Vec<i32>> = re
        .captures_iter(text)
        .filter_map(|cap| {
            cap.get(1) // Get the inner string of mul(...)
                .map(|m| m.as_str()) // Get the match as a string
                .map(|nums| {
                    nums.split(',') // Split by commas
                        .filter_map(|n| n.trim().parse::<i32>().ok()) // Parse each number
                        .collect::<Vec<i32>>() // Collect into a Vec<i32>
                })
        })
        .collect(); // Collect the results into a vector of vectors
                    //just for testing purposes

    println!("Matches.len() {:?}", matches.len());
    println!("matches: {:?}", matches);
    let _vec_of_vecs: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];

    let answer: i32 = matches
        .iter() // iterate over every vector
        .map(|v| v.iter().product::<i32>())
        .sum(); // take the sum of the products

    println!("part 1 answer {:?}", answer)
}
