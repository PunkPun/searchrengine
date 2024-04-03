mod boolean_search;

use std::path::Path;

fn main() {
    let path = Path::new("Engines");
    boolean_search::run_boolean_search_engine(&path);
}