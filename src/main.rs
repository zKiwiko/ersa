mod cli;

fn main() {
    let cli = cli::run();
    
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(cli);
    
    if let Err(e) = result {
        cli::console::err(format!("{:?}", e).as_str());
        std::process::exit(1);
    }
}
