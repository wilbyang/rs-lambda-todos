use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::{Arc};

use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput, UpdateItemInput};
use uuid::Uuid;

use crate::todos::domain::Todo;

pub(crate) type Db = Arc<DDBRepo>;

// ddb(dynamodb) as repo
pub struct DDBRepo {
    client: DynamoDbClient,
    table_name: String,
}

impl DDBRepo {
    pub fn new() -> Self {
        let table_name = env::var("TODO_TABLE_NAME").expect("DYNAMODB_TABLE_NAME must be set");
        let region = env::var("AWS_REGION").expect("AWS_REGION must be set");
        let client = DynamoDbClient::new(Region::from_str(&region).unwrap());
        Self { client, table_name }
    }
    pub async fn create(&self, todo: &Todo) -> Result<Todo, String> {
        let mut item = HashMap::new();
        item.insert(
            "id".to_string(),
            AttributeValue { s: Some(todo.id.to_string()), ..Default::default() },
        );
        item.insert(
            "text".to_string(),
            AttributeValue {
                s: Some(todo.text.clone()),
                ..Default::default()
            },
        );
        item.insert(
            "completed".to_string(),
            AttributeValue {
                bool: Some(todo.completed),
                ..Default::default()
            },
        );

        let input = PutItemInput {
            table_name: self.table_name.clone(),
            item,
            ..Default::default()
        };


        self.client.put_item(input).await.map_err(|e| e.to_string())?;
        Ok(todo.clone())
    }

    pub async fn list(&self) -> Result<Vec<Todo>, String> {
        return self.client.scan(Default::default()).await.map_err(|e| e.to_string())?
            .items.ok_or_else(|| "No items found".to_string())?
            .iter().map(|item| {
            let id = item
                .get("id")
                .and_then(|v| v.s.as_ref())
                .ok_or_else(|| "ID not found or not a string".to_string())?;
            let id = Uuid::parse_str(id).map_err(|_| "Failed to parse UUID".to_string())?;
            let text = item
                .get("text")
                .and_then(|v| v.s.as_ref())
                .ok_or_else(|| "Text not found or not a string".to_string())?
                .clone();
            let completed = item
                .get("completed")
                .and_then(|v| v.bool.as_ref())
                .ok_or_else(|| "Completed not found or not a bool".to_string())?;
            Ok(Todo { id, text, completed: *completed })
        }).collect();
    }
    pub async fn read(&self, id: &Uuid) -> Result<Todo, String> {
        let mut key = HashMap::new();
        key.insert("id".to_string(), AttributeValue {
            s: Some(id.to_string()),
            ..Default::default()
        });

        let input = GetItemInput {
            table_name: self.table_name.clone(),
            key,
            ..Default::default()
        };

        let output = self.client.get_item(input).await.map_err(|e| e.to_string())?;

        match output.item {
            Some(item) => {
                let id = item
                    .get("id")
                    .and_then(|v| v.s.as_ref())
                    .ok_or_else(|| "ID not found or not a string".to_string())?;
                let id = Uuid::parse_str(id).map_err(|_| "Failed to parse UUID".to_string())?;
                let text = item
                    .get("text")
                    .and_then(|v| v.s.as_ref())
                    .ok_or_else(|| "Text not found or not a string".to_string())?
                    .clone();
                let completed = item
                    .get("completed")
                    .and_then(|v| v.bool.as_ref())
                    .ok_or_else(|| "Completed not found or not a boolean".to_string())?
                    .clone();

                Ok(Todo { id, text, completed })
            }
            None => Err("Item not found".into()),
        }
    }

    pub async fn update(&self, id: &Uuid, text: &str, completed: bool) -> Result<(), String> {
        let mut key = HashMap::new();
        key.insert("id".to_string(), AttributeValue {
            s: Some(id.to_string()),
            ..Default::default()
        });

        let mut update_values = HashMap::new();
        update_values.insert(":t".to_string(), AttributeValue {
            s: Some(text.to_string()),
            ..Default::default()
        });
        update_values.insert(":c".to_string(), AttributeValue {
            bool: Some(completed),
            ..Default::default()
        });

        let input = UpdateItemInput {
            table_name: self.table_name.clone(),
            key,
            update_expression: Some("SET text = :t, completed = :c".to_string()),
            expression_attribute_values: Some(update_values),
            ..Default::default()
        };
        self.client.update_item(input).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let mut key = HashMap::new();
        key.insert("id".to_string(), AttributeValue {
            s: Some(id.to_string()),
            ..Default::default()
        });

        let input = DeleteItemInput {
            table_name: self.table_name.clone(),
            key,
            ..Default::default()
        };

        self.client.delete_item(input).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}