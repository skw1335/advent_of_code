mod data;

fn parse_data(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty()) // Skip empty lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(list: &[i32]) -> bool {
    list.windows(2).all(|window| {
        let diff = (window[0] - window[1]).abs();
        diff <= 3 && diff >= 1
    })
}

fn is_strictly_monotonic(lst: &[i32]) -> bool {
    lst.windows(2).all(|window| window[0] < window[1])
        || lst.windows(2).all(|window| window[0] > window[1])
}

fn main() {
    let lst = vec![1, 2, 7, 4]; // Example list
    println!("{}", is_safe(&lst)); // Output: false

    let lst2 = vec![40, 3, 2, 1]; // Example list
    println!("{}", is_safe(&lst2)); // Output: false

    let lst3 = vec![1, 2, 2, 4]; // Example list
    println!("{}", is_safe(&lst3)); // Output: false

    let data = parse_data(data::INPUT_STRING);
    let mut not_safe: Vec<Vec<i32>> = Vec::new();
    let mut test_not_safe: Vec<Vec<i32>> = Vec::new();
    test_not_safe.push(lst);
    test_not_safe.push(lst2);
    test_not_safe.push(lst3);

    //Part 1

    let mut count = 0;
    let mut data_count = 0;
    for row in &data {
        if is_strictly_monotonic(row) && is_safe(row) {
            count += 1;
            println!("Row is safe")
        } else if !is_strictly_monotonic(row) || !is_safe(row) {
            not_safe.push(row.clone());
            println!("Not Safe!!!")
        }
    }

    //Part 2
    for line in &not_safe {
        data_count += 1;
    }

    println!("not safe.len() {:?}", &not_safe.len());

    let mut part_2_count = 0;
    for elem in not_safe {
        for i in 0..elem.len() {
            let temp_list: Vec<i32> = elem
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &value)| value)
                .collect();

            if is_strictly_monotonic(&temp_list) && is_safe(&temp_list) {
                part_2_count += 1;
                println!("safe temp list: {:?}", &temp_list);
                break;
            }
        }
    }

    println!("part_2_count: {:?}", part_2_count);
    println!("amount of lines in data: {:?}", data_count);
    println!("part_1_count: {:?}", count);

    let part_2_answer = count + part_2_count;
    println!("part_2_answer: {:?}," part_2_answer);
}
