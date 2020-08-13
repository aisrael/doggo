use crate::doggo::{Context, Executable};
use anyhow::Result;
use reqwest::blocking::Response;
use serde_json::json;

/// Submit a metric
#[derive(Default)]
pub struct PostMetric {
    pub host: Option<String>,
    pub metric_type: String,
    pub metric_name: String,
    pub value: String,
}

impl PostMetric {
    fn build_body(&self, _context: &Context) -> String {
        let timestamp = super::unix_timestamp();
        let series_point = if let Some(host) = &self.host {
            json!(
                {
                    "host": host,
                    "metric": self.metric_name,
                    "points":[
                        [timestamp, self.value]
                    ],
                    "tags": "source:doggo",
                    "type": self.metric_type
                }
            )
        } else {
            json!({
                "metric": self.metric_name,
                "points":[
                    [timestamp, self.value]
                ],
                "tags": "source:doggo",
                "type": self.metric_type
            })
        };
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
