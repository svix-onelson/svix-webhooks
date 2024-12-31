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
                pub struct EndpointTransformationOut {
                        #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
                        pub code: Option<String>,
                        #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
                        pub enabled: Option<bool>,
                    }

                    impl EndpointTransformationOut {
                    pub fn new() -> EndpointTransformationOut {
                EndpointTransformationOut {
                    code: None,
                    enabled: None,
                    }
                    }
                    }

