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

    /// Get GitHub Actions workflows only. (.github directory)
    #[structopt(short, long)]
    pub workflows: bool,

    /// Auto-replaces '$default-branch' placeholders.
    #[structopt(short, long)]
    pub default_branch: bool,

    /// Previews without updating destination.
    #[structopt(short, long)]
    pub preview: bool,

    /// Glob filter to get only specific directories and files.
    #[structopt(long)]
    pub filter: Option<String>,

    /// Source branch name.  Defaults to primary branch.
    #[structopt(long)]
    pub branch: Option<String>,
}
