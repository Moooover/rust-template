use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::info;

use super::Config;

pub struct DatabaseDriver {
    pub client: Surreal<Client>,
}

impl DatabaseDriver {
    pub async fn init(config: &Config) -> Result<Self, ()> {
        let client = Surreal::new::<Ws>(&config.db_url)
            .await
            .expect(&format!("Unable to connect to DB!"));

        info!("Connected to the Database on {}", &config.db_url);

        client
            .signin(Root {
                username: &config.db_username,
                password: &config.db_password,
            })
            .await
            .expect(&format!("Failed to authorize DB access!"));

        info!("Database access granted to {}", &config.db_username);

        client
            .use_ns(&config.db_namespace)
            .use_db(&config.db_name)
            .await
            .expect(&format!("Unable to config namespace!"));

        info!(
            "Using {} namespace and {} database",
            &config.db_namespace, &config.db_name
        );

        Ok(Self { client })
    }
}
