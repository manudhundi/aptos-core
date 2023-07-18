// Copyright © Aptos Foundation

use crate::{
    image_optimizer::ImageOptimizer,
    json_parser::JSONParser,
    models::{NFTMetadataCrawlerURIs, NFTMetadataCrawlerURIsQuery},
    uri_parser::URIParser,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use image::ImageFormat;
use nft_metadata_crawler_utils::NFTMetadataCrawlerEntry;
use serde_json::Value;
use tracing::info;

// Stuct that represents a parser for a single entry from queue
#[allow(dead_code)]
pub struct Parser {
    entry: NFTMetadataCrawlerEntry,
    model: NFTMetadataCrawlerURIs,
    bucket: String,
    token: String,
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    cdn_prefix: String,
    uri_parser: URIParser,
}

impl Parser {
    pub fn new(
        entry: NFTMetadataCrawlerEntry,
        bucket: String,
        token: String,
        conn: PooledConnection<ConnectionManager<PgConnection>>,
        cdn_prefix: String,
        ipfs_prefix: String,
    ) -> Self {
        Self {
            model: NFTMetadataCrawlerURIs::new(entry.token_uri.clone()),
            entry,
            bucket,
            token,
            conn,
            cdn_prefix,
            uri_parser: URIParser::new(ipfs_prefix),
        }
    }

    // Main parsing flow
    pub async fn parse(&mut self) -> anyhow::Result<()> {
        // Deduplication
        if !self.entry.force
            && NFTMetadataCrawlerURIsQuery::get_by_token_uri(
                self.entry.token_uri.clone(),
                &mut self.conn,
            )
            .expect("Unable to get URIs")
            .is_some()
        {
            info!(
                last_transaction_version = self.entry.last_transaction_version,
                "Skipping URI parse"
            );
            return Ok(());
        }

        // Parse token_uri
        self.model.set_token_uri(self.entry.token_uri.clone());
        let json_uri = self.uri_parser.parse(self.model.get_token_uri().as_str());

        // Parse JSON for raw_image_uri and raw_animation_uri
        let (raw_image_uri, raw_animation_uri, json) = JSONParser::parse(json_uri).await;
        self.model.set_raw_image_uri(raw_image_uri);
        self.model.set_raw_animation_uri(raw_animation_uri);

        // Save parsed JSON to GCS
        let cdn_json_uri = self.handle_write_json_to_gcs(json).await;
        self.model.set_cdn_json_uri(cdn_json_uri);

        self.commit_to_postgres().await;

        // Parse raw_image_uri and raw_animation_uri, resort to default if parsing fails
        let img_uri = self.uri_parser.parse_with_default(
            self.model.get_raw_image_uri().as_deref(),
            Some(self.model.get_token_uri().as_str()),
        );
        let animation_uri = self
            .uri_parser
            .parse_with_default(self.model.get_raw_animation_uri().as_deref(), None);

        // Resize and optimize image and animation
        let image = ImageOptimizer::optimize(img_uri).await;
        let animation = ImageOptimizer::optimize(animation_uri).await;

        // Save resized and optimized image and animation to GCS
        let cdn_image_uri = self.handle_write_image_to_gcs(image).await;
        let cdn_animation_uri = self.handle_write_image_to_gcs(animation).await;
        self.model.set_cdn_image_uri(cdn_image_uri);
        self.model.set_cdn_animation_uri(cdn_animation_uri);

        self.commit_to_postgres().await;

        Ok(())
    }

    /**
     * Calls and handles error for writing JSON to GCS
     */
    async fn handle_write_json_to_gcs(&mut self, _json: Option<Value>) -> Option<String> {
        todo!();
    }

    /**
     * Calls and handles error for writing image to GCS
     */
    async fn handle_write_image_to_gcs(
        &mut self,
        _image: Option<(Vec<u8>, ImageFormat)>,
    ) -> Option<String> {
        todo!();
    }

    /**
     * Calls and handles error for upserting to Postgres
     */
    async fn commit_to_postgres(&mut self) {
        todo!();
    }
}
