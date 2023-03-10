fn main() {
    let cfg_override = anchor_cli::config::ConfigOverride::default();
    let r = anchor_cli::build(
        &cfg_override,
        None,
        None,
        false,
        false,
        None,
        None,
        None,
        anchor_cli::config::BootstrapMode::None,
        None,
        None,
        vec![],
        true,
    ).unwrap();
    println!("{:?}",r);
    println!("ss111s12232");
}
