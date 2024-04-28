//! The `prebuild` subcommand
//! This command is used to prebuild the assets of your project
//! It builds the PNG assets by reading PNG files, extracting the edges, simplifying the edges, and writing the edges to a JSON file

use argh::FromArgs;

use super::SubCommandTrait;

#[derive(PartialEq, Debug, FromArgs)]
#[argh(
    subcommand,
    name = "prebuild",
    description = "pre-build the assets of your project"
)]
pub struct PreBuild {}

impl SubCommandTrait for PreBuild {
    fn run(&self) {}
}
