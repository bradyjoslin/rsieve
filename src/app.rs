// Defines your CLI interface using structopt
use structopt::StructOpt;

/// Copies all or portions of a remote git repo.
#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
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

    /// Previews without updating destination.
    #[structopt(short, long)]
    pub preview: bool,

    /// Glob filter to get only specific directories and files.
    #[structopt(long)]
    pub filter: Option<String>,

    /// Branch name.  Defaults to primary branch.
    #[structopt(long)]
    pub branch: Option<String>,
}
