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
                pub struct RotatedUrlOut {
                        #[serde(rename = "url")]
                        pub url: String,
                    }

                    impl RotatedUrlOut {
                    pub fn new(url: String) -> RotatedUrlOut {
                RotatedUrlOut {
                    url,
                    }
                    }
                    }

