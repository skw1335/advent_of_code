mod data;

fn main() {
    let mut result = 0;

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in data::INPUT_STRING.trim().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            // Convert the string slices into integers and push them into separate vectors
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                vec1.push(num1);
                vec2.push(num2);
            }
        }
    }
    vec1.sort();
    vec2.sort();

    // part1
    for (x, y) in vec1.iter().zip(vec2.iter()) {
        result += (x - y).abs();
    }

    let testvec1 = vec![1, 2, 3, 4, 5];
    let testvec2 = vec![1, 1, 3, 5, 7];
    let mut res_holder: Vec<i32> = Vec::new();

    for x in &vec1 {
        let mut count = 0;
        for y in &vec2 {
            if x == y {
                count += 1;
            }
        }
        res_holder.push(count)
    }

    let mut diff = 0;
    //elem1 = every elem in vec1, elem2, every elem in res_holder
    for (elem1, elem2) in vec1.iter().zip(res_holder.iter()) {
        diff += elem1 * elem2
    }

    println!("diff {:?}", diff);
    println!("res_holder {:?}", res_holder);
    println!("result {:?}", result);
}
