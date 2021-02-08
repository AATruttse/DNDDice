use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Options", about = "StructOpt command line options for dice thrower")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(long)]
    pub debug: bool,

    /// Set verbose
    #[structopt(short, parse(from_occurrences))]
    pub verbose: u8,

    /// Stat generation method
    #[structopt(short, long, default_value = "adnd1")]
    pub method: String,

    /// Collect statistics to calculate means - number of iterations
    #[structopt(long, default_value = "1")]
    pub stat: u32,    

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

