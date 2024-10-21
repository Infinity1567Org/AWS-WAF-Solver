pub mod payload;

use payload::encryption::Encryptor;
use payload::fingerprint::Fingerprint;
use payload::metrics::MetricData;
use payload::pow::util::pow;
use serde::Serialize;

#[derive(Serialize)]
pub struct Signal {
    name: String,
    value: String,
}
#[derive(Serialize)]
pub struct VerifyChallengePayload {
    checksum: String,
    client: String,
    domain: String,
    existing_token: Option<String>,
    metrics: MetricData,
    signals: Vec<Signal>,
    solution: u32,
}

impl VerifyChallengePayload {}
