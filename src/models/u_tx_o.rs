/* 
 * Bitcoind
 *
 * The REST API can be enabled with the `-rest` option. The interface runs on the same port as the JSON-RPC interface, by default port `8332` for **mainnet**, port `18332` for **testnet**, and port `18443` for **regtest**.
 *
 * OpenAPI spec version: 0.16
 * Contact: johan@lepetitbloc.net
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UTxO {
  #[serde(rename = "txvers")]
  txvers: Option<i32>,
  #[serde(rename = "height")]
  height: Option<i32>,
  #[serde(rename = "value")]
  value: Option<f32>,
  #[serde(rename = "scriptPubKey")]
  script_pub_key: Option<::models::ScriptPubKey>
}

impl UTxO {
  pub fn new() -> UTxO {
    UTxO {
      txvers: None,
      height: None,
      value: None,
      script_pub_key: None
    }
  }

  pub fn set_txvers(&mut self, txvers: i32) {
    self.txvers = Some(txvers);
  }

  pub fn with_txvers(mut self, txvers: i32) -> UTxO {
    self.txvers = Some(txvers);
    self
  }

  pub fn txvers(&self) -> Option<&i32> {
    self.txvers.as_ref()
  }

  pub fn reset_txvers(&mut self) {
    self.txvers = None;
  }

  pub fn set_height(&mut self, height: i32) {
    self.height = Some(height);
  }

  pub fn with_height(mut self, height: i32) -> UTxO {
    self.height = Some(height);
    self
  }

  pub fn height(&self) -> Option<&i32> {
    self.height.as_ref()
  }

  pub fn reset_height(&mut self) {
    self.height = None;
  }

  pub fn set_value(&mut self, value: f32) {
    self.value = Some(value);
  }

  pub fn with_value(mut self, value: f32) -> UTxO {
    self.value = Some(value);
    self
  }

  pub fn value(&self) -> Option<&f32> {
    self.value.as_ref()
  }

  pub fn reset_value(&mut self) {
    self.value = None;
  }

  pub fn set_script_pub_key(&mut self, script_pub_key: ::models::ScriptPubKey) {
    self.script_pub_key = Some(script_pub_key);
  }

  pub fn with_script_pub_key(mut self, script_pub_key: ::models::ScriptPubKey) -> UTxO {
    self.script_pub_key = Some(script_pub_key);
    self
  }

  pub fn script_pub_key(&self) -> Option<&::models::ScriptPubKey> {
    self.script_pub_key.as_ref()
  }

  pub fn reset_script_pub_key(&mut self) {
    self.script_pub_key = None;
  }

}



