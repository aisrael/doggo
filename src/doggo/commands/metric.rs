use crate::doggo::{Context, Executable};
use anyhow::Result;
use reqwest::blocking::Response;
use serde_json::json;

/// Submit a metric
#[derive(Default)]
pub struct PostMetric {
    pub host: String,
    pub metric_type: String,
    pub name: String,
    pub value: String,
}

impl PostMetric {
    fn build_body(&self, _context: &Context) -> String {
        let timestamp = super::unix_timestamp();
        let json = json!({
            "series": [
                {
                    "host": self.host,
                    "metric": "test.metric",
                    "points":[
                        [timestamp, 1]
                    ],
                    "tags": "source:doggo",
                    "type": "count"
                }
            ]
        });
        json.to_string()
    }
}

impl Executable for PostMetric {
    fn execute(&self, context: &Context) -> Result<Response> {
        let builder = super::reqwest_client_builder_from_context(context)?;
        let client = &builder.build()?;
        let body = self.build_body(context);
        let resp = client
            .post("https://api.datadoghq.com/api/v1/series")
            .header("DD-API-KEY", &context.api_key)
            .header("DD-APPLICATION-KEY", context.app_key.as_ref().unwrap())
            .body(body)
            .send()?;
        Ok(resp)
    }
}
