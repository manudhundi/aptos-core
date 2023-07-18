// Copyright © Aptos Foundation

use regex::Regex;
use url::Url;

pub struct URIParser {
    prefix: String,
}

impl URIParser {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }

    /**
     * Attempts to parse IPFS URI to use dedicated gateway.
     * Returns the original URI if parsing fails.
     */
    pub fn parse(&self, uri: &str) -> String {
        match self.parse_uri(uri) {
            Ok(parsed_uri) => parsed_uri,
            Err(_) => uri.to_string(),
        }
    }

    pub fn parse_with_default(
        &self,
        uri_option: Option<&str>,
        default: Option<&str>,
    ) -> Option<String> {
        match uri_option {
            Some(uri) => match self.parse_uri(uri) {
                Ok(uri) => Some(uri),
                Err(_) => default.map(|s| s.to_string()),
            },
            None => default.map(|s| s.to_string()),
        }
    }

    fn parse_uri(&self, uri: &str) -> anyhow::Result<String> {
        let modified_uri = if uri.starts_with("ipfs://") {
            uri.replace("ipfs://", "https://ipfs.com/ipfs/")
        } else {
            uri.to_string()
        };

        let re = Regex::new(r"^(ipfs/)(?P<cid>[a-zA-Z0-9]+)(?P<path>/.*)?$")?;

        let path = Url::parse(&modified_uri)?
            .path_segments()
            .map(|segments| segments.collect::<Vec<_>>().join("/"));

        if let Some(captures) = re.captures(&path.unwrap_or_default()) {
            let cid = captures["cid"].to_string();
            let path = captures.name("path").map(|m| m.as_str().to_string());

            Ok(format!(
                "{}/{}{}",
                self.prefix,
                cid,
                path.unwrap_or_default()
            ))
        } else {
            Err(anyhow::anyhow!("Invalid IPFS URI"))
        }
    }
}
