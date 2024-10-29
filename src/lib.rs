pub mod payload;
use payload::verifypayload::fingerprint::Fingerprint;
use payload::verifypayload::metrics::MetricData;
use payload::verifypayload::pow::util::pow;
use payload::verifypayload::{encryption::Encryptor, pow::util::HashType};
use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;
#[derive(Serialize, Debug)]
pub struct Signal {
    name: String,
    value: KramerRio,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GokuProps {
    key: String,
    iv: String,
    context: String,
}
#[derive(Deserialize, Debug)]
pub struct VerifyResponse {
    token: String,
    inputs: Option<ChallengeInput>,
}
#[derive(Serialize, Debug)]
pub struct KramerRio {
    #[serde(rename = "Present")]
    present: String,
}
#[derive(Serialize, Debug)]
pub struct VerifyChallengePayload {
    challenge: ChallengeObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    goku_props: Option<GokuProps>,
    checksum: String,
    client: String,
    domain: String,
    existing_token: Option<String>,
    metrics: Vec<MetricData>,
    signals: Vec<Signal>,
    solution: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeInput {
    challenge: ChallengeObject,
    challenge_type: String,
    difficulty: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeObject {
    input: String,
    hmac: String,
    region: String,
}

impl VerifyChallengePayload {
    pub async fn new(
        user_agent: String,
        location: &str,
        referrer: &str,
        challenge_input: ChallengeInput,
        existing_token: Option<String>,
        html_body: &str,
        goku_props: Option<GokuProps>,
    ) -> Result<Self, String> {
        let payload = Fingerprint::new(
            referrer.to_string(),
            user_agent,
            location.to_string(),
            html_body,
        );
        let serialized = serde_json::to_string(&payload).unwrap();
        // println!("{serialized}");
        let payload_string = Encryptor::construct_payload_string(&serialized);
        let checksum = Encryptor::calculate_checksum(&serialized);
        let mut signals: Vec<Signal> = vec![];
        signals.push(Signal {
            name: String::from("KramerAndRio"),
            value: KramerRio {
                present: Encryptor::encrypt(
                    payload_string.as_bytes(),
                    "93d9f6846b629edb2bdc4466af627d998496cb0c08f9cf043de68d6b25aa9693",
                ),
            },
        });
        let hash_type: Option<HashType> = match challenge_input.challenge_type.as_str() {
            "h72f957df656e80ba55f5d8ce2e8c7ccb59687dba3bfb273d54b08a261b2f3002" => {
                Some(HashType::Scrypt)
            }
            "h7b0c470f0cfe3a80a9e26526ad185f484f6817d0832712a4a37a908786a6a67f" => {
                Some(HashType::Sha256)
            }

            _ => None,
        };
        match hash_type {
            Some(algorithm) => {
                let solution = pow(
                    algorithm,
                    &challenge_input.challenge.input,
                    checksum.to_string().as_str(),
                    challenge_input.difficulty,
                )
                .await;
                let domain = Url::parse(&location).unwrap().domain().unwrap().to_string();
                if let Ok(answer) = solution {
                    let body = Self {
                        goku_props: goku_props,
                        challenge: challenge_input.challenge,
                        checksum: checksum.to_string(),
                        client: String::from("Browser"),
                        domain: "www.".to_string() + domain.as_str(),
                        existing_token,
                        metrics: MetricData::metric_to_metric_data(payload.metrics),
                        signals,
                        solution: answer,
                    };

                    return Ok(body);
                } else {
                    return Err("Could not calculate pow".to_string());
                }
            }
            None => return Err("Challenge type not supported".to_string()),
        }
    }

    pub fn get_goku_props(html_body: &str) -> Option<GokuProps> {
        let re = Regex::new(
            r#"window\.gokuProps\s*=\s*(\{\s*"key"\s*:\s*"[^"]+"\s*,\s*"iv"\s*:\s*"[^"]+"\s*,\s*"context"\s*:\s*"[^"]+"\s*\});"#,
        )
        .unwrap();
        if let Some(caps) = re.captures(html_body) {
            if let Some(matched) = caps.get(1) {
                println!("{}", matched.as_str());
                return Some(serde_json::from_str(matched.as_str()).unwrap());
            } else {
                return None;
            }
        }
        return None;
    }
}

#[cfg(test)]

mod test {

    use crate::GokuProps;

    use super::{VerifyChallengePayload, VerifyResponse};
    use rquest::{
        header::{self},
        tls::Impersonate,
        Response,
    };

    async fn make_login_req(client: &rquest::Client, token: &str) -> Response {
        let params = [("username", "test@gmail.com"), ("password", "tester")];
        let login_response = client
            .post("https://huggingface.co/login")
            .form(&params)
            .header(header::REFERER, "https://huggingface.co/login")
            .header(header::COOKIE, format!("aws-waf-token={};", token))
            .send()
            .await
            .unwrap();
        login_response
    }

    async fn get_token(
        client: &rquest::Client,
        html_body: &str,
        referrer: &str,
        goku_props: Option<GokuProps>,
        existing_token: Option<String>,
    ) -> Response {
        let challenge_inputs = client.get("https://de5282c3ca0c.91803d22.us-east-1.token.awswaf.com/de5282c3ca0c/526cf06acb0d/inputs?client=browser").send().await.expect("Could not get inputs").text().await.expect("Error getting input json");
        let challenge_struct = serde_json::from_str(&challenge_inputs).unwrap();
        let user_agent = String::from(client.user_agent().unwrap().to_str().expect("Error"));
        let payload = VerifyChallengePayload::new(
            user_agent,
            "https://huggingface.co/login",
            referrer,
            challenge_struct,
            existing_token,
            html_body,
            goku_props,
        )
        .await
        .expect("Error building payload");
        // println!("{:?}", serde_json::to_string(&payload).unwrap());
        let response = client.post("https://de5282c3ca0c.91803d22.us-east-1.token.awswaf.com/de5282c3ca0c/526cf06acb0d/verify").json(&payload).send().await.unwrap();
        response
    }
    #[tokio::test]
    async fn test_verify() {
        let client = rquest::Client::builder()
            .impersonate(Impersonate::Chrome129)
            .brotli(true)
            .cookie_store(true)
            .build()
            .expect("Error");
        let referrer = "https://huggingface.co";
        let html = client
            .get("https://huggingface.co/login")
            .header(header::REFERER, referrer)
            .send()
            .await
            .expect("error");
        assert_eq!(html.status().as_u16(), 200);
        let html_body = html.text().await.unwrap();
        let response = get_token(&client, &html_body, referrer, None, None).await;
        assert_eq!(response.status().as_u16(), 200);
        let response_json: VerifyResponse = response.json().await.unwrap();
        println!("{:?}", response_json);
        let token = response_json.token;
        println!("{}", token);
        let login_response = make_login_req(&client, &token).await;
        assert_ne!(login_response.status().as_u16(), 403);
        if login_response.status().as_u16() == 202 {
            let response_body = login_response.text().await.unwrap();
            println!("{}", response_body);
            let goku_props = VerifyChallengePayload::get_goku_props(&response_body);
            // println!(
            //     "{:?}",
            //     goku_props.as_ref().expect("Could not parse goku props")
            // );
            let captcha_response =
                get_token(&client, &html_body, referrer, goku_props, Some(token));

            let token_json: VerifyResponse = captcha_response.await.json().await.unwrap();
            println!("{:?}", token_json);
            let new_login_response = make_login_req(&client, &token_json.token);

            println!(
                "Status after solved cap : {}",
                new_login_response.await.status().as_u16()
            );
        }
    }
}
