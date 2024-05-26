use langchain_rust::embedding::openai::OpenAiEmbedder;
use langchain_rust::semantic_router::utils::combine_embeddings;
use std::error::Error;
// use futures::future::join_all;
use langchain_rust::embedding::Embedder;

pub fn chunk_string(s: &str, chunk_size: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

pub async fn embed_chunks(chunks: Vec<String>) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let openai = OpenAiEmbedder::default();
    let mut embeddings = Vec::new();

    for chunk in chunks {
        let embedding = openai.embed_query(&chunk).await?;
        embeddings.push(embedding);
    }

    Ok(embeddings)
}


// pub async fn embed_chunks(chunks: Vec<String>) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
//     let openai = OpenAiEmbedder::default();

//     let futures = chunks.into_iter().map(|chunk| {
//         let embeddings = openai.embed_query(&chunk);
//         return embeddings;
//     });

//     let results: Vec<Result<Vec<f64>, _>> = join_all(futures).await;

//     let embeddings = results.into_iter().collect::<Result<Vec<Vec<f64>>, _>>()?;
//     Ok(embeddings)
// }

