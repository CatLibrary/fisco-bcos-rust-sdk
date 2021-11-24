use ethabi::Token;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde_json::{Value as JSONValue, json};

use crate::abi::ABI;
use crate::config::Config;
use crate::transaction::get_sign_transaction_data;
use crate::account::{Account, create_account_from_pem};
use crate::web3::{fetcher_trait::FetcherTrait, service_error::ServiceError};
use crate::helpers::{parse_json_string, parse_json_string_array, convert_hex_str_to_u32};

fn generate_request_params(method: &str, params: &JSONValue) -> JSONValue {
    json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": method.to_owned(),
        "params": params.clone(),
    })
}

#[derive(Debug)]
pub struct CallResponse {
    pub current_block_number: String,
    pub status: String,
    pub output: Option<Vec<Token>>,
}

pub struct Service {
    pub config: Config,
    account: Account,
    fetcher: Box<dyn FetcherTrait + Send + Sync>,
}

impl Service {
    fn get_abi(&self, contract_name: &str) -> Result<ABI, ServiceError> {
        Ok(ABI::new(&self.config.contract, contract_name, self.config.sm_crypto)?)
    }

    pub fn new(config: &Config, fetcher: Box<dyn FetcherTrait + Send + Sync>) -> Result<Service, ServiceError> {
        Ok(
            Service {
                fetcher,
                config: config.clone(),
                account: create_account_from_pem(&config.account, config.sm_crypto)?,
            }
        )
    }

