# Release Checklist

Heavily references [How to Publish your Rust project on Homebrew](https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/) by Federico Terzi.

## All platforms

1. Push a tag to GitHub to initiate the builds

1. Download the binaries and unzip:

   ```sh
   unzip macos.zip
   unzip linux.zip
   unzip windows.zip
   ```

1. On GitHub go to Releases and create a new release. Create a tag for the next version number. Upload the tar files generated previously. Click Publish Release.

## macOS (for Homebrew)

1. Generate SHA Hash

   ```sh
   shasum -a 256 rsieve-v0.0.1-x86_64-apple-darwin.tar.gz
   ```

1. Get the download URL for the tar file under the assets section of the release page. i.e. https://github.com/bradyjoslin/rsieve/releases/download/v0.0.1/rsieve-v0.0.1-x86_64-apple-darwin.tar.gz

1. Go to https://github.com/bradyjoslin/homebrew-rsieve/blob/main/Formula/rsieve.rb and create a revision, updating the version number, download URL, and sha hash to reflect the latest.
