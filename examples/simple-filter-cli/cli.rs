use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long = "where")]
    pub(crate) filter: Option<filter_expr::Filter>,
}
