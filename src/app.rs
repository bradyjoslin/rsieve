// Defines your CLI interface using structopt
use structopt::StructOpt;

/// TODO:  Replace comment with description of your CLI
#[derive(StructOpt, Debug)]
#[structopt(name = "TODO: Replace with CLI name")]
pub struct App {
    /// GitHub repo. Required.
    pub repo: String,

    /// Destination path.
    #[structopt(default_value = ".")]
    pub destination: String,

    /// Git clone (SSH) instead of tarball via HTTP.
    #[structopt(short, long)]
    pub git: bool,

    /// Write to destination directory even if not empty.
    #[structopt(short, long)]
    pub force: bool,

    /// Get GitHub Actions only.
    #[structopt(short, long)]
    pub workflows: bool,
}
