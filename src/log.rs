#![allow(dead_code)]

// Blue
#[inline]
pub fn info(msg: &str) {
    println!("[\x1b[1;36mINFO\x1b[0m] {}", msg);
}

#[inline]
pub fn vinfo(msg: &str) {
    if std::env::var("ERSA_VERBOSE").is_ok() {
        println!("[\x1b[1;36mINFO\x1b[0m] {}", msg);
    }
}

// Yellow
#[inline]
pub fn warn(msg: &str) {
    println!("[\x1b[1;33mWARN\x1b[0m] {}", msg);
}

#[inline]
pub fn vwarn(msg: &str) {
    if std::env::var("ERSA_VERBOSE").is_ok() {
        println!("[\x1b[1;33mWARN\x1b[0m] {}", msg);
    }
}

// Red
#[inline]
pub fn error(msg: &str) {
    eprintln!("[\x1b[1;31mERROR\x1b[0m] {}", msg);
}

#[inline]
pub fn verror(msg: &str) {
    if std::env::var("ERSA_VERBOSE").is_ok() {
        eprintln!("[\x1b[1;31mERROR\x1b[0m] {}", msg);
    }
}

// Green
#[inline]
pub fn success(msg: &str) {
    println!("[\x1b[1;32mSUCCESS\x1b[0m] {}", msg);
}

#[inline]
pub fn vsuccess(msg: &str) {
    if std::env::var("ERSA_VERBOSE").is_ok() {
        println!("[\x1b[1;32mSUCCESS\x1b[0m] {}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_functions() {
        info("This is an info message.");
        warn("This is a warning message.");
        error("This is an error message.");
        success("This is a success message.");
    }

    #[test]
    fn test_verbose_log_functions() {
        unsafe {
            std::env::set_var("ERSA_VERBOSE", "1");
        }
        vinfo("This is a verbose info message.");
        vwarn("This is a verbose warning message.");
        verror("This is a verbose error message.");
        vsuccess("This is a verbose success message.");
    }
}
