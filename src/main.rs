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
    let destination = if app.workflows && &app.destination == "." {
        check_distination(".github", app.force)?
    } else {
        check_distination(&app.destination, app.force)?
    };

    let tmp_dir = prep_tmp_dir()?;

    if app.git || &repo_meta.protocol == "git@" {
        git_clone(&repo_meta.url_stem, &tmp_dir)?;
    } else {
        get_tarball(&repo_meta.url_stem, &tmp_dir).await?;
    }

    let filter = if app.workflows {
        Some(".github/*".into())
    } else {
        app.filter
    };

    move_to_destination(&tmp_dir, &destination, filter)?;

    Ok(())
}
