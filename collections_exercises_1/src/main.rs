use std::collections::HashMap;

fn main() {
    let mut integers: Vec<i32> = vec![33, 55, 129, 22, 12, 50, 892, 11, 33, 4, 4, 4];
    let length = integers.len() as i32;
    // 1

    //THE MEAN
    let mut total = 0;
    for num in &integers {
        total += num
    }
    println!("the mean is {}", total / length);

    //alt solution
    fn find_mean(numbers: &Vec<i32>) -> f32 {
        let sum: i32 = numbers.iter().sum();
        sum as f32 / numbers.len() as f32
    }

    println!("the mean in 2nd func: {}", find_mean(&integers));

    //THE MEDIAN

    integers.sort();
    let middle_num: usize = length as usize / 2;
    println!("{:?}", integers);
    println!("{:?}", integers[middle_num]);

    //alt solution (will divide middle numbers in even list)

    fn median(ints: &mut Vec<i32>) -> i32 {
        ints.sort();
        let mid_num = ints.len() / 2;
        if ints.len() % 2 == 0 {
            find_mean(&vec![ints[mid_num - 1], ints[mid_num]]) as i32
        } else {
            ints[mid_num] as i32
        }
    };

    println!("alt solution for median: {}", median(&mut integers));

    //THE MODE

    let mut map = HashMap::new();

    let max = String::from("max");
    let max_int = String::from("max_int");

    for num in &integers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max_value = HashMap::new();
    max_value.insert(&max, 0);
    max_value.insert(&max_int, 0);
    for (&key, value) in map {
        if value > max_value[&max] {
            max_value.insert(&max, value);
            max_value.insert(&max_int, key);
        }
    }

    println!("mode int: {:?}", max_value[&max_int]);

    //alt Mode solution

    fn find_mode(ints: &Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in ints {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }
        let max_value = map.values().cloned().max().unwrap_or(0);

        map.into_iter()
            .filter(|&(_, v)| v == max_value)
            .map(|(&k, _)| k)
            .collect()
    }

    println!("find_mode: {:?}", find_mode(&integers));
}
