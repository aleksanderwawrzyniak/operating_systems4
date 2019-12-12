pub fn split_references(references: &str) -> Vec<u64> {
    references
        .trim()
        .split_whitespace()
        .filter_map(|c| c.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}

/// Returns true if finds the searched in collection, otherwise false
///
/// # Arguments
///
/// * `searched` - A 64 bit unsigned value you look for
/// * `collection` - A collection of 64 bit unsigned values to search through
pub fn does_contain(searched: u64, collection: &[u64]) -> bool {
    let mut out = false;

    for value in collection {
        if *value == searched {
            out = true;
            break;
        }
    }

    out
}
