use ethers::types::H160;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyPair {
    pub private: String,
    pub public: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ID {
    pub public_key: H160,
    pub ip: String,
    pub llm_model: String, 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Answer {
    pub answer: String,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PromptRequest {
    pub prompt: String,
}