# rsieve

`rsieve` copies all or portions of a remote GitHub repo.  Useful for pulling down template projects or reusable bits of code.  

Gets the tarball of the latest commit for the primary branch using HTTP.  Supports private repos via `--git` via SSH.  

Always omits the remote's .git directory and allow additional filtering using glob patterns to only get specific files.  Use the `--workflows` flag to make a local copy of the remote's GitHub Action workflows `.github` directory.

## Usage

```txt
rsieve 0.0.1
Copies all or portions of a remote git repo

USAGE:
    rsieve [FLAGS] [OPTIONS] <repo> [destination]

FLAGS:
    -f, --force        Write to destination directory even if not empty
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
rsieve https://owner/repo

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
🔬 These files would be copied to my-app.

.github
├─ workflows
│  ├─ pr.yml
│  └─ main.yml
└─ dependabot.yml

# Make local copy of public repo's .github directory, overwriting 
# any existing local .github directory.
rsieve --force --workflows owner/repo

# Copy all md files in public repo's root directory to current directory.
rsieve --filter "*.md" owner/repo

# Copy all png files in public repo's images directory to images directory
rsieve --filter "images/*.png" owner/repo images
```

## Installing

Building and installing requires [Rust](https://www.rust-lang.org/tools/install). To build, clone the repository and then:

```bash
cargo build
```

To run the debug build:

```bash
cargo run
```

To create a release build:

```bash
cargo build --release
```

To install:

```bash
cargo install --path .
```

## References

Inspired by [degit](https://github.com/tiged/tiged), [ghat](https://github.com/fregante/ghat), and [related forks](https://github.com/psnszsn/degit-rs).
