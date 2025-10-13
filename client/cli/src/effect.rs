use entertainarr_client_core::Effect;

pub fn process_effect(
    core: &crate::Core,
    effect: Effect,
    tx: tokio::sync::mpsc::UnboundedSender<Effect>,
) -> anyhow::Result<()> {
    match effect {
        render @ Effect::Render(_) => {
            tx.send(render).map_err(|err| anyhow::anyhow!("{err:?}"))?;
        } // Effect::Http(mut request) => {
          //     tokio::spawn({
          //         let core = core.clone();
          //         let tx = tx.clone();

          //         async move {
          //             let response = http::request(&request.operation).await;

          //             for effect in core.resolve(&mut request, response.into())? {
          //                 process_effect(&core, effect, &tx)?;
          //             }
          //             anyhow::Result::<()>::Ok(())
          //         }
          //     });
          // }
          // Effect::ServerSentEvents(mut request) => {
          //     spawn({
          //         let core = core.clone();
          //         let tx = tx.clone();

          //         async move {
          //             let mut stream = sse::request(&request.operation).await?;

          //             while let Ok(Some(response)) = stream.try_next().await {
          //                 for effect in core.resolve(&mut request, response)? {
          //                     process_effect(&core, effect, &tx)?;
          //                 }
          //             }
          //             Result::<()>::Ok(())
          //         }
          //     });
          // }
    }
    Ok(())
}
