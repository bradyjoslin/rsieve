use console::{style, Emoji, StyledObject};
use directories::*;
use downloaders::*;
use errors::AppResult;
use repos::*;
use structopt::StructOpt;
mod app;
mod directories;
mod downloaders;
mod errors;
mod git;
mod placeholders;
mod repos;
mod tree;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨  ", "");
static MICROSCOPE: Emoji<'_, '_> = Emoji("🔬  ", "");

#[tokio::main]
async fn main() -> AppResult<()> {
    let app = app::App::from_args();

    let steps = if app.preview { 2 } else { 3 };
    fn step_of(x: i32, steps: i32) -> StyledObject<String> {
        style(format!("[{}/{}]", x, steps)).bold().dim()
    }

    let repo_meta = parse_repo_input(&app.repo)?;
    let destination = if app.workflows && &app.destination == "." {
        check_distination(".github", false)?
    } else {
        check_distination(&app.destination, false)?
    };

    let tmp_dir = prep_tmp_dir()?;

    println!(
        "{} {}Getting {}...",
        step_of(1, steps),
        LOOKING_GLASS,
        app.repo
    );

    if app.git || &repo_meta.protocol == "git@" {
        git_clone(&repo_meta.url_stem, &tmp_dir, app.branch)?;
    } else {
        get_tarball(&repo_meta.url_stem, &tmp_dir, app.branch).await?;
    }

    let filter = if app.workflows {
        Some(".github/*".into())
    } else {
        app.filter
    };

    if !app.preview {
        println!(
            "{} {}Moving {} files to {}...",
            step_of(2, steps),
            TRUCK,
            &app.repo,
            &destination
        );
    } else {
        println!(
            "{} {}These files from {} would be copied to {}...",
            step_of(2, steps),
            MICROSCOPE,
            &app.repo,
            &destination
        );
    }

    move_to_destination(&tmp_dir, &destination, filter, app.preview, app.template)?;

    if !app.preview {
        println!("{} {}Done!", step_of(3, steps), SPARKLE);
    }

    Ok(())
}
