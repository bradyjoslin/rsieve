# rsieve

`rsieve` copies all or portions of a remote GitHub repo. Useful for pulling down template projects or reusable bits of code.

By default gets files by downloading the tarball of the latest commit for the primary branch using HTTP. Also supports private repos via git (SSH).

Always omits the remote's `.git` directory and allow additional filtering using glob patterns to get only the specific files desired. Includes convience feature to copy GitHub Action workflows `.github` directory.

## Usage

```txt
rsieve 0.0.1
Copies all or portions of a remote git repo

USAGE:
    rsieve [FLAGS] [OPTIONS] <repo> [destination]

FLAGS:
    -g, --git          Git clone (SSH) instead of tarball via HTTP
    -h, --help         Prints help information
    -p, --preview      Previews without updating destination
    -V, --version      Prints version information
    -w, --workflows    Get GitHub Actions only

OPTIONS:
        --branch <branch>    Branch name.  Defaults to primary branch
        --filter <filter>    Glob filter to get only specific directories and files

ARGS:
    <repo>           GitHub repo. Required
    <destination>    Destination path [default: .]
```

Sample usage:

```sh
# Make local copy of public repo in current directory
rsieve owner/repo
rsieve https://github.com/owner/repo

# Make local copy of private repo in current directory.
# Requires local installation of git.
rsieve --git owner/repo
rsieve -g owner/repo
rsieve git@github.com:owner/repo.git

# Make local copy of public repo in specified directory.
rsieve owner/repo my-app

# Make local copy of public repo's feature-1 branch in
# specified directory.
rsieve --branch "feature-1" owner/repo my-app

# Make local copy of public repo's .github directory.
rsieve --workflows owner/repo

# Preview making local copy of public repo's .github directory.
rsieve --preview --workflows owner/repo my-app

# Copy all md files in public repo's root directory to current directory.
rsieve --filter "*.md" owner/repo

# Copy all png files in public repo's images directory to images directory
rsieve --filter "images/*.png" owner/repo images
```

## Installing

### macOS using Homebew

The easiest way to install rsieve is by using Homebrew.

```bash
brew tap bradyjoslin/rsieve
brew install rsieve
```

### Manually install a release

Download the binary for your OS from the [releases page](https://github.com/bradyjoslin/rsieve/releases) and place the unpacked `rsieve` somewhere on your PATH.

### Building and installing manually 

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo install --branch main --git https://github.com/bradyjoslin/rsieve
```

## References

Inspired by [degit](https://github.com/tiged/tiged), [ghat](https://github.com/fregante/ghat), and [related forks](https://github.com/psnszsn/degit-rs).
