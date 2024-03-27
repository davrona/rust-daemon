use ethers::abi::ethereum_types::Signature;
use ethers::signers::{Signer, Wallet};
use ethers::core::k256::ecdsa::SigningKey;

use super::models::{Post, Answer, PromptRequest, ID};
use super::{MODEL_NAME, OLLAMA ,WALLET};



use ollama_rs::{
    generation::completion::{
        request::GenerationRequest, GenerationResponse
    },
    Ollama,
};

// A function to handle GET requests at /posts/{id}
pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let post: Post = Post {
        id,
        title: String::from("Hello, Warp!"),
        body: String::from("This is a post about Warp."),
    };
    Ok(warp::reply::json(&post))
}

// A function to handle GET requests at /id
pub async fn id() -> Result<impl warp::Reply, warp::Rejection> {
    println!("llm_sadfasdf");
    let wallet: Wallet<SigningKey> = WALLET.get().to_owned();
    let my_ip = local_ipaddress::get().unwrap();

    let identity = ID {
        public_key: wallet.address(),
        ip: my_ip,
        llm_model: MODEL_NAME.get().to_string(),
    };
    Ok(warp::reply::json(&identity))
}

// A function to handle POST requests at /prompt
pub async fn prompt(data: PromptRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let response = match get_llm_response(data.prompt).await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err(warp::reject::reject())
        }
    };

    let wallet: Wallet<SigningKey> = WALLET.get().to_owned();
    match wallet.sign_message(response.to_string()).await {
        Ok(sign) => {
            let answer = Answer {
                answer: response,
                signature: sign.to_string(),
            };
            Ok(warp::reply::json(&answer))
        }
        Err(err) => {
            eprintln!("Signature Error: {}", err);
            Err(warp::reject::reject())
        }
    }
}

// A function to handle POST requests at /prompt
pub async fn promptandpush(data: PromptRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let response = match get_llm_response(data.prompt).await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err(warp::reject::reject())
        }
    };
    
    let wallet: Wallet<SigningKey> = WALLET.get().to_owned();
    match wallet.sign_message(response.to_string()).await {
        Ok(sign) => {
            let answer = Answer {
                answer: response,
                signature: sign.to_string(),
            };
            Ok(warp::reply::json(&answer))
        }
        Err(err) => {
            eprintln!("Signature Error: {}", err);
            Err(warp::reject::reject())
        }
    }
}

pub async fn get_llm_response(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let ollama: Ollama = OLLAMA.get().to_owned();
    let model_name: String = MODEL_NAME.get().to_string();

    let request = GenerationRequest::new(model_name, prompt);
    let response: GenerationResponse = ollama.generate(request).await?;

    Ok(response.response)
}