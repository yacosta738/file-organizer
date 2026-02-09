use anyhow::Result;
use clap::Parser;

mod globals;
mod utils;

#[derive(Parser)]
#[command(name = "organizer")]
#[command(about = "Organizes a directory based on file extensions", long_about = None)]
struct Cli {
    /// The directory to organize. Defaults to the Downloads folder.
    path: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let dir = utils::get_download_dir(cli.path.as_deref())?;

    println!("Organizing directory: {}", dir.display());

    utils::set_write_permissions(&dir)?;
    utils::create_dirs(&dir)?;
    utils::organize(&dir)?;

    println!("Organization complete!");

    Ok(())
}
