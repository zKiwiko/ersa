pub mod git;
mod install;
mod list;
mod remove;
mod update;

pub async fn download(url: &str) -> Result<(), String> {
    install::url(&url).await
}

pub async fn update(package: &str) -> Result<(), String> {
    update::update(&package).await
}

pub fn list(package: &Option<String>) -> Result<(), String> {
    list::list(package)
}

pub fn remove(package: &str) -> Result<(), String> {
    remove::remove(&package)
}
