use chrono::Utc;
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use super::client::BinanceHttpClient;

pub struct BinanceFuturesApi {
    client: BinanceHttpClient,
}

impl BinanceFuturesApi {
    pub fn new(base_url: &str, api_key: &str, api_secret: &str) -> Self {
        let client = BinanceHttpClient::new(base_url, api_key, api_secret);
        Self { client: client }
    }

    pub async fn account(&self, recv_window: Option<u8>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match recv_window {
            Some(recvwindow) => {
                params.insert(
                    String::from("recvWindow"),
                    recvwindow.to_string().parse().unwrap(),
                );
            }
            None => {}
        }
        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/account", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn position_risk(&self, symbol: Option<&str>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match symbol {
            Some(symbol_s) => {
                params.insert(
                    String::from("symbol"),
                    String::from(symbol_s).parse().unwrap(),
                );
            }
            None => {}
        }
        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/positionRisk", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn trade_hiostory(&self, symbol: &str) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("symbol"), Value::from(symbol));

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v1/userTrades", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                // print!("历史成交数据{}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn position(&self, symbol: &str) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("symbol"), Value::from(symbol));

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/positionRisk", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }
}
