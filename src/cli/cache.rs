use crate::view::{cache::cache_view, larablade::compile_larablade};
use clap::{Parser, Subcommand};
use std::{fs, io};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cache {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Clear the view cache
    Clear,
    /// Warm up the view cache
    Warmup,
}

pub fn run(cache: &Cache) {
    match &cache.command {
        Commands::Clear => {
            if let Err(e) = clear_view_cache() {
                eprintln!("Error clearing view cache: {}", e);
            }
        }
        Commands::Warmup => {
            println!("Warming up view cache...");
            if let Err(e) = warmup_view_cache() {
                eprintln!("Error warming up view cache: {}", e);
            }
        }
    }
}

fn clear_view_cache() -> io::Result<()> {
    let path = "storage/framework/views";
    if fs::metadata(path).is_ok() {
        fs::remove_dir_all(path)?;
        println!("View cache cleared successfully.");
    } else {
        println!("View cache directory does not exist, nothing to clear.");
    }
    Ok(())
}

fn warmup_view_cache() -> io::Result<()> {
    let views_dir = "resources/views";
    let entries = fs::read_dir(views_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let template_name = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let content = fs::read_to_string(&path)?;
            let compiled = compile_larablade(&content);
            cache_view(template_name, &compiled);
        }
    }

    Ok(())
}
