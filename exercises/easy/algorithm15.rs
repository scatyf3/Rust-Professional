/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters. 
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.
    
    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::{collections::{HashMap, HashSet}, fmt::{self, Display, Formatter}};

// 这题有难度...应该是滑动窗口而不是dp
pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // TODO: Implement the logic to find the longest substring without repeating characters
    let mut char_set: HashSet<char> = HashSet::new();
    let (mut left, mut max_length) = (0, 0);

    for right in 0..s.len() {
        let current_char = s.chars().nth(right).unwrap();
        // 当有重复字符时，移动左指针
        while char_set.contains(&current_char) {
            char_set.remove(&s.chars().nth(left).unwrap());
            left += 1;
        }
        char_set.insert(current_char);
        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1);  // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0);  // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5);  // "abcde"
    }
}
