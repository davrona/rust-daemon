use clap::{Arg, ArgAction, Command};
use ethers::core::k256::ecdsa::SigningKey;
use ollama_rs::Ollama;
use ethers::core::rand::thread_rng;
use ethers::signers::{LocalWallet, Signer, Wallet};

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
        println!("Generate a new ECDSA keypair");
        
        let wallet = LocalWallet::new(&mut thread_rng());
        
        WALLET.set(wallet);
        // save wallet key to local
    } else {
        println!("load ECDSA keypair from key file");
        let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
            .parse::<LocalWallet>()?;
        println!("Private Key: {:?}", wallet.signer().to_bytes());
        println!("Public Key: {:?}", wallet.address());

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
