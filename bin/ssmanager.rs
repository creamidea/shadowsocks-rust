//! This is a binary running in the server environment
//!
//! You have to provide all needed configuration attributes via command line parameters,
//! or you could specify a configuration file. The format of configuration file is defined
//! in mod `config`.
//!
//! *It should be notice that the extended configuration file is not suitable for the server
//! side.*

use clap::clap_app;
use shadowsocks_rust::service::manager;

fn main() {
    let mut app = clap_app!(shadowsocks =>
        (version: shadowsocks_rust::VERSION)
        (about: "A fast tunnel proxy that helps you bypass firewalls. (https://shadowsocks.org)")
    );
    app = manager::define_command_line_options(app);

    let matches = app.get_matches();
    manager::main(&matches);
}
