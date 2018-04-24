use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  block_api: Box<::apis::BlockApi>,
  chain_api: Box<::apis::ChainApi>,
  memory_pool_api: Box<::apis::MemoryPoolApi>,
  transaction_api: Box<::apis::TransactionApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      block_api: Box::new(::apis::BlockApiClient::new(rc.clone())),
      chain_api: Box::new(::apis::ChainApiClient::new(rc.clone())),
      memory_pool_api: Box::new(::apis::MemoryPoolApiClient::new(rc.clone())),
      transaction_api: Box::new(::apis::TransactionApiClient::new(rc.clone())),
    }
  }

  pub fn block_api(&self) -> &::apis::BlockApi{
    self.block_api.as_ref()
  }

  pub fn chain_api(&self) -> &::apis::ChainApi{
    self.chain_api.as_ref()
  }

  pub fn memory_pool_api(&self) -> &::apis::MemoryPoolApi{
    self.memory_pool_api.as_ref()
  }

  pub fn transaction_api(&self) -> &::apis::TransactionApi{
    self.transaction_api.as_ref()
  }


}
