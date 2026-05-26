use zyblog::config::Config;

#[test]
fn config_from_env_succeeds() {
    let config = Config::from_env();
    assert!(config.is_ok());
}

#[test]
fn config_has_required_fields() {
    let config = Config::from_env().unwrap();

    assert!(!config.server_addr.is_empty());
    assert!(!config.database_url.is_empty());
    assert!(config.smtp_port > 0);
}

#[test]
fn config_from_env_with_custom_values() {
    std::env::set_var("SERVER_ADDR", "127.0.0.1:3000");
    std::env::set_var("SMTP_PORT", "465");
    std::env::set_var("SMTP_FROM", "custom@example.com");

    let config = Config::from_env().unwrap();

    assert_eq!(config.server_addr, "127.0.0.1:3000");
    assert_eq!(config.smtp_port, 465);
    assert_eq!(config.smtp_from, "custom@example.com");

    std::env::remove_var("SERVER_ADDR");
    std::env::remove_var("SMTP_PORT");
    std::env::remove_var("SMTP_FROM");
}

#[test]
fn config_from_env_invalid_port_falls_back() {
    std::env::set_var("SMTP_PORT", "not-a-number");

    let config = Config::from_env().unwrap();

    assert_eq!(config.smtp_port, 587);

    std::env::remove_var("SMTP_PORT");
}

#[test]
fn config_is_cloneable() {
    let config = Config::from_env().unwrap();
    let config2 = config.clone();

    assert_eq!(config.server_addr, config2.server_addr);
    assert_eq!(config.smtp_port, config2.smtp_port);
}
