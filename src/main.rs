
extern crate git2;

mod lib;

use lib::merge_all_hooks;
use futures::executor::block_on;

fn main() {
    block_on(merge_all_hooks())
}
