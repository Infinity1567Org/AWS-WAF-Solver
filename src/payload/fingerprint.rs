use crate::payload::formdetector::get_form_data;
use crate::payload::graphics::{Canvas, Gpu};
use crate::payload::metrics::Metrics;
use rand::seq::SliceRandom; // For random selection
use rand::thread_rng;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const OPERAND: f64 = -1.0e300;

#[derive(Debug, Clone)]
struct Resolution(i32, i32);

impl Resolution {
    fn new() -> Resolution {
        let resolutions: [Resolution; 4] = [
            Resolution(1366, 768),  // HD
            Resolution(1920, 1080), // Full HD
            Resolution(1600, 900),  // HD+
            Resolution(1440, 900),  // 16:10
        ];
        let mut rng = thread_rng();
        resolutions.choose(&mut rng).unwrap().clone()
    }

    fn construct_screeninfo_string(&self) -> String {
        format!("{}-{}-{}-24-*-*-*", self.0, self.1, self.1 - 40)
    }
}

#[derive(Debug, Serialize)]
struct Plugin {
    name: String,
    #[serde(rename = "str")]
    str_: String,
}

impl Plugin {
    fn generate_plugin_string(plugins: &Vec<Plugin>) -> String {
        let plugin_strings: Vec<String> =
            plugins.iter().map(|plugin| plugin.str_.clone()).collect();
        plugin_strings.join("")
    }
}

#[derive(Debug, Serialize)]
struct Capabilities {
    css: CssCapabilities,
    js: JsCapabilities,
    elapsed: u64,
}

impl Capabilities {
    fn new() -> Capabilities {
        Capabilities {
            css: CssCapabilities::new(),
            js: JsCapabilities::new(),
            elapsed: 0,
        }
    }
}

#[derive(Debug, Serialize)]
struct CssCapabilities {
    #[serde(rename = "textShadow")]
    text_shadow: u8,
    #[serde(rename = "WebkitTextStroke")]
    webkit_text_stroke: u8,
    #[serde(rename = "boxShadow")]
    box_shadow: u8,
    #[serde(rename = "borderRadius")]
    border_radius: u8,
    #[serde(rename = "borderImage")]
    border_image: u8,
    opacity: u8,
    transform: u8,
    transition: u8,
}

impl CssCapabilities {
    fn new() -> CssCapabilities {
        CssCapabilities {
            text_shadow: 1,
            webkit_text_stroke: 1,
            box_shadow: 1,
            border_radius: 1,
            border_image: 1,
            opacity: 1,
            transform: 1,
            transition: 1,
        }
    }
}

#[derive(Debug, Serialize)]
struct JsCapabilities {
    audio: bool,
    geolocation: bool,
    #[serde(rename = "localStorage")]
    local_storage: String,
    touch: bool,
    video: bool,
    #[serde(rename = "webWorker")]
    web_worker: bool,
}

