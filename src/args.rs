use clap::{AppSettings, ColorChoice, Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, name = "uly", color(ColorChoice::Never))]
#[clap(after_help = "\
EXAMPLES:
    uly create Test     # Creates a new document named Test.ulysses
    uly c Test          # Creates a new document named Test.ulysses
    uly c -o            # Create a new document and open it with Ulysses
    uly uo Test         # Open the existing file Test.ulysses with Ulysses
    uly o Test.md       # Open the existing file Test.md with Ulysses
    uly help c          # Display help msg for subcommand 'create'")]

pub struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        visible_alias = "uo",
        about = "Open an already existing Ulysses document without specifying extension (.ulysses)"
    )]
    UlyssesOpen {
        /// The file's path to be opened
        #[clap(required = true)]
        file: String,

        /// Create a new file if FILE does not already exist []
        #[clap(short, long)]
        create: bool,
    },

    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        visible_alias = "o",
        about = "Open any document on Ulysses, like a normal Markdown"
    )]
    Open {
        /// The file's path to be opened
        #[clap(required = true)]
        file: String
    },

    #[clap(visible_alias = "c", about = "Create a new Ulysses document")]
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
