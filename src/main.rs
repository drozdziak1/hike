extern crate docopt;
extern crate libc;

use std::env;
use std::io;

use std::ffi::CString;
use std::io::Write;
use std::path::Path;
use std::process;

use docopt::Docopt;
use libc::WEXITSTATUS;

const USAGE: &'static str = "
Hike - do stuff in another directory.

Usage:
  hike <dir> <command>
  hike (-h | --help)
  hike --version

Options:
  <dir>         The directory to hike into.
  <command>     The command to execute in <dir>.
  -h --help     Shows this screen.
  --version     Prints the version of your Hike.
";

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version") {
        writeln!(io::stderr(), "{}", VERSION.unwrap_or("unknown")).expect("Could not write to stderr");
        process::exit(libc::EXIT_SUCCESS);
    }

    let dir = Path::new(args.get_str("<dir>"));
    if !dir.exists() {
        writeln!(io::stderr(), "hike: {}: No such file or directory", dir.to_str().unwrap())
            .expect("Could not write to stderr");
        process::exit(libc::ENOENT);
    }

    if !dir.is_dir() {
        writeln!(io::stderr(), "hike: {}: Not a directory", dir.to_str().unwrap())
            .expect("Could not write to stderr");
        process::exit(libc::EXIT_FAILURE);
    }

    let command = CString::new(args.get_str("<command>")).unwrap();

    let old_cwd = env::current_dir().unwrap();

    env::set_current_dir(dir).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Could not change to path \"{}\"", dir.to_str().unwrap())
            .expect("Could not write to stderr");
        process::exit(libc::EXIT_SUCCESS);
    });

    let command_result: libc::c_int;

    unsafe {
        command_result = WEXITSTATUS(libc::system(command.as_ptr()));
    }

    env::set_current_dir(old_cwd).unwrap();
    process::exit(command_result);
}
