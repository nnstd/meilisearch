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
            let others = self
                .remotes
                .keys()
                .filter(|name| name.as_str() != this)
                .map(|name| name.to_owned())
                .collect();
            Some(Shards { own: vec![this.to_owned()], others })
        } else {
            None
        }
    }

    pub fn sharding(&self) -> bool {
        self.sharding
    }
}
