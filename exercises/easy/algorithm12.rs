/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.
    
    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/

use std::fmt::{self, Display, Formatter};

fn clean_string(input: &str) -> String {
    input
        .chars() // 获取字符迭代器
        .filter(|c| c.is_alphabetic()) // 过滤掉非字母字符
        .map(|c| c.to_lowercase().next().unwrap()) // 转换为小写
        .collect() // 收集成一个新的 String
}

pub fn is_palindrome(s: String) -> bool {
    // TODO: Implement the logic to check if the string is a palindrome
    let a = clean_string(&s);
    let c:Vec<char> = a.chars().collect();// rust中字符串/char都无法索引，只能变成vector
    println!("{}", a);
    let mut l:usize = 0;
    let mut r = a.len()-1;
    println!("{}",r);
    
    while l!=r && l+1!=r { //考虑both奇数偶数
        if c[l]!=c[r]{
            println!("{}!={}",c[l],c[r]);
            return false;
        }
        l+=1;
        r-=1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}
