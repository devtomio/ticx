use clap::Parser;
use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(default_value = "./public")]
    dir: String,

    #[clap(short, long, default_value = "8080")]
    port: String,

    #[clap(short, long, default_value = "0.0.0.0")]
    address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/", &cli.dir).index_file("index.html").show_files_listing())
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", cli.address, cli.port))?
    .run()
    .await
}