    pub async fn get_client_version(&self)  -> Result<JSONValue, ServiceError> {
        let params = generate_request_params("getClientVersion", &json!([self.config.group_id]));
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_number(&self) -> Result<String, ServiceError> {
        let params = generate_request_params("getBlockNumber", &json!([self.config.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string(&response))
    }

    pub async fn get_pbft_view(&self) -> Result<String, ServiceError> {
        let params = generate_request_params("getPbftView", &json!([self.config.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string(&response))
    }

    pub async fn get_sealer_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getSealerList", &json!([self.config.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string_array(&response))
    }

    pub async fn get_observer_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getObserverList", &json!([self.config.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string_array(&response))
    }

    pub async fn get_consensus_status(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params("getConsensusStatus", &json!([self.config.group_id]));
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_sync_status(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params("getSyncStatus", &json!([self.config.group_id]));
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_peers(&self) -> Result<Vec<JSONValue>, ServiceError> {
        let params = generate_request_params("getPeers", &json!([self.config.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(response.as_array().unwrap().clone())
    }

    pub async fn get_group_peers(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getGroupPeers", &json!([self.config.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string_array(&response))
    }

    pub async fn get_node_id_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getNodeIDList", &json!([self.config.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string_array(&response))
    }

    pub async fn get_group_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getGroupList", &json!(null));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string_array(&response))
    }

    pub async fn get_block_by_hash(&self, block_hash: &str, include_transactions: bool) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getBlockByHash",
            &json!([self.config.group_id, block_hash, include_transactions]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_by_number(&self, block_number: &str, include_transactions: bool)-> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getBlockByNumber",
            &json!([self.config.group_id, block_number, include_transactions]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_header_by_hash(&self, block_hash: &str, include_transactions: bool) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getBlockHeaderByHash",
            &json!([self.config.group_id, block_hash, include_transactions]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_header_by_number(&self, block_number: &str, include_transactions: bool) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getBlockHeaderByNumber",
            &json!([self.config.group_id, block_number, include_transactions]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_hash_by_number(&self, block_number: &str) -> Result<String, ServiceError> {
        let params = generate_request_params(
            "getBlockHashByNumber",
            &json!([self.config.group_id, block_number]),
        );
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string(&response))
    }

    pub async fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getTransactionByHash",
            &json!([self.config.group_id, transaction_hash]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_transaction_by_block_hash_and_index(&self, block_hash: &str, transaction_index: &str) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getTransactionByBlockHashAndIndex",
            &json!([self.config.group_id, block_hash, transaction_index]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_transaction_by_block_number_and_index(&self, block_number: &str, transaction_index: &str) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getTransactionByBlockNumberAndIndex",
            &json!([self.config.group_id, block_number, transaction_index]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_transaction_receipt(&self, transaction_hash: &str) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getTransactionReceipt",
            &json!([self.config.group_id, transaction_hash]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_pending_transactions(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getPendingTransactions",
            &json!([self.config.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_pending_tx_size(&self) -> Result<String, ServiceError> {
        let params = generate_request_params(
            "getPendingTxSize",
            &json!([self.config.group_id]),
        );
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string(&response))
    }

    pub async fn get_code(&self, address: &str) -> Result<String, ServiceError> {
        let params = generate_request_params(
            "getCode",
            &json!([self.config.group_id, address]),
        );
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string(&response))
    }

    pub async fn get_total_transaction_count(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getTotalTransactionCount",
            &json!([self.config.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_system_config_by_key(&self, key: &str) -> Result<String, ServiceError> {
        let params = generate_request_params(
            "getSystemConfigByKey",
            &json!([self.config.group_id, key]),
        );
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_json_string(&response))
    }

    pub async fn call(
        &self,
        contract_name: &str,
        to_address: &str,
        function_name: &str,
        tokens: &Vec<Token>,
    ) -> Result<CallResponse, ServiceError> {
        let abi = self.get_abi(contract_name)?;
        let transaction_data = abi.encode_function_input(function_name, tokens)?;
        let params = json!({
            "from": format!("0x{}", hex::encode(&self.account.address)),
            "to": to_address.to_owned(),
            "value": "0x0",
            "data": format!("0x{}", hex::encode(&transaction_data)),
        });
        let response = self.fetcher.fetch(
            &generate_request_params("call", &json!([self.config.group_id, params]))
        ).await?;
        Ok(CallResponse {
            status: parse_json_string(&response["status"]),
            current_block_number: parse_json_string(&response["currentBlockNumber"]),
            output: abi.decode_output(function_name, &parse_json_string(&response["output"]))?,
        })
    }

    async fn send_transaction(
        &self,
        method: &str,
        contract_name: &str,
        to_address: &str,
        function_name: &str,
        tokens: &Vec<Token>,
    ) -> Result<String, ServiceError> {
        let block_number = convert_hex_str_to_u32(&self.get_block_number().await?);
        let abi = self.get_abi(contract_name)?;
        let data = abi.encode_function_input(function_name, tokens)?;
        let transaction_data = get_sign_transaction_data(
            &self.account,
            self.config.group_id,
            self.config.chain_id,
            block_number + 500,
            to_address,
            &data,
            self.config.sm_crypto,
        )?;
        let params = generate_request_params(
            method,
            &json!([self.config.group_id, format!("0x{}", hex::encode(&transaction_data))]),
        );
        Ok(parse_json_string(&self.fetcher.fetch(&params).await?))
    }

    pub async fn send_raw_transaction(
        &self,
        contract_name: &str,
        to_address: &str,
        function_name: &str,
        tokens: &Vec<Token>,
    ) -> Result<String, ServiceError> {
        Ok(self.send_transaction("sendRawTransaction", contract_name, to_address, function_name, tokens).await?)
    }

    pub async fn send_raw_transaction_and_get_proof(
        &self,
        contract_name: &str,
        to_address: &str,
        function_name: &str,
        tokens: &Vec<Token>,
    ) -> Result<String, ServiceError> {
        Ok(self.send_transaction("sendRawTransactionAndGetProof", contract_name, to_address, function_name, tokens).await?)
    }

    pub async fn deploy(&self, contract_name: &str, tokens: &Vec<Token>) -> Result<JSONValue, ServiceError> {
        let block_number = convert_hex_str_to_u32(&self.get_block_number().await?);
        let abi = self.get_abi(contract_name)?;
        let data = abi.encode_constructor_input(tokens)?;
        let transaction_data = get_sign_transaction_data(
            &self.account,
            self.config.group_id,
            self.config.chain_id,
            block_number + 500,
            "",
            &data,
            self.config.sm_crypto,
        )?;
        let params = generate_request_params(
            "sendRawTransactionAndGetProof",
            &json!([self.config.group_id, format!("0x{}", hex::encode(&transaction_data))]),
        );
        let transaction_hash = parse_json_string(&self.fetcher.fetch(&params).await?);
        let start = Instant::now();
        let timeout_milliseconds = (1000 * self.config.timeout_seconds) as u128;
        while Instant::now().duration_since(start).as_millis() < timeout_milliseconds {
            let transaction_receipt: JSONValue = self.get_transaction_receipt(&transaction_hash).await?;
            if transaction_receipt.is_null() {
                tokio::time::sleep(Duration::from_millis(200)).await;
                continue;
            }
            let transaction_receipt = self.get_transaction_receipt(&transaction_hash).await?;
            return Ok(json!({
                "status": transaction_receipt["status"],
                "transactionHash": transaction_receipt["transactionHash"],
                "contractAddress": transaction_receipt["contractAddress"]
            }));
        }
        Err(ServiceError::FiscoBcosError {
            code: -1,
            message: format!(
                "Contract deployed, but the action for fetching transaction receipt is timeout. Transaction hash is {:?}",
                transaction_hash
            ),
        })
    }

    ///
    /// link_libraries 中的键为要链接的 library 的名称，其值为要链接的 library 的地址
    ///
    pub async fn compile(
        &self,
        contract_name: &str,
        link_libraries: &Option<HashMap<String, String>>,
    ) -> Result<(), ServiceError> {
        let mut abi = self.get_abi(contract_name)?;
        Ok(abi.compile(link_libraries)?)
    }

    pub async fn get_transaction_by_hash_with_proof(&self, transaction_hash: &str) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getTransactionByHashWithProof",
            &json!([self.config.group_id, transaction_hash]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_transaction_receipt_by_hash_with_proof(&self, transaction_hash: &str) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getTransactionReceiptByHashWithProof",
            &json!([self.config.group_id, transaction_hash]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    ///
    /// params 中的属性如下所示：
    ///
    /// |  属性名 | 类型 | 备注 |
    /// |  ----  | ---- | ---- |
    /// | timestamp | u32 | 创世块时间戳 |
    /// | sealers   | Vec\<String\> | 共识节点列表，要求所有所列共识节点间存在有效的 P2P 连接 |
    /// | enable_free_storage | bool | 可选，是否启用 free storage 模式，启用后节点将减少 STORAGE 相关指令的 gas 耗费 |
    ///
    pub async fn generate_group(&self, params: &JSONValue) -> Result<JSONValue, ServiceError> {
        let request_params = json!([self.config.group_id, params.clone()]);
        Ok(self.fetcher.fetch(&generate_request_params("generateGroup", &request_params)).await?)
    }

    pub async fn start_group(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "startGroup",
            &json!([self.config.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn stop_group(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "stopGroup",
            &json!([self.config.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn remove_group(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "removeGroup",
            &json!([self.config.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn recover_group(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "recoverGroup",
            &json!([self.config.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn query_group_status(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "queryGroupStatus",
            &json!([self.config.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_node_info(&self) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params("getNodeInfo", &json!(null));
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_batch_receipts_by_block_number_and_range(
        &self,
        block_number: &str,
        from: u32,
        count: i32,
        compress_flag: bool,
    ) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getBatchReceiptsByBlockNumberAndRange",
            &json!([self.config.group_id, block_number, from, count, compress_flag]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_batch_receipts_by_block_hash_and_range(
        &self,
        block_hash: &str,
        from: u32,
        count: i32,
        compress_flag: bool,
    ) -> Result<JSONValue, ServiceError> {
        let params = generate_request_params(
            "getBatchReceiptsByBlockHashAndRange",
            &json!([self.config.group_id, block_hash, from, count, compress_flag]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }
}