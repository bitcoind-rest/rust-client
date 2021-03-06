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
pub struct Transaction {
  /// The transaction amount in BTC
  #[serde(rename = "amount")]
  amount: Option<f32>,
  /// The amount of the fee in BTC. This is negative and only available for the send category of transactions.
  #[serde(rename = "fee")]
  fee: Option<f32>,
  /// The number of confirmations
  #[serde(rename = "confirmations")]
  confirmations: Option<i64>,
  /// The block hash
  #[serde(rename = "blockhash")]
  blockhash: Option<String>,
  /// The index of the transaction in the block that includes it
  #[serde(rename = "blockindex")]
  blockindex: Option<i64>,
  /// The time in seconds since epoch (1 Jan 1970 GMT)
  #[serde(rename = "blocktime")]
  blocktime: Option<i32>,
  /// The transaction id
  #[serde(rename = "txid")]
  txid: Option<String>,
  /// The transaction hash
  #[serde(rename = "txhash")]
  txhash: Option<String>,
  #[serde(rename = "version")]
  version: Option<i32>,
  #[serde(rename = "size")]
  size: Option<i32>,
  #[serde(rename = "vsize")]
  vsize: Option<i32>,
  #[serde(rename = "locktime")]
  locktime: Option<i32>,
  /// The transaction time in seconds since epoch (1 Jan 1970 GMT)
  #[serde(rename = "time")]
  time: Option<i32>,
  /// The time received in seconds since epoch (1 Jan 1970 GMT)
  #[serde(rename = "timereceived")]
  timereceived: Option<i32>,
  /// Whether this transaction could be replaced due to BIP125 (replace-by-fee); may be unknown for unconfirmed transactions not in the mempool
  #[serde(rename = "bip125-replaceable")]
  bip125_replaceable: Option<String>
}

impl Transaction {
  pub fn new() -> Transaction {
    Transaction {
      amount: None,
      fee: None,
      confirmations: None,
      blockhash: None,
      blockindex: None,
      blocktime: None,
      txid: None,
      txhash: None,
      version: None,
      size: None,
      vsize: None,
      locktime: None,
      time: None,
      timereceived: None,
      bip125_replaceable: None
    }
  }

  pub fn set_amount(&mut self, amount: f32) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: f32) -> Transaction {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&f32> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_fee(&mut self, fee: f32) {
    self.fee = Some(fee);
  }

  pub fn with_fee(mut self, fee: f32) -> Transaction {
    self.fee = Some(fee);
    self
  }

  pub fn fee(&self) -> Option<&f32> {
    self.fee.as_ref()
  }

  pub fn reset_fee(&mut self) {
    self.fee = None;
  }

  pub fn set_confirmations(&mut self, confirmations: i64) {
    self.confirmations = Some(confirmations);
  }

  pub fn with_confirmations(mut self, confirmations: i64) -> Transaction {
    self.confirmations = Some(confirmations);
    self
  }

  pub fn confirmations(&self) -> Option<&i64> {
    self.confirmations.as_ref()
  }

  pub fn reset_confirmations(&mut self) {
    self.confirmations = None;
  }

  pub fn set_blockhash(&mut self, blockhash: String) {
    self.blockhash = Some(blockhash);
  }

  pub fn with_blockhash(mut self, blockhash: String) -> Transaction {
    self.blockhash = Some(blockhash);
    self
  }

  pub fn blockhash(&self) -> Option<&String> {
    self.blockhash.as_ref()
  }

  pub fn reset_blockhash(&mut self) {
    self.blockhash = None;
  }

  pub fn set_blockindex(&mut self, blockindex: i64) {
    self.blockindex = Some(blockindex);
  }

  pub fn with_blockindex(mut self, blockindex: i64) -> Transaction {
    self.blockindex = Some(blockindex);
    self
  }

  pub fn blockindex(&self) -> Option<&i64> {
    self.blockindex.as_ref()
  }

  pub fn reset_blockindex(&mut self) {
    self.blockindex = None;
  }

  pub fn set_blocktime(&mut self, blocktime: i32) {
    self.blocktime = Some(blocktime);
  }

  pub fn with_blocktime(mut self, blocktime: i32) -> Transaction {
    self.blocktime = Some(blocktime);
    self
  }

  pub fn blocktime(&self) -> Option<&i32> {
    self.blocktime.as_ref()
  }

  pub fn reset_blocktime(&mut self) {
    self.blocktime = None;
  }

  pub fn set_txid(&mut self, txid: String) {
    self.txid = Some(txid);
  }

  pub fn with_txid(mut self, txid: String) -> Transaction {
    self.txid = Some(txid);
    self
  }

  pub fn txid(&self) -> Option<&String> {
    self.txid.as_ref()
  }

  pub fn reset_txid(&mut self) {
    self.txid = None;
  }

  pub fn set_txhash(&mut self, txhash: String) {
    self.txhash = Some(txhash);
  }

  pub fn with_txhash(mut self, txhash: String) -> Transaction {
    self.txhash = Some(txhash);
    self
  }

  pub fn txhash(&self) -> Option<&String> {
    self.txhash.as_ref()
  }

  pub fn reset_txhash(&mut self) {
    self.txhash = None;
  }

  pub fn set_version(&mut self, version: i32) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: i32) -> Transaction {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&i32> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

  pub fn set_size(&mut self, size: i32) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: i32) -> Transaction {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&i32> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_vsize(&mut self, vsize: i32) {
    self.vsize = Some(vsize);
  }

  pub fn with_vsize(mut self, vsize: i32) -> Transaction {
    self.vsize = Some(vsize);
    self
  }

  pub fn vsize(&self) -> Option<&i32> {
    self.vsize.as_ref()
  }

  pub fn reset_vsize(&mut self) {
    self.vsize = None;
  }

  pub fn set_locktime(&mut self, locktime: i32) {
    self.locktime = Some(locktime);
  }

  pub fn with_locktime(mut self, locktime: i32) -> Transaction {
    self.locktime = Some(locktime);
    self
  }

  pub fn locktime(&self) -> Option<&i32> {
    self.locktime.as_ref()
  }

  pub fn reset_locktime(&mut self) {
    self.locktime = None;
  }

  pub fn set_time(&mut self, time: i32) {
    self.time = Some(time);
  }

  pub fn with_time(mut self, time: i32) -> Transaction {
    self.time = Some(time);
    self
  }

  pub fn time(&self) -> Option<&i32> {
    self.time.as_ref()
  }

  pub fn reset_time(&mut self) {
    self.time = None;
  }

  pub fn set_timereceived(&mut self, timereceived: i32) {
    self.timereceived = Some(timereceived);
  }

  pub fn with_timereceived(mut self, timereceived: i32) -> Transaction {
    self.timereceived = Some(timereceived);
    self
  }

  pub fn timereceived(&self) -> Option<&i32> {
    self.timereceived.as_ref()
  }

  pub fn reset_timereceived(&mut self) {
    self.timereceived = None;
  }

  pub fn set_bip125_replaceable(&mut self, bip125_replaceable: String) {
    self.bip125_replaceable = Some(bip125_replaceable);
  }

  pub fn with_bip125_replaceable(mut self, bip125_replaceable: String) -> Transaction {
    self.bip125_replaceable = Some(bip125_replaceable);
    self
  }

  pub fn bip125_replaceable(&self) -> Option<&String> {
    self.bip125_replaceable.as_ref()
  }

  pub fn reset_bip125_replaceable(&mut self) {
    self.bip125_replaceable = None;
  }

}



