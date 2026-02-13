pub mod imports;
pub mod macros;
pub mod optimize;

use std::collections::HashSet;
use std::path::Path;

pub fn preprocess(code: &str, base_path: &Path) -> Result<String, String> {
    let mut processed = code.to_string();

    processed = imports::process_imports(&processed, base_path, &mut HashSet::new())?;

    processed = macros::process_macros(&processed)?;

    processed = optimize::optimize(&processed)?;

    Ok(processed)
}
