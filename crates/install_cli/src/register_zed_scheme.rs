use gpui::{AsyncApp, actions};
use release_channel::RELEASE_CHANNEL;

actions!(
    cli,
    [
        /// Registers the isolated Dez URL scheme handler.
        RegisterDezScheme
    ]
);

pub async fn register_dez_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    let scheme = RELEASE_CHANNEL.url_scheme();
    cx.update(|cx| cx.register_url_scheme(scheme)).await
}
