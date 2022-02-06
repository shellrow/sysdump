mod util;
use util::table::show_system_overview;
use util::table::show_connection_simple;

fn main() {
    show_system_overview();
    show_connection_simple();
}
