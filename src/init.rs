use structopt::StructOpt;

use crate::dices::IntValue;

#[derive(Debug, StructOpt)]
#[structopt(name = "Options", about = "StructOpt command line options for dice thrower")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(long)]
    pub debug: bool,

    /// Set verbose
    #[structopt(short, parse(from_occurrences))]
    pub verbose: u8,

    /// Number of repetitions
    #[structopt(short="N", long="repetitions-num", default_value = "1")]
    pub num: usize,

    /// Number of dices
    #[structopt(short="n", long="dice-num", default_value = "1")]
    pub dices_num: usize,

    /// Dice sides
    #[structopt(short="d", long="dice", default_value = "6")]
    pub dice: usize,

    /// Result plus
    #[structopt(long="plus", default_value = "0")]
    pub plus: usize,

    /// Result minus
    #[structopt(long="minus", default_value = "0")]
    pub minus: usize,

    /// Number of lowest dices to be dropped
    #[structopt(short="D", long="drop", default_value = "0")]
    pub drop: usize,

    /// Number of greatest dices to be dropped
    #[structopt(short="C", long="crop", default_value = "0")]
    pub crop: usize,

    /// Stat generation method
    #[structopt(short, long, default_value = "")]
    pub method: String,

    /// Show all statistics
    #[structopt(long)]
    pub stat: bool,    

    /// Show minimum
    #[structopt(long)]
    pub min: bool,

    /// Show maximum
    #[structopt(long)]
    pub max: bool,

    /// Show mean
    #[structopt(long)]
    pub mean: bool,

    /// Show median
    #[structopt(long)]
    pub median: bool,

    /// Show mode
    #[structopt(long)]
    pub mode: bool,

    /// Show probabilities
    #[structopt(long)]
    pub probabilities: bool,

    /// Show only numbers
    #[structopt(long)]
    pub numbers_only: bool,

    /// Dice code
    #[structopt(default_value = "")]
    pub dices: String,
}

lazy_static! {
    pub static ref OPT: Opt = {
        let opt = Opt::from_args();
        if opt.debug {
            println!("{:?}", opt);
        }
        opt
    };
}

impl Opt {
    /// checks, if any statistics need to be collected
    pub fn is_collect_stat(&self) -> bool {
        return self.stat ||
               self.min ||
               self.max ||
               self.mean ||
               self.median ||
               self.mode ||
               self.probabilities;
    }
}

