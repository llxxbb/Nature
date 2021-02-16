use nature::nature_lib::web_init::web_init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    web_init().await
}



