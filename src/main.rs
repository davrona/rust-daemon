use clap::{Arg, ArgAction, Command};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::utils::hex::ToHex;
use ollama_rs::Ollama;
use ethers::core::rand::thread_rng;
use ethers::signers::{LocalWallet, Signer, Wallet};
use warp::filters::body::json;
use std::str;

mod handlers;
mod routes;
mod models;

static MODEL_NAME: state::InitCell<String> = state::InitCell::new();
static OLLAMA: state::InitCell<Ollama> = state::InitCell::new();
static WALLET: state::InitCell<Wallet<SigningKey>> = state::InitCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("prog")
        .arg(Arg::new("llm")
            .short('l')
            .long("llm")
            .default_value("llama2")
            .help("Sets the LLM to use"))
        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .default_value("8080")
            .help("Sets the port for HTTPS listener"))
        .arg(Arg::new("genkey")
            .long("genkey")
            .action(ArgAction::SetTrue)
            .help("Generate a new ECDSA keypair"))
        .arg(Arg::new("gpu")
            .long("gpu")
            .action(ArgAction::SetTrue)
            .help("Pin the LLM to a GPU"))
        .get_matches();

    // Get command line arguments
    // Initialize LLM based on provided flag (--llm llama2)
    let llm_name = matches.get_one::<String>("llm").unwrap();
    println!("llm_name:{}", llm_name);

    // Set Global Variable
    MODEL_NAME.set(llm_name.to_string() + ":latest");
    OLLAMA.set(Ollama::default());

    // Generate ECDSA keypair if --genkey flag is provided
    // For demonstration, generate a keypair here 
    if matches.get_flag("genkey") {
        println!("Generating a new ECDSA keypair...");
        
        let wallet = LocalWallet::new(&mut thread_rng());
        let wallet = wallet.with_chain_id(1337u64);

        WALLET.set(wallet.to_owned());

        let public_key_str = hex::encode(wallet.address().as_bytes()).to_string();
        let private_key_str = hex::encode(wallet.signer().to_bytes()).to_string();

        let key_pair = models::KeyPair {
            private: private_key_str,
            public: public_key_str,
        };

        // Serialize the struct to JSON
        let json_data = serde_json::to_string(&key_pair)?;

        file::put_text("key.json", &json_data)?;
    } else {
        println!("Loading ECDSA keypair from key file...");
        let key_string = file::get_text("key.json")?;
        let key_pair: models::KeyPair = serde_json::from_str(&key_string)?;
        
        let wallet = key_pair.private
            .parse::<LocalWallet>()?;
        WALLET.set(wallet);
    }

    // Optionally, handle GPU usage based on --gpu flag
    if matches.get_flag("gpu") {
        println!("Pin the LLM to a GPU");
    }

    let port_name = matches.get_one::<String>("port").unwrap();

    // Setup web server
    let routes = routes::routes();
    match port_name.parse::<u16>() {
        Ok(port) => {
            println!("Server started at port:{}", port);
            warp::serve(routes).run(([127, 0, 0, 1], port)).await;
        },
        Err(_) => {
            println!("Server started at port 8080");
            warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
        },
    }

    Ok(())
}