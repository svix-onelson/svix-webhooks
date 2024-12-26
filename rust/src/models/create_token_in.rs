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
                pub struct CreateTokenIn {
                        /// How long the token will be valid for, in seconds.
                        #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
                        pub expiry: Option<i32>,
                        /// The name of the token.
                        #[serde(rename = "name")]
                        pub name: String,
                    }

                    impl CreateTokenIn {
                    pub fn new(name: String) -> CreateTokenIn {
                CreateTokenIn {
                    expiry: None,
                    name,
                    }
                    }
                    }
