use anyhow::anyhow;
use shared::persistence::level::Level;

use crate::consts::DATABASE_NAME;
use crate::types::upload_metadata::UploadMetadata;

pub struct DB {
    inner: Level,
}

impl DB {
    pub fn new() -> anyhow::Result<Self> {
        let inner = Level::open(DATABASE_NAME)?;
        Ok(Self { inner })
    }

    pub fn store_upload_metadata(&mut self, metadata: &UploadMetadata) -> anyhow::Result<()> {
        self.inner
            .put(&metadata.root_hash, &serde_json::to_string(&metadata)?)?;

        Ok(())
    }

    pub fn list_upload_metadata(&mut self) -> anyhow::Result<Vec<UploadMetadata>> {
        let previous_uploads = self.inner.read()?;

        let mut res = vec![];
        for (_key, val) in previous_uploads {
            let val_str: String =
                String::from_utf8(val).map_err(|_| anyhow!("Failed to decode key"))?;
            let val: UploadMetadata = serde_json::from_str(&val_str)
                .map_err(|_| anyhow!("Failed to decode upload_metadata"))?;

            res.push(val);
        }

        Ok(res)
    }

    pub fn get_upload_metadata(&mut self, hash: &str) -> anyhow::Result<UploadMetadata> {
        let val = self.inner.get(hash)?;

        let retrieved_string: String =
            String::from_utf8(val).map_err(|_| anyhow!("Failed to retrieve metadata"))?;

        let metadata: UploadMetadata = serde_json::from_str(&retrieved_string)
            .map_err(|_| anyhow!("Failed to decode metadata"))?;

        Ok(metadata)
    }
}
