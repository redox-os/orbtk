extern crate orbclient;
extern crate orbtk;

use orbtk::dialogs::FileDialog;

fn main() {
    println!("{:?}", FileDialog::new().exec());
}
