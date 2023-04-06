use std::sync::Arc;

use config::Config;
use mongodb::Client;
use service::post::PostServiceImpl;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    run(config::config()?).await
}

async fn run(config: Config) -> eyre::Result<()> {
    let client = Client::with_uri_str(config.mongo).await?;
    let database = client.database(&config.database);

    let state = routes::RouterState {
        post_service: Arc::from(PostServiceImpl {
            collection: database.collection(&config.post.collection),
        }) as _,
    };

    let app = routes::router(state);
    axum_server::bind(config.listen)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
