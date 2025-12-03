pub mod network {
    use milli::update::new::indexer::sharding::Shards;

    use crate::network::Network;

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
}
