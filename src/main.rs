struct Solution;

impl Solution {
    fn is_pair_of_ordered_brackets(bracket1: char, bracket2: char) -> bool {
        match bracket1 {
            '{' => bracket2 == '}',
            '[' => bracket2 == ']',
            '(' => bracket2 == ')',
            _ => false,
        }
    }
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            if let Some(&b) = stack.last() {
                if Self::is_pair_of_ordered_brackets(b as char, c as char) {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }
        stack.is_empty()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid(String::from("({}[])")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
}

#[test]
fn test_foo() {
    assert_eq!(Solution::is_valid(String::from("({}[])")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
}