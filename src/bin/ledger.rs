extern crate ardite;
#[macro_use]
extern crate clap;

use clap::App;

fn main() {
  let matches = App::new("ledger")
                  .version(&crate_version!())
                  .global_version(true)
                  .unified_help_message(true)
                  .author("Victor M. Suarez <svmnotn@gmail.com>\nCaleb Meredith <calebmeredith8@gmail.com>")
                  .about("TODO")
                  .args_from_usage("--directory=[path] -d 'where the data is on disk (defaults to the current dir)'")
                  .get_matches();
  let dir = matches.value_of("path").unwrap_or(".");
  println!("{}", dir);
}
