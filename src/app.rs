// Defines your CLI interface using structopt
use structopt::StructOpt;

/// TODO:  Replace comment with description of your CLI
#[derive(StructOpt, Debug)]
#[structopt(name = "TODO: Replace with CLI name")]
pub struct App {
    /// GitHub repo. Required.
    pub repo: String,

    /// Local destination path.
    #[structopt(default_value = ".")]
    pub destination: String,

    /// Use git clone (SSH) instead of tarball via HTTP.
    #[structopt(short, long)]
    pub git: bool,
}
