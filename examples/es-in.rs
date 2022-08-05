use elasticsearch::{Elasticsearch, Error, SearchParts, IndexParts, http::{Method, headers::HeaderMap, response}};
use serde_json::{json, Value};


async fn add_one_doc(client: Elasticsearch, index: &str, data: String) {
    let data_json: Value = data.into();

    let response2 = client.index(IndexParts::Index(index)).body(data_json).send().await.unwrap();
    println!("{}", response2.json::<Value>().await.unwrap());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Elasticsearch::default();
    let body = r##"{"settings":{"index":{"number_of_shards":"5","number_of_replicas":"1"}}}"##;
    let response = client
        .send(
            Method::Put,
            "/book",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body),
            None,
        )
        .await?;
    println!("{}", response.json::<Value>().await?);

    let body2 = r##"{
          "properties": {
            "name":    { "type": "keyword" }
          }
      }"##;
    let response2 = client.send(Method::Put, "/book/_mapping", HeaderMap::new(), Option::<&Value>::None, Some(body2), None).await?;
    println!("r2: {}", response2.json::<Value>().await?);
    
    let response3 = client.send(Method::Post, "/book/_doc", HeaderMap::new(), Option::<&Value>::None, Some(body), None).await?;
    println!("r3: {}", response3.json::<Value>().await?);

    let body = b"{\"query\":{\"match_all\":{}}}";

    let response = client
    .send(Method::Post,
        SearchParts::Index(&["book"]).url().as_ref(),
        HeaderMap::new(),
        Option::<&Value>::None,
        Some(body.as_ref()),
        None,
    )
    .await?;

    println!("{}", response.json::<Value>().await?);   

    Ok(())
}