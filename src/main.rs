mod boolean_search;
mod bigram_search;

use std::path::Path;

fn main() {
    let path = Path::new("Engines");
    // boolean_search::run_boolean_search_engine(&path);
    bigram_search::run_boolean_bigram_search_engine(&path);
}