use clap::Clap;
use libipld::cid::Cid;
use std::path::PathBuf;

#[derive(Clone, Debug, Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: SubCommand,
    #[clap(short = "p", long = "path")]
    pub path: PathBuf,
    #[clap(short = "t", long = "tree")]
    pub tree: Option<String>,
}

#[derive(Clone, Debug, Clap)]
pub enum SubCommand {
    Tree,
    Ls(LsCommand),
    Cat(CatCommand),
    Refs(RefsCommand),
    Unpin(UnpinCommand),
}

#[derive(Clone, Debug, Clap)]
pub struct LsCommand {
    #[clap(long = "pinned", conflicts_with_all(&["live", "dead", "all"]))]
    pub pinned: bool,
    #[clap(long = "live", conflicts_with_all(&["pinned", "dead", "all"]))]
    pub live: bool,
    #[clap(long = "dead", conflicts_with_all(&["pinned", "live", "all"]))]
    pub dead: bool,
    #[clap(long = "all", conflicts_with_all(&["pinned", "live", "dead"]))]
    pub all: bool,
}

#[derive(Clone, Debug, Clap)]
pub struct CatCommand {
    pub cid: Cid,
}

#[derive(Clone, Debug, Clap)]
pub struct RefsCommand {
    pub cid: Cid,
}

#[derive(Clone, Debug, Clap)]
pub struct UnpinCommand {
    pub cid: Cid,
}
