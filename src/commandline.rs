use getopts::{Matches, Options};

pub struct CommandLine {
    matches: Matches
}

impl CommandLine {
    pub fn new() -> Self {
        let mut opts = getopts::Options::new();
    
        opts.optopt("m", "method", "Brightness method",
            "Minmax or average");
        opts.optopt("f", "file", "Image file name", 
            "File Name");
    
        opts.optflag("i", "invert", "Invert color");
        opts.optflag("g", "green", "Print in green");
        opts.optflag("h", "help", "Print help menu");
    
        let matches = opts.parse(std::env::args().skip(1))
            .unwrap_or_else(|f| {
                eprintln!("Error: {}", f);
                Self::print_usage(&opts);
                std::process::exit(1);
            });
        
        if matches.opt_present("h"){
            Self::print_usage(&opts);
            std::process::exit(1);
        }

        Self {matches}
    }
    
    pub fn str_arg(&self, flag: &str, default: &str) -> String {
        self.matches.opt_str(flag).unwrap_or(default.to_string())
    }
    
    pub fn is_present(&self, flag: &str) -> bool {
        self.matches.opt_present(flag)
    }
    
    fn print_usage(opts: &Options) {
        let brief = format!("Usage: [options]");
        println!("{}", opts.usage(&brief));
    }
}