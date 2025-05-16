mod create;

pub fn create(name: &str, language: &str, output: Option<&str>) -> Result<(), String> {
    create::new(name, language, output)
}