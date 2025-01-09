use std::io;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
/// Glagolica is a modern documentation tool designed to streamline development in every possible environment, fitting seamlessly into all languages and ecosystems with effortless pipeline integration and versatile customization options.
/// Harnessing the elegance of Markdown, Glagolica transforms simplicity into brilliance, generating stunning web documentation that combines form with functionality.
struct GlagCli {}

fn main() -> io::Result<()> {
    let _cli = GlagCli::parse();
    
    println!("Hello from Glagolica!");
    
    Ok(())
}
