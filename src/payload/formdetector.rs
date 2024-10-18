use tl::{Node, Parser};
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
    (5, 5)
}

pub fn get_form_elements_count(form_node: &Node, parser: &Parser) -> usize {
    form_node
        .children()
        .unwrap()
        .top()
        .iter()
        .filter(|child| (**child).get(parser).unwrap().as_tag().is_some())
        .count()
}
#[cfg(test)]
mod tests {
    use super::get_form_data;
    use std::time::Instant;
    use rquest::tls::Impersonate;
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
        assert_eq!(forms,6);
        // let dom = tl::parse(html_body, tl::ParserOptions::default()).unwrap();
        // let parser = dom.parser();
        // let form_children = forms[0].children().unwrap();
        // for f in form_children.top().iter() {
        //     if let Some(tag) = f.get(parser).unwrap().as_tag() {
        //         println!("{:#?}",tag);
        //     }

        // }
    }

    #[tokio::test]
    async fn test_webpage() {
        let client = rquest::Client::builder()
        .impersonate(Impersonate::Chrome129)
        .build();
        client.as_ref().expect("Error").get("https://www.caesars.com/sportsbook-and-casino/nj/").send().await.expect("error");
        let html = client.expect("Error").get("https://sportsbook.caesars.com/us/nj/bet/registration?bc=CZR1000&utm_urlreferrer=https%3A%2F%2Fwww.caesars.com%2Fsportsbook-and-casino%2Fnj%2F").send().await.expect("error");
        let status = html.status().as_u16();
        if status!=200 {
            panic!("Status code was not 200. Status: {}",status);
        }
        let html_body = html.text().await.expect("error");
        println!("{}",html_body);
        let (num_form_elements,num_forms) = get_form_data(&html_body);
        assert_eq!(num_forms,1);
        // assert_eq!(num_form_elements,3);
        
        // assert_eq!()
    }
}
