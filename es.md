# index

index -> database

## 新建index

```shell
PUT /my-index-000001
{
  "mappings": {
    "properties": {
      "age":    { "type": "integer" },  
      "email":  { "type": "keyword"  }, 
      "name":   { "type": "text"  }     
    }
  }
}
```

## 删除index

DELETE /index

## 添加文档

POST /<target>/_doc/

## 获取一个文档

GET /<index>/_doc/<_id>

## 查询所有?

GET /<index>/_search

## 批量操作

POST /_bulk

POST /<target>/_bulk

```json
{ "index" : { "_index" : "test", "_id" : "1" } }
{ "field1" : "value1" }
{ "delete" : { "_index" : "test", "_id" : "2" } }
{ "create" : { "_index" : "test", "_id" : "3" } }
{ "field1" : "value3" }
{ "update" : {"_id" : "1", "_index" : "test"} }
{ "doc" : {"field2" : "value2"} }
```

action取值为`index`, `create`, `delete`, 和 `update`

- `update`
    expects that the partial doc, upsert, and script and its options are specified on the next line.
