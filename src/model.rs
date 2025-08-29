use std::sync::mpsc::{self, Receiver, Sender};
use sui_sdk::types::{base_types::SuiAddress, crypto::SuiKeyPair};
use tokio::runtime::Runtime;

/// 支持的网络
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Network {
    Devnet,
    Testnet,
    Mainnet,
}

impl Network {
    pub fn url(&self) -> &'static str {
        match self {
            Network::Devnet => "https://fullnode.devnet.sui.io:443",
            Network::Testnet => "https://fullnode.testnet.sui.io:443",
            Network::Mainnet => "https://fullnode.mainnet.sui.io:443",
        }
    }
}

/// 钱包状态
pub enum WalletState {
    // 未导入钱包，存储用户输入的私钥字符串
    NoWallet { private_key_input: String },
    // 已加载钱包
    Loaded {
        address: SuiAddress,
        keypair: SuiKeyPair,
    },
}

/// 应用的所有状态
pub struct Model {
    pub wallet: WalletState,
    pub network: Network,
    pub result_text: String,
    pub is_loading: bool,
    // 转账信息
    pub recipient_address: String,
    pub transfer_amount: String,
    // 异步处理
    pub rt: Runtime,
    pub sender: Sender<Result<String, String>>,
    pub receiver: Receiver<Result<String, String>>,
}

impl Default for Model {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            wallet: WalletState::NoWallet {
                private_key_input: "".to_string(),
            },
            network: Network::Devnet,
            result_text: "Please import a private key to begin.".to_string(),
            is_loading: false,
            recipient_address: "".to_string(),
            transfer_amount: "".to_string(),
            rt: Runtime::new().expect("Failed to create Tokio runtime"),
            sender,
            receiver,
        }
    }
}