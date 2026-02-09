use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod globals;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to organize (defaults to ~/Downloads)
    directory: Option<PathBuf>,

    /// Only list actions without executing them
    #[arg(short, long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dir = if let Some(path) = args.directory {
        path.canonicalize()?
    } else {
        // We use the original logic but adapted
        // Actually I'll call get_download_dir with empty args or similar
        // or just re-implement here for simplicity since we use clap now.
        utils::get_default_download_dir()?
    };

    println!("Organizing directory: {}", dir.display());
    if args.dry_run {
        println!("Running in DRY RUN mode");
    }

    utils::set_write_permissions(&dir, args.dry_run)?;
    utils::create_dirs(&dir, args.dry_run)?;
    utils::organize(&dir, args.dry_run)?;

    println!("Done!");
    Ok(())
}
