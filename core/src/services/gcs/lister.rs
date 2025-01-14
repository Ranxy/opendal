// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::sync::Arc;

use async_trait::async_trait;
use serde_json;

use super::core::*;
use super::error::parse_error;
use crate::raw::*;
use crate::*;

/// GcsLister takes over task of listing objects and
/// helps walking directory
pub struct GcsLister {
    core: Arc<GcsCore>,

    path: String,
    delimiter: &'static str,
    limit: Option<usize>,

    /// Filter results to objects whose names are lexicographically
    /// **equal to or after** startOffset
    start_after: Option<String>,
}

impl GcsLister {
    /// Generate a new directory walker
    pub fn new(
        core: Arc<GcsCore>,
        path: &str,
        recursive: bool,
        limit: Option<usize>,
        start_after: Option<&str>,
    ) -> Self {
        let delimiter = if recursive { "" } else { "/" };
        Self {
            core,

            path: path.to_string(),
            delimiter,
            limit,
            start_after: start_after.map(String::from),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl oio::PageList for GcsLister {
    async fn next_page(&self, ctx: &mut oio::PageContext) -> Result<()> {
        let resp = self
            .core
            .gcs_list_objects(
                &self.path,
                &ctx.token,
                self.delimiter,
                self.limit,
                if ctx.token.is_empty() {
                    self.start_after.clone()
                } else {
                    None
                },
            )
            .await?;

        if !resp.status().is_success() {
            return Err(parse_error(resp).await?);
        }
        let bytes = resp.into_body().bytes().await?;

        let output: ListResponse =
            serde_json::from_slice(&bytes).map_err(new_json_deserialize_error)?;

        if let Some(token) = &output.next_page_token {
            ctx.token = token.clone();
        } else {
            ctx.done = true;
        }

        for prefix in output.prefixes {
            let de = oio::Entry::new(
                &build_rel_path(&self.core.root, &prefix),
                Metadata::new(EntryMode::DIR),
            );

            ctx.entries.push_back(de);
        }

        for object in output.items {
            // exclude the inclusive start_after itself
            let path = build_rel_path(&self.core.root, &object.name);
            if path == self.path || path.is_empty() {
                continue;
            }
            if self.start_after.as_ref() == Some(&path) {
                continue;
            }

            let mut meta = Metadata::new(EntryMode::from_path(&path));

            // set metadata fields
            meta.set_content_md5(object.md5_hash.as_str());
            meta.set_etag(object.etag.as_str());

            let size = object.size.parse().map_err(|e| {
                Error::new(ErrorKind::Unexpected, "parse u64 from list response").set_source(e)
            })?;
            meta.set_content_length(size);
            if !object.content_type.is_empty() {
                meta.set_content_type(&object.content_type);
            }

            meta.set_last_modified(parse_datetime_from_rfc3339(object.updated.as_str())?);

            let de = oio::Entry::with(path, meta);

            ctx.entries.push_back(de);
        }

        Ok(())
    }
}
