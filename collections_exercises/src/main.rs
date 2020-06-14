use std::collections::HashMap;

fn main() {
    let mut integers = vec![33, 55, 129, 22, 12, 892, 11, 33, 4, 4, 4];
    let length = integers.len();
    // 1

    //THE MEAN
    let mut total = 0;
    for num in &integers {
        total += num
    }
    println!("the mean is {}", total / length);

    //THE MEDIAN

    integers.sort();
    let middle_num = length / 2;
    println!("{:?}", integers);
    println!("{:?}", integers[middle_num]);

    //THE MODE

    let mut map = HashMap::new();

    for num in integers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max_value = HashMap::new();
    max_value.insert(String::from("max"), 0);
    max_value.insert(String::from("max_int"), 0);
    for (key, value) in map {
        if value > max_value["max"] {
            max_value.insert(String::from("max"), value);
            max_value.insert(String::from("max_int"), key);
        }
    }

    println!("mode int: {:?}", max_value["max_int"]);

    //2
}
