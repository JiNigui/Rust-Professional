use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let elements: Vec<&str> = input_str.split(',').collect();
    
    let mut unique_elements = HashSet::new();
    
    for element in elements {
        unique_elements.insert(element);
    }
    
    unique_elements.len()
}
