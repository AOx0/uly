mod args;
mod utils;

use args::*;
use utils::*;

fn main() {
    let app = Args::load();

    if let Commands::Create {
        name,
        do_not_open,
        overwrite,
    } = app.command
    {
        let name = &name.gen_name();

        name.create_file(overwrite).unwrap_or_else(|_| {
            eprintln!("File {} already exists", name);
            std::process::exit(1);
        });

        if !do_not_open {
            name.open_file();
        }
    } else if let Commands::UlyssesOpen { file, create } = app.command {
        let file = file.gen_name();

        if create {
            file.create_file(false).unwrap_or_else(|_| {})
        };

        file.open_file();
    } else if let Commands::Open { file } = app.command {
        file.open_file();
    }
}
