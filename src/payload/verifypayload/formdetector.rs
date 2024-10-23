use tl::{Node, Parser};

const FORM_ELEMENT_TYPES: [&str; 4] = ["input", "fieldset", "button", "textarea"];
pub fn get_form_data(html_body: &str) -> (usize, usize) {
    // let mut forms: Vec<tl::Node> = vec![];
    let dom = tl::parse(html_body, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let elements = dom.query_selector("form");
    if let Some(mut node_iter) = elements {
        let form_elements = node_iter
            .clone()
            .map(|handler| handler.get(parser).unwrap().clone())
            .map(|node| get_form_elements_count(&node, parser))
            .sum();
        let count = node_iter.by_ref().count();
        return (form_elements, count);
    }
    (0, 0)
}

pub fn get_form_elements_count(form_node: &Node, parser: &Parser) -> usize {
    form_node
        .as_tag()
        .unwrap()
        .children()
        .all(parser)
        .iter()
        .filter(|child| {
            if let Some(tag) = (**child).as_tag() {
                // println!("{}", &tag.raw().as_utf8_str());

                if FORM_ELEMENT_TYPES.contains(&tag.name().as_utf8_str().as_ref()) {
                    println!("{}", &tag.name().as_utf8_str().as_ref());
                    return true;
                }
                return false;
            }

            return false;
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::get_form_data;
    use rquest::{header, tls::Impersonate};
    use std::time::Instant;
    #[test]
    fn test_local_html() {
        let start = Instant::now();
        let html_body = r#"<!DOCTYPE html>
<html lang="en">
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
"#;
        let (forms, count) = get_form_data(html_body);
        println!("Time : {:?}", start.elapsed());
        assert_eq!(count, 2);
        assert_eq!(forms, 4);
    }

    #[tokio::test]
    async fn test_webpage() {
        let client = rquest::Client::builder()
            .impersonate(Impersonate::Chrome129)
            .brotli(true)
            .cookie_store(true)
            .build()
            .expect("Error");
        let html = client
            .get("https://www.booking.com/searchresults.html?ss=newark+&ssne=Las+Vegas&ssne_untouched=Las+Vegas&efdco=1&label=gen173nr-1FCAEoggI46AdIM1gEaKQCiAEBmAExuAEXyAEM2AEB6AEB-AECiAIBqAIDuAKeoNq4BsACAdICJDE5MjQzYmY4LTNhZjgtNDYzMi05NDMwLWQ0ZWQwMDI4MTJlY9gCBeACAQ&aid=304142&lang=en-us&sb=1&src_elem=sb&src=searchresults&group_adults=2&no_rooms=1&group_children=0")
            .header(header::REFERER, "https://www.booking.com")
            .send()
            .await
            .expect("error");
        let status = html.status().as_u16();
        // println!("{:?}", html.headers());
        if status != 200 {
            panic!("Status code was not 200. Status: {}", status);
        }
        let html_body = html.text().await.expect("error");
        // println!("{html_body:?}");
        let (num_form_elements, num_forms) = get_form_data(&html_body);
        assert_eq!(num_forms, 2);
        assert_eq!(num_form_elements, 23);

        // assert_eq!()
    }
}
