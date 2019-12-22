
extern crate git2;

mod lib;

fn main() {
    lib::get_fullcommit_log();
    lib::create_new_branch();
    lib::rename_branch();
    lib::delete_branch();
}
