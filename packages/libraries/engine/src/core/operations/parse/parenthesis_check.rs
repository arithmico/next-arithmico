use std::collections::HashMap;

use crate::core::ParseNodeError;

pub fn check_missing_open_parenthesis(
    input: &str,
) -> Result<(), ParseNodeError> {
    let mut counts = HashMap::<char, Vec<usize>>::new();

    input.chars().enumerate().for_each(|(pos, c)| match c {
        '(' | ')' | '[' | ']' => {
            if let Some(p) = counts.get_mut(&c) {
                p.push(pos);
            } else {
                counts.insert(c, vec![pos]);
            }
        }
        _ => (),
    });

    let c_1_1 = counts.get(&'(').map(|v| v.len()).unwrap_or(0) as i64;
    let c_1_2 = counts.get(&')').map(|v| v.len()).unwrap_or(0) as i64;
    let delta_1 = c_1_1 - c_1_2;

    let c_2_1 = counts.get(&'[').map(|v| v.len()).unwrap_or(0) as i64;
    let c_2_2 = counts.get(&']').map(|v| v.len()).unwrap_or(0) as i64;
    let delta_2 = c_2_1 - c_2_2;

    if delta_1 < 0 || delta_2 < 0 {
        Err(ParseNodeError::MissingOpeningParenthesis {
            round: delta_1.abs() as usize,
            square: delta_2.abs() as usize,
        })
    } else {
        Ok(())
    }
}