impl JsCapabilities {
    fn new() -> JsCapabilities {
        JsCapabilities {
            audio: true,
            geolocation: true,
            local_storage: String::from("supported"),
            touch: false,
            video: true,
            web_worker: true,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Fingerprint {
    metrics: Metrics,
    start: u64,
    #[serde(rename = "flashVersion")]
    flash_version: Option<String>,
    plugins: Vec<Plugin>,
    #[serde(rename = "dupedPlugins")]
    duped_plugins: String,
    #[serde(rename = "screenInfo")]
    screen_info: String,
    referrer: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
    location: String,
    #[serde(rename = "webDriver")]
    web_driver: bool,
    capabilities: Capabilities,
    gpu: Gpu,
    dnt: Option<u8>,
    math: Math,
    automation: Automation,
    crypto: Crypto,
    canvas: Canvas,
    #[serde(rename = "formDetected")]
    form_detected: bool,
    #[serde(rename = "numForms")]
    num_forms: usize,
    #[serde(rename = "numFormElements")]
    num_form_elements: usize,
    be: Be,
    end: u64,
    errors: Vec<String>,
    version: String,
    id: String,
}

impl Fingerprint {
    pub fn new(
        referrer: String,
        user_agent: String,
        location: String,
        html_body: &str,
    ) -> Fingerprint {
        let math = Math::new();
        let plugins: Vec<Plugin> = vec![
            Plugin {
                name: String::from("PDF Viewer"),
                str_: String::from("PDF Viewer "),
            },
            Plugin {
                name: String::from("Chrome PDF Viewer"),
                str_: String::from("Chrome PDF Viewer "),
            },
            Plugin {
                name: String::from("Chromium PDF Viewer"),
                str_: String::from("Chromium PDF Viewer "),
            },
            Plugin {
                name: String::from("Microsoft Edge PDF Viewer"),
                str_: String::from("Microsoft Edge PDF Viewer "),
            },
            Plugin {
                name: String::from("WebKit built-in PDF"),
                str_: String::from("WebKit built-in PDF "),
            },
        ];
        let capabilities = Capabilities::new();
        let automation = Automation::new();
        let crypto = Crypto::new();
        let flash_version = None;
        let be = Be { si: false };
        let resolution = Resolution::new();
        let screen_info = resolution.construct_screeninfo_string();
        let duped_plugins = Plugin::generate_plugin_string(&plugins) + "||" + &screen_info;
        let canvas = Canvas::new();
        let (num_form_elements, num_forms) = get_form_data(html_body);
        let gpu = Gpu::new();
        let start = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()) as u64;
        Fingerprint {
            metrics: Metrics::new(),
            start,
            flash_version,
            plugins,
            duped_plugins,
            screen_info,
            referrer,
            user_agent,
            location,
            web_driver: false,
            capabilities,
            gpu,
            dnt: None,
            math,
            automation,
            crypto,
            canvas,
            form_detected: num_forms > 0,
            num_forms,
            num_form_elements,
            be,
            end: (start + 1),
            errors: vec![],
            version: String::from("2.3.0"),
            id: format!("{}", Uuid::new_v4()),
        }
    }
}

#[derive(Debug, Serialize)]
struct Math {
    tan: String,
    sin: String,
    cos: String,
}

impl Math {
    fn new() -> Math {
        Math {
            tan: String::from("-1.4214488238747245"),
            sin: String::from("0.8178819121159085"),
            cos: String::from("-0.5753861119575491"),
        }
    }
}

#[derive(Debug, Serialize)]
struct Automation {
    wd: WebDriver,
    phantom: Phantom,
}

impl Automation {
    fn new() -> Automation {
        Automation {
            wd: WebDriver::new(),
            phantom: Phantom::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct WebDriver {
    properties: Properties,
}

impl WebDriver {
    fn new() -> WebDriver {
        WebDriver {
            properties: Properties {
                document: Some(vec![]),
                window: vec![],
                navigator: Some(vec![]),
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct Phantom {
    properties: Properties,
}

impl Phantom {
    fn new() -> Phantom {
        Phantom {
            properties: Properties {
                document: None,
                window: vec![],
                navigator: None,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct Properties {
    #[serde(skip_serializing_if = "Option::is_none")]
    document: Option<Vec<String>>,
    window: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    navigator: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct Crypto {
    crypto: u8,
    subtle: u8,
    encrypt: bool,
    decrypt: bool,
    wrap_key: bool,
    unwrap_key: bool,
    sign: bool,
    verify: bool,
    digest: bool,
    derive_bits: bool,
    derive_key: bool,
    get_random_values: bool,
    random_uuid: bool,
}

impl Crypto {
    fn new() -> Crypto {
        Crypto {
            crypto: 1,
            subtle: 1,
            encrypt: true,
            decrypt: true,
            wrap_key: true,
            unwrap_key: true,
            sign: true,
            verify: true,
            digest: true,
            derive_bits: true,
            derive_key: true,
            get_random_values: true,
            random_uuid: true,
        }
    }
}

#[derive(Debug, Serialize)]
struct Be {
    si: bool,
}

#[cfg(test)]
mod tests {
    use super::Fingerprint;
    use std::time::Instant;
    #[test]
    fn test_payload() {
        let start = Instant::now();
        let result = Fingerprint::new(
            String::from("https://huggingface.co/login"),
            String::from(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0",
            ),
            String::from("https://huggingface.co/login"),
            r#"<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test Forms</title>
</head>
<body>
    <h1>Form Test</h1>

    <form id="form1">
        <label for="input1">Input 1:</label>
        <input type="text" id="input1" name="input1">
        <button type="submit">Submit Form 1</button>
    </form>

    <form id="form2">
        <label for="input2">Input 2:</label>
        <input type="text" id="input2" name="input2">
        <button type="submit">Submit Form 2</button>
    </form>

    <div>
        <p>This is a sample HTML document with two form elements for testing.</p>
    </div>
</body>
</html>
"#,
        );
        let serialized = serde_json::to_string(&result).unwrap();
        println!("Time : {:?}", start.elapsed());
        println!("{serialized:#}");

        // println!("{}",result.canvas.histogram_bins.len())
    }
}
