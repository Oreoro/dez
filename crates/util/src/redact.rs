use std::sync::LazyLock;

static REDACT_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"([A-Z_][A-Z0-9_]*)=("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|\S+)"#).unwrap()
});

static REDACT_CLI_FLAG_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r#"(?i)(--[a-z0-9_-]*(?:api[-_]?key|token|password|secret|pass|credentials|license))(=|\s+)("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|\S+)"#,
    )
    .unwrap()
});

/// Whether a given environment variable name should have its value redacted
pub fn should_redact(env_var_name: &str) -> bool {
    const REDACTED_SUFFIXES: &[&str] = &[
        "KEY",
        "TOKEN",
        "PASSWORD",
        "SECRET",
        "PASS",
        "CREDENTIALS",
        "LICENSE",
    ];
    REDACTED_SUFFIXES
        .iter()
        .any(|suffix| env_var_name.ends_with(suffix))
}

/// Redact a string which could include a command with environment variables
pub fn redact_command(command: &str) -> String {
    let command = REDACT_REGEX
        .replace_all(command, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let value = &caps[2];
            if should_redact(var_name) {
                format!(r#"{}="[REDACTED]""#, var_name)
            } else {
                format!("{}={}", var_name, value)
            }
        })
        .to_string();
    REDACT_CLI_FLAG_REGEX
        .replace_all(&command, |caps: &regex::Captures| {
            format!(r#"{}{}"[REDACTED]""#, &caps[1], &caps[2])
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_string_with_multiple_env_vars() {
        let input = r#"failed to spawn command cd "/code/something" && ANTHROPIC_API_KEY="sk-ant-api03-WOOOO" COMMAND_MODE="unix2003" GEMINI_API_KEY="AIGEMINIFACE" HOME="/Users/foo""#;
        let result = redact_command(input);
        let expected = r#"failed to spawn command cd "/code/something" && ANTHROPIC_API_KEY="[REDACTED]" COMMAND_MODE="unix2003" GEMINI_API_KEY="[REDACTED]" HOME="/Users/foo""#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_redact_secret_cli_flags_without_masking_ordinary_arguments() {
        let input = r#"deploy --api-key=abc123 --mode check --access-token 'secret value' --license-file ./license.toml"#;
        let result = redact_command(input);
        let expected = r#"deploy --api-key="[REDACTED]" --mode check --access-token "[REDACTED]" --license-file ./license.toml"#;
        assert_eq!(result, expected);
    }
}
