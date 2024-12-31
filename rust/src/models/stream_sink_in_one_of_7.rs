/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.1
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use crate::models;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
        
                #[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
                pub struct StreamSinkInOneOf7 {
                        #[serde(rename = "config")]
                        pub config: Box<models::BigQueryConfig>,
                        #[serde(rename = "type")]
                        pub r#type: Type,
                    }

                    impl StreamSinkInOneOf7 {
                    pub fn new(config: models::BigQueryConfig, r#type: Type) -> StreamSinkInOneOf7 {
                StreamSinkInOneOf7 {
                    config: Box::new(config),
                    r#type,
                    }
                    }
                    }
                    /// 
                    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
                    pub enum Type {
                            #[serde(rename = "bigQuery")]
                        BigQuery,
                    }

                    impl Default for Type {
                    fn default() -> Type {
                        Self::BigQuery
                    }
                    }

