use langchain_rust::document_loaders::lo_loader::LoPdfLoader;
use langchain_rust::schemas::Document;
use futures_util::StreamExt;
use std::error::Error;
use langchain_rust::document_loaders::Loader;

pub async fn load_resume(path: &str) -> Result<Vec<Document>, Box<dyn Error>> {
    let loader = LoPdfLoader::from_path(path)?;
    let resume = loader.load().await?
        .map(|d| d.unwrap())
        .collect::<Vec<Document>>().await;
    Ok(resume)
}
