use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dnddice", about = "RPG dice thrower for command line. Author: Dargot, dargot@yandex.ru")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(long)]
    pub debug: bool,

    /// Show help about dicecodes
    #[structopt(long)]
    pub help_dice_codes: bool,

    /// Show help about generation methods
    #[structopt(long)]
    pub help_methods: bool,

    /// Show help about generation method
    #[structopt(long, default_value = "")]
    pub help_method: String,

    /// Verbose mode (-v, -vv, etc.)
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

    /// Stat generation method (adnd1, adnd2, etc...)
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

    /// Show sum
    #[structopt(long)]
    pub sum: bool,

    /// Round probabilities to number of digits
    #[structopt(long, default_value = "2")]
    pub round_digits: u8,

    /// Show only numbers
    #[structopt(long)]
    pub numbers_only: bool,

    /// No help messages
    #[structopt(long)]
    pub no_help: bool,

    /// Dice codes (2d8+1, 4d6drop1 etc...)
    #[structopt(default_value = "")]
    pub dicecodes: Vec<String>,
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

    /// checks, if any help info need to be shown
    pub fn is_help(&self) -> bool {
        return  self.help_dice_codes ||
                self.help_methods ||
                !self.help_method.is_empty()
    }

}

