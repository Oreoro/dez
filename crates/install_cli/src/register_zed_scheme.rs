use client::ZED_URL_SCHEME;
use gpui::{AsyncApp, actions};
use release_channel::{RELEASE_CHANNEL, ReleaseChannel};

actions!(
    cli,
    [
        /// Registers the dez:// and legacy zed:// URL scheme handlers.
        RegisterZedScheme
    ]
);

pub async fn register_zed_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    let (dez_scheme, legacy_scheme) = match *RELEASE_CHANNEL {
        ReleaseChannel::Dev => ("dez-dev", "zed-dev"),
        ReleaseChannel::Nightly => ("dez-nightly", "zed-nightly"),
        ReleaseChannel::Preview => ("dez-preview", "zed-preview"),
        ReleaseChannel::Stable => ("dez", ZED_URL_SCHEME),
    };
    cx.update(|cx| cx.register_url_scheme(dez_scheme)).await?;
    cx.update(|cx| cx.register_url_scheme(legacy_scheme)).await
}
