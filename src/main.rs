use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use regex::Regex;

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

            for n1 in l1.iter() {
                if let Some(val) = map1.get(n1) {
                    *map1.get_mut(n1).unwrap() = val + 1;
                } else {
                    map1.insert(*n1, 1);
                }
            }

            for n2 in l2.iter() {
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

fn third_day() {
    let file = match File::open("src/3.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut data: String = String::new();
    let mut buf_reader = BufReader::new(file);
    match buf_reader.read_to_string(&mut data) {
        Ok(_) => {
            let re = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").expect("invalid re");
            let mul_sum: i32 = re
                .captures_iter(data.as_str())
                .map(|c| {
                    let a = c
                        .name("a")
                        .expect("invalid")
                        .as_str()
                        .parse::<i32>()
                        .expect("invalid");
                    let b = c
                        .name("b")
                        .expect("invalid")
                        .as_str()
                        .parse::<i32>()
                        .expect("invalid");
                    a * b
                })
                .sum();

            println!("{}", mul_sum);

            let mut enabled = true;

            let re = Regex::new(
                r"(mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\))|(?<d>do\(\))|(?<n>don't\(\))",
            )
            .expect("invalid re");

            let sum: i32 = re
                .captures_iter(data.as_str())
                .map(|captured| {
                    if let Some(_) = captured.name("d") {
                        enabled = true;
                        0
                    } else if let Some(_) = captured.name("n") {
                        enabled = false;
                        0
                    } else if enabled {
                        let a = captured
                            .name("a")
                            .expect("invalid a")
                            .as_str()
                            .parse::<i32>()
                            .expect("parsing a");

                        let b = captured
                            .name("b")
                            .expect("invalid b")
                            .as_str()
                            .parse::<i32>()
                            .expect("parsing b");

                        a * b
                    } else {
                        0
                    }
                })
                .sum();
            println!("{}", sum);
        }
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    }
}

fn fourth_day() {
    let directions = vec![
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];

    let mut file = File::open("src/4.txt").expect("should open!");
    let mut buf = String::new();

    file.read_to_string(&mut buf).expect("should read");

    let lines: Vec<String> = buf.trim().split_whitespace().map(String::from).collect();

    let m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    fn xmas(m: &Vec<Vec<char>>, x: usize, y: usize, dir: &(i32, i32)) -> bool {
        let mut x: i32 = x as i32;
        let mut y: i32 = y as i32;
        let (dir_x, dir_y) = dir;
        for ch in "XMAS".chars() {
            match m.get(x as usize) {
                Some(m) => match m.get(y as usize) {
                    Some(_) => {}
                    None => {
                        return false;
                    }
                },
                None => {
                    return false;
                }
            }
            if ch == m[x as usize][y as usize] {
                x += dir_x;
                y += dir_y;
            } else {
                return false;
            }
        }

        return true;
    }

    let mut count = 0;

    for line in 0..m.len() {
        for column in 0..m[line].len() {
            for dir in &directions {
                if xmas(&m, line, column, &dir) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn fourth_day_2() {
    let mut file = File::open("src/4.txt").expect("should open!");
    let mut buf = String::new();

    file.read_to_string(&mut buf).expect("should read");

    let lines: Vec<String> = buf.trim().split_whitespace().map(String::from).collect();

    let m: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    fn mas(m: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if m[x][y] != 'A' || x >= m.len() - 1 || y >= m[0].len() - 1 || x == 0 || y == 0 {
            return false;
        }

        let (d11, d12) = (m[x + 1][y + 1], m[x - 1][y - 1]);

        let (d21, d22) = (m[x + 1][y - 1], m[x - 1][y + 1]);

        match (d11, d12, d21, d22) {
            ('M', 'S', 'M', 'S')
            | ('M', 'S', 'S', 'M')
            | ('S', 'M', 'M', 'S')
            | ('S', 'M', 'S', 'M') => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    let mut count = 0;

    for line in 0..m.len() {
        for column in 0..m[line].len() {
            if mas(&m, line, column) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn main() {
    fourth_day_2();
}
