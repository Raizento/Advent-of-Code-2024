use std::collections::HashMap;

/// Counts how often a number appears in a list.
/// Returns a map with the number as the key and the number of occurences as the value.
/// Example:
/// occurence_map(vec![1, 2, 2, 2, 3]) -> {(1, 1), (2, 3), (3, 1)}
pub fn occurence_map(list: &[i32]) -> HashMap<i32, i32> {
    let mut occurence_map: HashMap<i32, i32> = HashMap::new();
    for number in list {
        match occurence_map.get(number) {
            Some(value) => occurence_map.insert(*number, value + 1),
            None => occurence_map.insert(*number, 1),
        };
    }

    occurence_map
}

pub fn similarity(left: &[i32], right: &[i32]) -> i32 {
    let occurence_map = occurence_map(right);
    
    let similarity: i32 = left.iter().map(|number: &i32| {
        let multiplier = occurence_map.get(number).unwrap_or(&0);
        number * multiplier
    }).sum();

    similarity
}
