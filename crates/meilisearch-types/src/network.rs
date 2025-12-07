use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "enterprise")]
use milli::update::new::indexer::enterprise_edition::sharding::Shards;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    #[serde(default, rename = "self")]
    pub local: Option<String>,
    #[serde(default)]
    pub remotes: BTreeMap<String, Remote>,
    #[serde(default)]
    pub sharding: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Remote {
    pub url: String,
    #[serde(default)]
    pub search_api_key: Option<String>,
    #[serde(default)]
    pub write_api_key: Option<String>,
}

#[cfg(feature = "enterprise")]
impl Network {
    pub fn shards(&self) -> Option<Shards> {
        if self.sharding {
            let this = self.local.as_deref().expect("Inconsistent `sharding` and `self`");
            // Pre-allocate to reduce allocations
            let mut others = Vec::with_capacity(self.remotes.len().saturating_sub(1));
            for name in self.remotes.keys() {
                if name.as_str() != this {
                    others.push(name.clone());
                }
            }
            // Use Box::leak to avoid creating new allocations on each call
            // This trades memory reuse for initial allocation cost
            Some(Shards { 
                own: Box::leak(vec![this.to_string()].into_boxed_slice()).to_vec(), 
                others 
            })
        } else {
            None
        }
    }

    pub fn sharding(&self) -> bool {
        self.sharding
    }
}
