use directories::*;
use downloaders::*;
use repos::*;
use structopt::StructOpt;

mod app;
mod directories;
mod downloaders;
mod errors;
mod repos;

use errors::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    let app = app::App::from_args();

    let repo_meta = parse_repo_input(&app.repo)?;
    let repo_base = format!("{}/{}", &repo_meta.owner, &repo_meta.repo);
    let git = app.git || &repo_meta.protocol == "git@";

    let force = app.force;
    let destination = check_distination(&app.destination, force)?;

    let filter = if app.workflows {
        Some(".github".into())
    } else {
        app.filter
    };

    let tmp_dir = prep_tmp_dir()?;

    if git {
        git_clone(&repo_base, &tmp_dir)?;
    } else {
        get_tarball(&repo_base, &tmp_dir).await?;
    }

    move_to_destination(&tmp_dir, &destination, filter)?;

    Ok(())
}
