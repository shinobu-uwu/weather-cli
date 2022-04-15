#[derive(Debug, StructOpt)]
#[structopt(name = "weather")]
pub struct Cli {
    #[structopt(name="toggle", short="t", short="Toggles unit between metric and imperial")]
    pub toggle_units: bool
}