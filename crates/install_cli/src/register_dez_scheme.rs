use gpui::{AsyncApp, actions};

pub const DEZ_URL_SCHEME: &str = "dez";

actions!(
    cli,
    [
        /// Registers the dez:// URL scheme handler.
        RegisterdezScheme
    ]
);

pub async fn register_dez_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    cx.update(|cx| cx.register_url_scheme(DEZ_URL_SCHEME))
        .await
}
