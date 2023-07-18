// Copyright © Aptos Foundation

use crate::schema::nft_metadata_crawler_uris;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(token_uri))]
#[diesel(table_name = nft_metadata_crawler_uris)]
pub struct NFTMetadataCrawlerURIs {
    token_uri: String,
    raw_image_uri: Option<String>,
    raw_animation_uri: Option<String>,
    cdn_json_uri: Option<String>,
    cdn_image_uri: Option<String>,
    cdn_animation_uri: Option<String>,
    image_optimizer_retry_count: i32,
    json_parser_retry_count: i32,
}

impl NFTMetadataCrawlerURIs {
    pub fn new(token_uri: String) -> Self {
        Self {
            token_uri: token_uri.clone(),
            raw_image_uri: None,
            raw_animation_uri: None,
            cdn_json_uri: None,
            cdn_image_uri: None,
            cdn_animation_uri: None,
            image_optimizer_retry_count: 0,
            json_parser_retry_count: 0,
        }
    }

    pub fn get_token_uri(&self) -> String {
        self.token_uri.clone()
    }

    pub fn set_token_uri(&mut self, token_uri: String) {
        self.token_uri = token_uri;
    }

    pub fn get_raw_image_uri(&self) -> Option<String> {
        self.raw_image_uri.clone()
    }

    pub fn set_raw_image_uri(&mut self, raw_image_uri: Option<String>) {
        self.raw_image_uri = raw_image_uri;
    }

    pub fn get_raw_animation_uri(&self) -> Option<String> {
        self.raw_animation_uri.clone()
    }

    pub fn set_raw_animation_uri(&mut self, raw_animation_uri: Option<String>) {
        self.raw_animation_uri = raw_animation_uri;
    }

    pub fn get_cdn_json_uri(&self) -> Option<String> {
        self.cdn_json_uri.clone()
    }

    pub fn set_cdn_json_uri(&mut self, cdn_json_uri: Option<String>) {
        self.cdn_json_uri = cdn_json_uri;
    }

    pub fn get_cdn_image_uri(&self) -> Option<String> {
        self.cdn_image_uri.clone()
    }

    pub fn set_cdn_image_uri(&mut self, cdn_image_uri: Option<String>) {
        self.cdn_image_uri = cdn_image_uri;
    }

    pub fn get_cdn_animation_uri(&self) -> Option<String> {
        self.cdn_animation_uri.clone()
    }

    pub fn set_cdn_animation_uri(&mut self, cdn_animation_uri: Option<String>) {
        self.cdn_animation_uri = cdn_animation_uri;
    }

    pub fn get_image_optimizer_retry_count(&self) -> i32 {
        self.image_optimizer_retry_count
    }

    pub fn set_image_optimizer_retry_count(&mut self, image_optimizer_retry_count: i32) {
        self.image_optimizer_retry_count = image_optimizer_retry_count;
    }

    pub fn get_json_parser_retry_count(&self) -> i32 {
        self.json_parser_retry_count
    }

    pub fn set_json_parser_retry_count(&mut self, json_parser_retry_count: i32) {
        self.json_parser_retry_count = json_parser_retry_count;
    }
}

#[derive(Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[diesel(primary_key(token_uri))]
#[diesel(table_name = nft_metadata_crawler_uris)]
pub struct NFTMetadataCrawlerURIsQuery {
    pub token_uri: String,
    pub raw_image_uri: Option<String>,
    pub raw_animation_uri: Option<String>,
    pub cdn_json_uri: Option<String>,
    pub cdn_image_uri: Option<String>,
    pub cdn_animation_uri: Option<String>,
    pub image_optimizer_retry_count: i32,
    pub json_parser_retry_count: i32,
    pub last_updated: chrono::NaiveDateTime,
}

impl NFTMetadataCrawlerURIsQuery {
    pub fn get_by_token_uri(
        token_uri: String,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> diesel::QueryResult<Option<Self>> {
        nft_metadata_crawler_uris::table
            .find(token_uri)
            .first::<NFTMetadataCrawlerURIsQuery>(conn)
            .optional()
    }
}
