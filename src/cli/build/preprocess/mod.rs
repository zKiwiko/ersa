pub mod imports;
pub mod macros;
pub mod optimize;

use std::collections::HashSet;
use std::path::Path;

pub fn preprocess(code: &str, base_path: &Path) -> Result<String, String> {
    let mut processed = code.to_string();

    // Step 1: Process imports
    processed = imports::process_imports(&processed, base_path, &mut HashSet::new())?;

    // Step 2: Process macros
    processed = macros::process_macros(&processed)?;

    // Step 3: Optimize (constant folding, expression simplification)
    processed = optimize::optimize(&processed)?;

    Ok(processed)
}
