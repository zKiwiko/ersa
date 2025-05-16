mod git;
mod install;
mod list;
mod update;

pub async fn download(url: &str) -> Result<(), String> {
    install::url(&url).await
}

pub async fn update(package: &str) -> Result<(), String> {
    update::update(&package).await
}

pub fn list(package: &str) -> Result<(), String> {
    list::list(&package)
}
