use rig::{
    agent::MultiTurnStreamItem,
    completion::{CompletionModel, Prompt},
    prelude::*,
    providers::ollama::Client,
    streaming::{StreamedAssistantContent, StreamingPrompt},
};
