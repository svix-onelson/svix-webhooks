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
        
            /// SnowflakeConfig : Configuration parameters for defining a Snowflake sink.
                #[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
                pub struct SnowflakeConfig {
                        /// Snowflake account identifier, which includes both the organization and account IDs separated by a hyphen.
                        #[serde(rename = "accountIdentifier")]
                        pub account_identifier: String,
                        /// Database name.  Only required if not using transformations.
                        #[serde(rename = "dbName", skip_serializing_if = "Option::is_none")]
                        pub db_name: Option<String>,
                        /// PEM-encoded private key used for signing token-based requests to the Snowflake API.  Beginning/end delimiters are not required.
                        #[serde(rename = "privateKey")]
                        pub private_key: String,
                        /// Schema name.  Only required if not using transformations.
                        #[serde(rename = "schemaName", skip_serializing_if = "Option::is_none")]
                        pub schema_name: Option<String>,
                        /// Table name.  Only required if not using transformations.
                        #[serde(rename = "tableName", skip_serializing_if = "Option::is_none")]
                        pub table_name: Option<String>,
                        /// The Snowflake user id.
                        #[serde(rename = "userId")]
                        pub user_id: String,
                    }

                    impl SnowflakeConfig {
                        /// Configuration parameters for defining a Snowflake sink.
                    pub fn new(account_identifier: String, private_key: String, user_id: String) -> SnowflakeConfig {
                SnowflakeConfig {
                    account_identifier,
                    db_name: None,
                    private_key,
                    schema_name: None,
                    table_name: None,
                    user_id,
                    }
                    }
                    }

