use std::{thread, time};

use elasticsearch::{Elasticsearch, Error, SearchParts, IndexParts, http::{Method, headers::HeaderMap, response}, DeleteParts, BulkParts, BulkIndexOperation, BulkOperation};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Add_one_doc_result{
    _id: String,
    _index: String,
    _primary_term: u32,
    _seq_no: u32,
    _version: u64,
    result: String,
    _shards: Shards
}

#[derive(Debug, Serialize, Deserialize)]
struct Shards{
    total: u8,
    successful: u8,
    failed: u8
}

#[derive(Debug, Serialize, Deserialize)]
struct Person{
    name: String,
    age: u8
}

async fn crate_index(client: &Elasticsearch, index: &str) {
    let body = r##"{"settings":{"index":{"number_of_shards":"5","number_of_replicas":"1"}}}"##;
    let response = client
        .send(
            Method::Put,
            index,
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body),
            None,
        )
        .await.unwrap();
    println!("{}", response.json::<Value>().await.unwrap());
}

async fn add_one_doc(client: &Elasticsearch, index: &str, data: &str) -> Result<Add_one_doc_result, Box<dyn std::error::Error>> {
    match serde_json::from_str::<Value>(data) {
        Ok(data_json) => {
            let response2 = client.index(IndexParts::Index(index)).body(data_json).send().await?;
            let result = response2.json::<Add_one_doc_result>().await?;
            println!("id: {}", result._id);
            Ok(result)
        }

        Err(e) => {
            Err(Box::new(e))
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Elasticsearch::default();

    // crate_index(&client, "book").await;

    // let body = r#"{"age": 1000}"#;

    // let result = add_one_doc(&client, "book", body).await?;
    // let response = client.delete(DeleteParts::IndexId("book", result._id.as_ref())).send().await?;
    // println!("{}", response.text().await?);


    // let response = client.search(SearchParts::Index(&["book"])).send().await?;

    // let v = response.json::<Value>().await?;
    // println!("{}", v);

    let mut ops: Vec<BulkOperation<Value>> = vec![];
    ops.push(BulkOperation::index(json!({
        "name": "赵云"
    })).into());

    ops.push(BulkOperation::index(json!({
        "name": "关羽"
    })).into());

    ops.push(BulkOperation::index(json!({
        "name": "张飞"
    })).into());

    ops.push(BulkOperation::index(json!({
        "name": "马超"
    })).into());

    ops.push(BulkOperation::update("L2DybIIBnHjWqk6BzWxl", json!({
        "doc":{
            "age": 500
        }
    })).into());

    let response = client.bulk(BulkParts::Index("book")).body(ops).send().await?;

    println!("{}", response.json::<Value>().await?);

    Ok(())
}