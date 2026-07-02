// use rig::rig_lancedb::{LanceDbVectorIndex, SearchParams};
use rig::{
    agent::Text,
    client::{CompletionClient, EmbeddingsClient, ProviderClient},
    completion::{CompletionModel, Document},
    embeddings::EmbeddingsBuilder,
    providers::ollama::Client,
    vector_store::{VectorSearchRequest, VectorStoreIndex, in_memory_store::InMemoryVectorStore},
};

use crate::agent::Agent;

// async fn rag_main() -> Result<(), Box<dyn std::error::Error>> {
//     let ollama_client = Client::from_env()?;

//     let db = lancedb::connect("data/lancedb-store").execute().await?;
//     let vector_store = LanceDbVectorIndex::new(table, model, "id", SearchParams::default()).await?;

//     Ok(())
// }
