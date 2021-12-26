use clap::{AppSettings, ColorChoice, Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, name = "uly", color(ColorChoice::Never))]
#[clap(after_help = "\
EXAMPLES:
    uly create Test     # Creates a new document named Test.ulysses
    uly c Test          # Creates a new document named Test.ulysses
    uly c -o            # Create a new document and open it with Ulysses
    uly o Test          # Open the existing file Test.ulysses with Ulysses
    uly help c          # Display help msg for subcommand 'create'")]

pub struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        alias = "o",
        about = "Open an already existing Ulysses document [alias: o]"
    )]
    Open {
        /// The file's path to be opened
        #[clap(required = true)]
        file: String,

        /// Create a new file if FILE does not already exist
        #[clap(short, long)]
        create: bool,
    },

    #[clap(alias = "c", about = "Create a new Ulysses document [alias: c]")]
    Create {
        /// The name for the new Ulysses document
        #[clap(required = true, default_value = "Writeup")]
        name: String,

        /// Create even if the file already exists, overwriting it in the process
        #[clap(short, long)]
        overwrite: bool,

        /// Open the created file on Ulysses
        #[clap(short, long)]
        do_not_open: bool,
    },
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
