use clap::Parser;

/// Set exif for media based on the date in their filename.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// just show what changes would be made to files, do not actually make them
    #[arg(short, long)]
    pub dryrun: bool,

    /// files to set date for
    pub files: Vec<String>,
}