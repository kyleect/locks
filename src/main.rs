use anyhow::Result;
use clap::Parser;
use locks::cmd::Cmd;

fn main() -> Result<()> {
    Cmd::parse().run()
}
