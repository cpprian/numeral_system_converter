use std::collections::HashMap;

fn r2a(s: &str, h: &HashMap<char, i32>) -> i32 {
    let mut result = 0;
    let mut prev = 0;
    let roman = s.chars().map(|c| h.get(&c).unwrap());

    for r in roman {
        result += r;
        if prev != 0 && r > &prev {
            result -= 2 * prev;
        }
        prev = *r;
    }
    return result;
}

fn a2r(n: i32, h: Vec<(&str, i32)>) -> String {
    let mut result = String::new();
    let mut n = n;

    while n > 0 {
        for k in h.iter() {
            if n >= k.1 {
                result.push_str(k.0);
                n -= k.1;
                break;
            }
        }
    }
    return result;
}

fn main() {
    let converter = HashMap::from([('I', 1), ('V', 5), ('X', 10),
                ('L', 50), ('C', 100), ('D', 500), ('M', 1000)]);

    let converter2 = Vec::from([("M", 1000), ("CM", 900), ("D", 500),
        ("CD", 400), ("C", 100), ("XC", 90), ("L", 50), ("XL", 40),
        ("X", 10), ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)]);

    println!("Roman {} to arabic {}", "MCM", r2a("MCM", &converter));
    println!("Arabic {} to roman {}", 1900, a2r(1900, converter2));
}