use crate::manifest::yaml::Network;
use crate::types::code::Code;

/// Generates the provider name for a given network.
///
/// # Arguments
///
/// * `network` - A reference to the `Network` configuration.
///
/// # Returns
///
/// A `String` representing the provider name.
fn network_provider_name(network: &Network) -> String {
    network_provider_name_from_name(&network.name)
}

/// Generates the provider name from the network name.
///
/// # Arguments
///
/// * `network_name` - The name of the network.
///
/// # Returns
///
/// A `String` representing the provider name.
fn network_provider_name_from_name(network_name: &str) -> String {
    format!(
        "{network_name}_PROVIDER",
        network_name = network_name.to_uppercase()
    )
}

/// Generates the function name for the network provider.
///
/// # Arguments
///
/// * `network` - A reference to the `Network` configuration.
///
/// # Returns
///
/// A `String` representing the function name for the network provider.
pub fn network_provider_fn_name(network: &Network) -> String {
    format!(
        "get_{fn_name}",
        fn_name = network_provider_name(network).to_lowercase()
    )
}

/// Generates the function name for the network provider from the network name.
///
/// # Arguments
///
/// * `network_name` - The name of the network.
///
/// # Returns
///
/// A `String` representing the function name for the network provider.
pub fn network_provider_fn_name_by_name(network_name: &str) -> String {
    format!(
        "get_{fn_name}",
        fn_name = network_provider_name_from_name(network_name).to_lowercase()
    )
}

/// Generates the lazy provider code for a given network.
///
/// # Arguments
///
/// * `network` - A reference to the `Network` configuration.
///
fn generate_network_lazy_provider_code(network: &Network) -> Code {
    Code::new(format!(
        r#"
            static ref {network_name}: Arc<Provider<RetryClient<Http>>> = create_retry_client("{network_url}", {compute_units_per_second}).expect("Error creating provider");
        "#,
        network_name = network_provider_name(network),
        network_url = network.url,
        compute_units_per_second =
            if let Some(compute_units_per_second) = network.compute_units_per_second {
                format!("Some({})", compute_units_per_second)
            } else {
                "None".to_string()
            }
    ))
}

/// Generates the provider function code for a given network.
///
/// # Arguments
///
/// * `network` - A reference to the `Network` configuration.
///
fn generate_network_provider_code(network: &Network) -> Code {
    Code::new(format!(
        r#"
            pub fn {fn_name}() -> Arc<Provider<RetryClient<Http>>> {{
                {provider_lazy_name}.clone()
            }}
        "#,
        fn_name = network_provider_fn_name(network),
        provider_lazy_name = network_provider_name(network)
    ))
}

/// Generates the code for all network providers.
///
/// # Arguments
///
/// * `networks` - A reference to a slice of `Network` configurations.
///
/// # Returns
///
/// The generated network providers code.
pub fn generate_networks_code(networks: &[Network]) -> Code {
    let mut output = Code::new(r#"
            /// THIS IS A GENERATED FILE. DO NOT MODIFY MANUALLY.
            ///
            /// This file was auto generated by rindexer - https://github.com/joshstevens19/rindexer.
            /// Any manual changes to this file will be overwritten.
            
            use ethers::providers::{Provider, Http, RetryClient};
            use rindexer_core::lazy_static;
            use rindexer_core::provider::create_retry_client;
            use std::sync::Arc;

            lazy_static! {
        "#
    .to_string());

    for network in networks {
        output.push_str(&generate_network_lazy_provider_code(network));
    }

    output.push_str(&Code::new("}".to_string()));

    for network in networks {
        output.push_str(&generate_network_provider_code(network));
    }

    output
}
