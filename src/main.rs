
extern crate git2;

mod lib;

fn main() {
    lib::get_fullcommit_log();
    lib::get_fullrepo_log();
}
