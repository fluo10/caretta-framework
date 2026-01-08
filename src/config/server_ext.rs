use iroh::{
    Endpoint,
    protocol::{Router, RouterBuilder},
};
use iroh_blobs::BlobsProtocol;
use iroh_docs::protocol::Docs;
use iroh_gossip::Gossip;

use crate::config::{P2pConfig, StorageConfig};

pub trait ServerConfigExt {
    fn p2p_config(&self) -> &P2pConfig;
    fn storage_config(&self) -> &StorageConfig;

    async fn to_iroh_router_builder(
        &self,
        app_name: &'static str,
    ) -> Result<(Endpoint, Docs, RouterBuilder), iroh::endpoint::BindError> {
        let p2p_config = self.p2p_config();
        let storage_config = self.storage_config();
        use iroh_blobs::BlobsProtocol;
        use iroh_gossip::Gossip;
        let endpoint = p2p_config.spawn_iroh_endpoint(app_name).await.unwrap();

        let iroh_dir = storage_config.to_iroh_path();
        let blobs = iroh_blobs::store::fs::FsStore::load(&iroh_dir.join("blobs"))
            .await
            .unwrap();
        let gossip = Gossip::builder().spawn(endpoint.clone());
        let docs_dir = iroh_dir.join("docs");
        std::fs::create_dir_all(&docs_dir).unwrap();
        let docs = Docs::persistent(docs_dir)
            .spawn(endpoint.clone(), blobs.clone().into(), gossip.clone())
            .await
            .unwrap();
        let mut router = Router::builder(endpoint.clone())
            .accept(iroh_blobs::ALPN, BlobsProtocol::new(&blobs, None))
            .accept(iroh_docs::ALPN, docs.clone())
            .accept(iroh_gossip::ALPN, gossip);
        #[cfg(feature = "server-devtools")]
        {
            router = router.accept(iroh_ping::ALPN, iroh_ping::Ping::new());
        }
        Ok((endpoint, docs, router))
    }
}
