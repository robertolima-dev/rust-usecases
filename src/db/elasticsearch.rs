use crate::config::get_settings;
use elasticsearch::{Elasticsearch, http::transport::Transport};

pub fn get_elastic_client() -> Result<Elasticsearch, Box<dyn std::error::Error + Send + Sync>> {
    let settings = get_settings();
    let transport = Transport::single_node(&settings.elasticsearch.url)?;
    Ok(Elasticsearch::new(transport))
}
