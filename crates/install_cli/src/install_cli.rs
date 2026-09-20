#[cfg(not(target_os = "windows"))]
mod install_cli_binary;
mod register_dez_scheme;

#[cfg(not(target_os = "windows"))]
pub use install_cli_binary::{InstallCliBinary, install_cli_binary};
pub use register_dez_scheme::{DEZ_URL_SCHEME, RegisterdezScheme, register_dez_scheme};

#[cfg(test)]
mod tests {
    #[test]
    fn registers_the_dez_url_scheme() {
        assert_eq!(super::DEZ_URL_SCHEME, "dez");
    }
}
