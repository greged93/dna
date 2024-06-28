use std::{sync::Arc, time::Duration};

use apibara_dna_common::{core::Cursor, ingestion::CursorProvider};
use error_stack::{Result, ResultExt};
use futures_util::TryFutureExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::warn;

use crate::ingestion::models::BlockIdExt;

use super::{
    models::{self, BlockExt},
    provider::JsonRpcProviderError,
    JsonRpcProvider,
};

pub struct EvmCursorProvider {
    provider: Arc<JsonRpcProvider>,
}

impl EvmCursorProvider {
    pub fn new(provider: JsonRpcProvider) -> Self {
        Self {
            provider: provider.into(),
        }
    }
}

#[async_trait::async_trait]
impl CursorProvider for EvmCursorProvider {
    type Error = JsonRpcProviderError;
    type CursorStream = ReceiverStream<Cursor>;

    async fn subscribe_head(&self) -> Result<Self::CursorStream, Self::Error> {
        let head = get_block_id(
            &self.provider,
            models::BlockId::Number(models::BlockNumberOrTag::Latest),
        )
        .await?;

        let (tx, rx) = mpsc::channel(1024);

        tokio::spawn(
            poll_block_id(
                self.provider.clone(),
                models::BlockId::Number(models::BlockNumberOrTag::Latest),
                tx,
            )
            .inspect_err(|err| {
                warn!(error = ?err, "head block poll task failed");
            }),
        );

        Ok(ReceiverStream::new(rx))
    }

    /// Subscribe to changes to the current finalized block.
    async fn subscribe_finalized(&self) -> Result<Self::CursorStream, Self::Error> {
        let finalized = get_block_id(
            &self.provider,
            models::BlockId::Number(models::BlockNumberOrTag::Finalized),
        )
        .await?;

        let (tx, rx) = mpsc::channel(1024);

        tokio::spawn(
            poll_block_id(
                self.provider.clone(),
                models::BlockId::Number(models::BlockNumberOrTag::Finalized),
                tx,
            )
            .inspect_err(|err| {
                warn!(error = ?err, "finalized block poll task failed");
            }),
        );

        Ok(ReceiverStream::new(rx))
    }

    /// Returns the cursor of the parent of the provided cursor.
    async fn get_parent_cursor(&self, cursor: &Cursor) -> Result<Cursor, Self::Error> {
        let block_id = cursor.to_block_id();
        let block = self
            .provider
            .get_block(block_id)
            .await?
            .ok_or(JsonRpcProviderError)
            .attach_printable("missing block")?;
        let hash = block.header.parent_hash.to_vec();
        Ok(Cursor::new(cursor.number - 1, hash))
    }
}

async fn poll_block_id(
    provider: Arc<JsonRpcProvider>,
    id: models::BlockId,
    tx: mpsc::Sender<Cursor>,
) -> Result<(), JsonRpcProviderError> {
    let mut previous = get_block_id(&provider, id).await?;

    tx.send(previous.clone())
        .await
        .change_context(JsonRpcProviderError)?;

    loop {
        let cursor = get_block_id(&provider, id).await?;

        if cursor != previous {
            tx.send(cursor.clone())
                .await
                .change_context(JsonRpcProviderError)?;
            previous = cursor;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn get_block_id(
    provider: &JsonRpcProvider,
    id: models::BlockId,
) -> Result<Cursor, JsonRpcProviderError> {
    let cursor = provider
        .get_block(id)
        .await?
        .ok_or(JsonRpcProviderError)
        .attach_printable("missing block")?
        .cursor()
        .ok_or(JsonRpcProviderError)
        .attach_printable("missing block cursor")?;
    Ok(cursor)
}
