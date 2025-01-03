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
                pub struct AppUsageStatsIn {
                        /// Specific app IDs or UIDs to aggregate stats for.  Note that if none of the given IDs or UIDs are resolved, a 422 response will be given.
                        #[serde(rename = "appIds", skip_serializing_if = "Option::is_none")]
                        pub app_ids: Option<Vec<String>>,
                        #[serde(rename = "since")]
                        pub since: String,
                        #[serde(rename = "until")]
                        pub until: String,
                    }

                    impl AppUsageStatsIn {
                    pub fn new(since: String, until: String) -> AppUsageStatsIn {
                AppUsageStatsIn {
                    app_ids: None,
                    since,
                    until,
                    }
                    }
                    }

