use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn first_day() {
    match File::open("src/1.txt") {
        Ok(file) => {
            let mut data: String = String::new();
            let mut buf_reader = BufReader::new(file);
            match buf_reader.read_to_string(&mut data) {
                Ok(ok) => ok,
                Err(e) => {
                    eprintln!("{e}");
                    return;
                }
            };
            data = data.trim().to_string();
            let mut l1: Vec<i64> = Vec::new();
            let mut l2: Vec<i64> = Vec::new();
            for line in data.split('\n') {
                let numbers: Vec<String> = line.split_whitespace().map(String::from).collect();
                let num1 = match numbers[0].parse::<i64>() {
                    Ok(num) => num,
                    Err(_) => {
                        return;
                    }
                };
                let num2 = match numbers[1].parse::<i64>() {
                    Ok(num) => num,
                    Err(_) => {
                        return;
                    }
                };
                l1.push(num1);
                l2.push(num2);
            }

            l1.sort();
            l2.sort();

            let mut map1: HashMap<i64, i32> = HashMap::new();
            let mut map2: HashMap<i64, i32> = HashMap::new();
            let mut final_map: HashMap<i64, i32> = HashMap::new();

            for (n1) in l1.iter() {
                if let Some(val) = map1.get(n1) {
                    *map1.get_mut(n1).unwrap() = val + 1;
                } else {
                    map1.insert(*n1, 1);
                }
            }

            for (n2) in l2.iter() {
                if let Some(val) = map2.get(n2) {
                    *map2.get_mut(n2).unwrap() = val + 1;
                } else {
                    map2.insert(*n2, 1);
                }
            }

            for (k, v1) in &map1 {
                if let Some(v2) = map2.get(k) {
                    let v = match v1 > v2 {
                        true => v1,
                        false => v2,
                    };
                    final_map.insert(*k, *v);
                }
            }

            let mut result = 0;

            for (k, v) in &final_map {
                result += k * *v as i64;
            }

            println!("{:?}", final_map);

            println!("Result: {}", result);
            return;

            let mut result = 0;
            for i in 0..l1.len() {
                result += (l1[i] - l2[i]).abs();
            }
            println!("Result: {}", result);
            return;
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}

fn second_day() {
    let file = match File::open("src/2.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut data: String = String::new();
    let mut buf_reader = BufReader::new(file);
    let mut acc = 0;
    match buf_reader.read_to_string(&mut data) {
        Ok(_) => {
            data = data.trim().to_string();
            let lines: Vec<String> = data.split('\n').map(String::from).collect();

            for line in lines {
                let numbers: Vec<i32> = line
                    .split_whitespace()
                    .map(|v| v.parse::<i32>().expect("woopsie did not parse!"))
                    .collect();

                println!("{:?}", &numbers);

                let mut aux: Vec<i32> = numbers.clone();
                aux.sort();

                let mut aux_rev: Vec<i32> = aux.clone();
                aux_rev.reverse();

                let mut results: Vec<bool> = Vec::new();

                let n = numbers.clone();
                let mut result = true;

                let mut prev = n[0];
                for number in n[1..].iter() {
                    if number - prev >= 4 || number - prev < 1 {
                        result = false;
                        break;
                    }
                    prev = *number;
                }
                if !result {
                    let mut prev = n[0];
                    for number in n[1..].iter() {
                        if prev - number >= 4 || prev - number < 1 {
                            result = false;
                            break;
                        }
                        prev = *number;

                        if *number == aux_rev[aux_rev.len() - 1] {
                            result = true;
                        }
                    }
                }

                println!("none -> {}", result);
                results.push(result);

                for i in 0..(numbers.len()) {
                    let mut n = numbers.clone();
                    let mut result = true;
                    n.remove(i);
                    println!("removed: {} -> {:?}", numbers[i], n);

                    let mut prev = n[0];
                    for number in n[1..].iter() {
                        if number - prev >= 4 || number - prev < 1 {
                            result = false;
                            break;
                        }
                        prev = *number;
                    }
                    if !result {
                        let mut prev = n[0];
                        for number in n[1..].iter() {
                            if prev - number >= 4 || prev - number < 1 {
                                result = false;
                                break;
                            }
                            prev = *number;

                            if *number == n[n.len() - 1] {
                                result = true;
                            }
                        }
                    }

                    println!("{} -> {}", numbers[i], result);
                    results.push(result);
                }

                if results.contains(&true) {
                    acc += 1;
                    println!("adicionado");
                } else {
                    println!("não adicionado");
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    println!("{}", acc);
}

fn main() {
    second_day();
}
