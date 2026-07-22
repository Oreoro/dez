use gpui::{AsyncApp, actions};
use release_channel::{RELEASE_CHANNEL, ReleaseChannel};

actions!(
    cli,
    [
        /// Registers the isolated Dez URL scheme handler.
        RegisterDezScheme
    ]
);

pub async fn register_dez_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    let scheme = match *RELEASE_CHANNEL {
        ReleaseChannel::Dev => "dez-dev",
        ReleaseChannel::Nightly => "dez-nightly",
        ReleaseChannel::Preview => "dez-preview",
        ReleaseChannel::Stable => "dez",
    };
    cx.update(|cx| cx.register_url_scheme(scheme)).await
}
