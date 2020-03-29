extern crate nature;

use nature::system::sys_init;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    sys_init().await
}



