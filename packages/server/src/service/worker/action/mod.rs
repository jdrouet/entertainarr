mod sync_every_tvshow;
mod sync_tvshow;

#[derive(Debug)]
enum ActionParams {
    SyncEveryTVShow(sync_every_tvshow::SyncEveryTVShow),
    SyncTvShow(sync_tvshow::SyncTVShow),
}

#[derive(Debug)]
pub(crate) struct Action {
    params: ActionParams,
    pub(crate) retry: u8,
}

impl Action {
    pub fn sync_every_tvshow() -> Self {
        Self {
            params: ActionParams::SyncEveryTVShow(sync_every_tvshow::SyncEveryTVShow),
            retry: 0,
        }
    }

    pub fn sync_tvshow(tvshow_id: u64) -> Self {
        Self {
            params: ActionParams::SyncTvShow(sync_tvshow::SyncTVShow { tvshow_id }),
            retry: 0,
        }
    }

    pub(super) async fn execute(&self, ctx: &super::Context) -> Result<(), super::Error> {
        match self.params {
            ActionParams::SyncTvShow(ref inner) => inner.execute(ctx).await,
            ActionParams::SyncEveryTVShow(ref inner) => inner.execute(ctx).await,
        }
    }

    pub fn retry(self) -> Option<Self> {
        if self.retry > 10 {
            None
        } else {
            Some(Self {
                params: self.params,
                retry: self.retry + 1,
            })
        }
    }
}
