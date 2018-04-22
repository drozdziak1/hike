extern crate docopt;
extern crate libc;

use std::env;

use std::ffi::CString;
use std::fs::create_dir_all;
use std::path::Path;
use std::process;

use docopt::Docopt;
use libc::WEXITSTATUS;

const USAGE: &'static str = "
Hike - do stuff in another directory.

Usage:
  hike [-p] <dir> <command>
  hike (-h | --help)
  hike --version

Options:
  <dir>         The directory to hike into.
  <command>     The command to execute in <dir>.
  -h --help     Shows this screen.
  -p            Creates the directory (like mkdir -p)
  --version     Prints the version of your Hike.
";

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version") {
        eprintln!("{}", VERSION.unwrap_or("unknown"));
        process::exit(libc::EXIT_SUCCESS);
    }

    let dir = if args.get_bool("-p") {
        let new_dir = Path::new(args.get_str("<dir>"));
        create_dir_all(new_dir).expect("Could not create directory");
        new_dir
    } else {
        let existing_dir = Path::new(args.get_str("<dir>"));
        if !existing_dir.exists() {
            eprintln!("hike: {}: No such file or directory", existing_dir.to_str().unwrap());
            process::exit(libc::ENOENT);
        }
        existing_dir
    };

    if !dir.is_dir() {
        eprintln!("hike: {}: Not a directory", dir.to_str().unwrap());
        process::exit(libc::EXIT_FAILURE);
    }

    let command = CString::new(args.get_str("<command>")).unwrap();

    let old_cwd = env::current_dir().unwrap();

    env::set_current_dir(dir).unwrap_or_else(|_| {
        eprintln!("Could not change to path \"{}\"", dir.to_str().unwrap());
        process::exit(libc::EXIT_SUCCESS);
    });

    let command_result: libc::c_int;

    unsafe {
        command_result = WEXITSTATUS(libc::system(command.as_ptr()));
    }

    env::set_current_dir(old_cwd).unwrap();
    process::exit(command_result);
}
