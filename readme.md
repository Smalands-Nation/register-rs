# Register-rs
**also known as smalands-rs**

This is a cash register implementation for Smålands Nation in Halmstad, Sweden

## External dependencies
To allow for printing reciepts on windows this app uses [PDFtoPrinter](http://www.columbia.edu/~em36/pdftoprinter.html)

## Updating and tagging
Smalands-rs uses [`self_update`](https://crates.io/crates/self_update) to automatically update itself from github releases. For whis to work properly it assumes that the version specified in `Cargo.toml` is the same as the closest previous tag in any given commit. To make this parity less of a headache a tool such as [`cargo-workspaces`](https://crates.io/crates/cargo-workspaces) is recomended. To furher simplify new releases there is a GitHub action that automatically creates, compiles and publishes a new release every time a new version tag is pushed.

## Environment Variables
When building a release manually the environment variable `SMALANDS_PASSWORD` must exist to set the password for the 'manage' tab
