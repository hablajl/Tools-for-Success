// FILE: tools_for_success.rs

// 1
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

// 2
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

// 3
fn is_palindrome(s: &str) -> bool {
    let rev = reverse(s);
    s == rev
}

// 4
fn find_max(data: &[i32]) -> Option<&i32> {
    let mut max = None;
    for el in data {
        match max {
            None => max = Some(el),
            Some(y) if y < el => max = Some(el),
            _ => (),
        }
    }
    max
}

// 5
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

// 6
fn contains_vowel(s: &str) -> bool {
    let vowels = ["a", "e", "i", "o", "u"];
    s.to_lowercase().chars().any(|c| vowels.contains(&c.to_string().as_str()))
}

// 7
fn gcd(mut n1: u64, mut n2: u64) -> u64 {
    assert!(n1 != 0 && n2 != 0);
    while n2 != 0 {
        if n1 > n2 {
            n1 = n1 - n2;
        } else {
            n2 = n2 - n1;
        }
    }
    n1
}

// 8
fn encode(s: &str) -> String {
    let mut encoded = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        let shifted = if (i % 2 == 0) {
            c.to_string().chars().nth(1).unwrap()
        } else {
            c.to_string().chars().nth(0).unwrap()
        };
        encoded.push(shifted);
    }
    encoded
}

// 9
fn is_substring(s1: &str, s2: &str) -> bool {
    s2.contains(s1)
}

// 10
fn binary_search(data: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = data.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let mid_val = data[mid];

        if mid_val == target {
            return Some(mid);
        } else if target < mid_val {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}

// 11
fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut s1_chars: Vec<char> = s1.chars().collect();
    s1_chars.sort();
    let mut s2_chars: Vec<char> = s2.chars().collect();
    s2_chars.sort();
    s1_chars == s2_chars
}

// 12
fn max_occuring_char(s: &str) -> Option<char> {
    let mut max = None;
    let mut max_count = 0;
    let mut counts = [0; 256];
    for c in s.chars() {
        counts[c as usize] += 1;
        if counts[c as usize] > max_count {
            max = Some(c);
            max_count = counts[c as usize];
        }
    }
    max
}

// 13
fn is_ascending(data: &[i32]) -> bool {
    if data.len() <= 1 {
        return true;
    }
    let mut prev = data[0];
    for i in 1..data.len() {
        if data[i] < prev {
            return false;
        }
        prev = data[i];
    }
    true
}

// 14
fn is_pi_number(n: u32) -> bool {
    let pi = std::f64::consts::PI;
    (n as f64 - pi).abs() < 1e-10
}

// 15 
fn string_to_date(s: &str) -> Result<chrono::NaiveDate, chrono::ParseError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

// 16
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// 17
fn is_power_of_two(n: u64) -> bool {
    (n & (n - 1)) == 0
}

// 18
fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// 19
fn sqrt(n: f64) -> f64 {
    let mut x = 1.0;
    loop {
        let last_x = x;
        x = (x + (n / x)) / 2.0;
        if (x - last_x).abs() < 1e-10 {
            break;
        }
    }
    x
}

// 20
fn linear_search(data: &[i32], target: i32) -> Option<usize> {
    for (i, el) in data.iter().enumerate() {
        if *el == target {
            return Some(i);
        }
    }
    None
}

// 21
fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|s| capitalize(s))
        .collect::<Vec<String>>()
        .join(" ")
}

// 22 
fn sort_words_alphabetically(s: &str) -> String {
    let mut words = s.split_whitespace().collect::<Vec<&str>>();
    words.sort();
    words.join(" ")
}

// 23
fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

// 24
fn is_armstrong_number(n: u32) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let len = digits.len() as u32;
    let sum = digits
        .iter()
        .map(|d| d.pow(len))
        .sum::<u32>();
    n == sum
}

// 25
fn most_frequent(data: &[i32]) -> Option<i32> {
    let mut max_el = None;
    let mut max_count = 0;
    let mut counts = std::collections::HashMap::new();
    for el in data {
        let count = counts.entry(el).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_el = Some(*el);
            max_count = *count;
        }
    }
    max_el
}

// 26
fn double_characters(s: &str) -> String {
    s.chars()
        .map(|c| c.to_string().repeat(2))
        .collect::<Vec<String>>()
        .join("")
}

// 27
fn compute_median(data: &[i32]) -> f64 {
    let mut data_sorted = data.to_vec();
    data_sorted.sort();
    let len = data_sorted.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (data_sorted[mid - 1] + data_sorted[mid]) as f64 / 2.0
    } else {
        data_sorted[len / 2] as f64
    }
}

// 28
fn is_pangram(s: &str) -> bool {
    let mut letters = std::collections::HashSet::new();
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| {
            letters.insert(c);
        });
    letters.len() == 26
}

// 29
fn bubble_sort(data: &mut [i32]) {
    let len = data.len();
    for _ in 0..len {
        for j in 1..len {
            if data[j - 1] > data[j] {
                data.swap(j - 1, j);
            }
        }
    }
}

// 30
fn find_missing_number(data: &[u32]) -> Option<u32> {
    let mut missing = None;
    for i in 0..data.len() + 1 {
        if !data.contains(&i) {
            missing = Some(i);
            break;
        }
    }
    missing
}

// 31
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

// 32
fn generate_alphabet() -> String {
    (0..26)
        .map(|i| (i + 'a' as u8) as char)
        .collect::<Vec<char>>()
        .join("")
}

// 33
fn binary_string_to_int(s: &str) -> Option<u32> {
    let mut sum = 0;
    for (i, c) in s.chars().rev().enumerate() {
        if c == '1' {
            sum += 2_u32.pow(i as u32);
        } else if c != '0' {
            return None;
        }
    }
    Some(sum)
}

// 34
fn reverse_int(n: i32) -> i32 {
    n.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

// 35
fn map_characters_to_numbers(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

// 36
fn compute_average(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / (data.len() as f64)
}

// 37
fn convert_base(s: &str, from: u32, to: u32) -> Option<String> {
    let decimal = match u32::from_str_radix(s, from) {
        Ok(num) => num,
        Err(_) => return None,
    };
    Some(format!("{:X}", decimal.to_string(to)))
}

// 38
fn is_valid_email(s: &str) -> bool {
    let re = regex::Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$",
    )
    .unwrap();
    re.is_match(s)
}

// 39
fn count_substrings(s: &str, substr: &str) -> usize {
    s.matches(substr).count()
}

// 40
fn sort_by_length(data: &mut [&str]) {
    data.sort_by_key(|s| s.len());
}

// 41
fn compute_lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

// 42
fn remap_alphabet(s: &str) -> String {
    s.chars().map(|c| match c {
        'a'...'z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
        'A'...'Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
        _ => c,
    }).collect()
}

// 43
fn compute_gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

// 44
fn compute_hamming_distance(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars()).filter(|(x, y)| x != y).count()
}

// 45