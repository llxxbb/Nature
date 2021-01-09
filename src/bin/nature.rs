use nature::util::system::sys_init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sys_init().await
}



