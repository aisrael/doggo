use crate::doggo::{Context, Executable};
use anyhow::Result;
use reqwest::blocking::Response;
use serde_json::json;
use std::collections::HashMap;

/// Submit a metric
#[derive(Default)]
pub struct PostMetric {
    pub host: Option<String>,
    pub metric_type: String,
    pub metric_name: String,
    pub value: String,
    pub tags: Option<Vec<String>>,
}

impl PostMetric {
    fn build_body(&self, _context: &Context) -> String {
        let timestamp = super::unix_timestamp();
        let mut series_point: HashMap<String, serde_json::Value> = HashMap::new();
        if let Some(host) = &self.host {
            series_point.insert("host".to_owned(), json!(host));
        }
        series_point.insert("metric".to_owned(), json!(self.metric_name));
        series_point.insert("type".to_owned(), json!(self.metric_type));
        series_point.insert("points".to_owned(), json!([[timestamp, self.value]]));
        if let Some(tags) = &self.tags {
            series_point.insert("tags".to_owned(), json!(tags));
        }
        let json = json!({ "series": [series_point] });
        json.to_string()
    }
}

impl Executable for PostMetric {
    fn execute(&self, context: &Context) -> Result<Response> {
        let builder = super::reqwest_client_builder_from_context(context)?;
        let client = &builder.build()?;
        let body = self.build_body(context);
        if context.verbose {
            println!("POST https://api.datadoghq.com/api/v1/series");
            println!("{}", body);
        }
        let resp = client
            .post("https://api.datadoghq.com/api/v1/series")
            .header("DD-API-KEY", &context.api_key)
            .header("DD-APPLICATION-KEY", context.app_key.as_ref().unwrap())
            .body(body)
            .send()?;
        Ok(resp)
    }
}
