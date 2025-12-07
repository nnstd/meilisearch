pub mod network {
    use milli::update::new::indexer::sharding::Shards;

    use crate::network::Network;

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
                Some(Shards { 
                    own: vec![this.to_string()], 
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
}
