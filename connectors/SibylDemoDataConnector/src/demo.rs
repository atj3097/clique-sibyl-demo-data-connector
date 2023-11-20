// src/etherscan_connector.rs
use std::prelude::v1::*;
use sibyl_base_data_connector::utils::simple_tls_client;
use sibyl_base_data_connector::base::DataConnector;
use sibyl_base_data_connector::serde_json::Value;
use sibyl_base_data_connector::errors::NetworkError;
use std::env;
use crate::env;

pub struct EtherscanConnector {}

impl DataConnector for EtherscanConnector {
  fn query(&self, query_type: &Value, query_param: &Value) -> Result<Value, NetworkError> {
    let query_type_str = match query_type.as_str() {
      Some(r) => r,
      _ => {
        let err = format!("query_type to str failed");
        println!("{:?}", err);
        return Err(NetworkError::String(err));
      }
    };
    match query_type_str {
      "get_balance" => {
        let address = query_param["address"].as_str().unwrap_or("");
        let api_key = env::var("ETHERSCAN_API_KEY").expect("Expected ETHERSCAN_API_KEY in the environment");
        let url = format!("{}?module=account&action=balance&address={}&tag=latest&apikey={}",
                          env::ETHERSCAN_API_SUFFIX, address, api_key);

        let req = format!(
          "GET {} HTTP/1.1\r\n\
          Host: {}\r\n\
          User-Agent: curl/7.79.1\r\n\
          Accept: */*\r\n\r\n",
          url, env::ETHERSCAN_API_HOST
        );
        simple_tls_client(env::ETHERSCAN_API_HOST, &req, 443)
      },
      _ => {
        Err(NetworkError::String(format!("Unexpected query_type: {:?}", query_type)))
      }
    }
  }
}
