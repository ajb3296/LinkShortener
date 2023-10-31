pub fn base10_to_base62(num: u128) -> String {
    let mut num = num;
    let mut result = String::new();
    let base62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    
    let base62: Vec<char> = base62.chars().collect();

    while num > 0 {
        let remainder = num % 62;
        result.push(base62[remainder as usize]);
        num /= 62;
    }

    result
}

pub fn base62_to_base10(num: String) -> u128 {
    let mut num = num;
    let mut result = 0;
    let base62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    
    let base62: Vec<char> = base62.chars().collect();

    while num.len() > 0 {
        let remainder = num.pop().unwrap();
        let index = base62.iter().position(|&r| r == remainder).unwrap();
        result += index as u128 * 62u128.pow(num.len() as u32);
    }

    result
}