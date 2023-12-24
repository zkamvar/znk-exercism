use std::collections::HashMap;
use std::vec::Vec;

pub fn brackets_are_balanced(string: &str) -> bool {
    // Create a character vector that will contain the braces we see
    let mut braces: Vec<char> = Vec::new();
    // define the pairs of braces we understand
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    // split the characters and loop over them, keeping the balance.
    let chars = string.chars();
    for this_char in chars {
        // balance populates and trims the stack. It will return false if an
        // unmatched closing brace exists, so we return early.
        let good: bool = balance(&mut braces, &pairs, this_char);
        if !good {
            return false;
        }
    }
    // If we get here, we should confirm that the balance is correct.
    return braces.len() == 0;
}

// Balance the stack of braces
//
// I used the introduction to
// https://www.geeksforgeeks.org/check-for-balanced-parentheses-in-an-expression/
// to help me with this solution, and then I looked up information on how to
// work with stacks in Rust.
//
// This function will only record opening braces on the stack, removing them
// when the matched closing brace is encountered. If an unmatched closing brace
// is encountered, the function will return false, indicating that it's
// unbalanced too early. Non-brace characters are passed through.
fn balance(braces: &mut Vec<char>, pairs: &HashMap<char, char>, this: char) -> bool {
    // get the last recorded element, which should be an opening brace
    let open: Option<&char> = braces.last();

    // MATCHING CLOSER: pop the stack
    // If we have something on the stack, check the known pairs to see if this
    // is a closing brace (using a lambda with the is_some_and() method).
    if open.is_some_and(|x| pairs.get(x) == Some(&this)) {
        braces.pop();
        return true;
    }

    // we did not find a matching closer, so we need to check the next options
    match this {
        // OPENER: push the stack
        '[' | '(' | '{' => {
            braces.push(this);
            return true;
        }
        // UNMATCHED CLOSER: we are no longer valid
        ']' | ')' | '}' => return false,
        // NON-BRACE: pass through
        _ => return true,
    }
}
