extern crate execute;

use std::process::{exit, Stdio};

use execute::{shell, Execute};

use fs_extra::*;
use std::fs::*;
use std::io::Write;

#[macro_export]
macro_rules! e_printf {
    ( $($t:tt)* ) => {
        {
            let mut h = std::io::stderr();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}

#[macro_export]
macro_rules! err {
    ( $($t:tt)* ) => {
        {
            |_|{
                e_printf!($($t)*);
                std::process::exit(1);
            }
        }
    }
}

pub trait Util {
    fn gen_name(&self) -> String;
    fn open_file(&self);
    fn create_file(&self, overwrite: bool) -> Result<(), ()>;
}

impl Util for String {
    fn gen_name(&self) -> String {
        if self.contains(".ulysses") {
            format!("{}", self)
        } else {
            format!("{}.ulysses", self)
        }
    }

    fn open_file(&self) {
        let mut command = shell(format!("open -a UlyssesMac {}", self));

        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let output = command
            .execute_output()
            .unwrap_or_else(err!("Failed to execute command"));

        if let Some(code) = output.status.code() {
            if code != 0 {
                e_printf!("Error: ");
                e_printf!(
                    "{}",
                    String::from_utf8(output.stderr).unwrap_or_else(err!("Failed to get stderr"))
                );
                exit(1);
            }
        }
    }

    fn create_file(&self, overwrite: bool) -> Result<(), ()> {
        let text = include_bytes!("text");

        if dir::create(self, overwrite).is_err() {
            if overwrite {
                return Err(());
            }
        }

        let mut file = File::create(format!("{}/Content.xml", self))
            .unwrap_or_else(err!("Failed to create Content.xml"));

        file.write_all(text)
            .unwrap_or_else(err!("Failed to write to Content.xml"));

        Ok(())
    }
}
