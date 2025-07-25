use separator::{separated_float, separated_int, separated_uint_with_output, Separatable};
use spectre_consensus_core::constants::*;
use spectre_consensus_core::network::NetworkType;

#[inline]
pub fn sompi_to_spectre(sompi: u64) -> f64 {
    sompi as f64 / SOMPI_PER_SPECTRE as f64
}

#[inline]
pub fn spectre_to_sompi(spectre: f64) -> u64 {
    (spectre * SOMPI_PER_SPECTRE as f64) as u64
}

#[inline]
pub fn sompi_to_spectre_string(sompi: u64) -> String {
    sompi_to_spectre(sompi).separated_string()
}

#[inline]
pub fn sompi_to_spectre_string_with_trailing_zeroes(sompi: u64) -> String {
    separated_float!(format!("{:.8}", sompi_to_spectre(sompi)))
}

pub fn spectre_suffix(network_type: &NetworkType) -> &'static str {
    match network_type {
        NetworkType::Mainnet => "SPR",
        NetworkType::Testnet => "TSPR",
        NetworkType::Simnet => "SSPR",
        NetworkType::Devnet => "DSPR",
    }
}

#[inline]
pub fn sompi_to_spectre_string_with_suffix(sompi: u64, network_type: &NetworkType) -> String {
    let spr = sompi_to_spectre_string(sompi);
    let suffix = spectre_suffix(network_type);
    format!("{spr} {suffix}")
}
