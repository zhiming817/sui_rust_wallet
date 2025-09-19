use std::fmt;

/// 支持的网络类型
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Network {
    Devnet,
    Testnet,
    Mainnet,
}

impl Network {
    /// 获取网络的 RPC URL
    pub fn url(&self) -> &'static str {
        match self {
            Network::Devnet => "https://fullnode.devnet.sui.io:443",
            Network::Testnet => "https://fullnode.testnet.sui.io:443",
            Network::Mainnet => "https://fullnode.mainnet.sui.io:443",
        }
    }

    /// 获取网络名称
    pub fn name(&self) -> &'static str {
        match self {
            Network::Devnet => "Devnet",
            Network::Testnet => "Testnet",
            Network::Mainnet => "Mainnet",
        }
    }

    /// 获取网络简称
    pub fn short_name(&self) -> &'static str {
        match self {
            Network::Devnet => "DEV",
            Network::Testnet => "TEST",
            Network::Mainnet => "MAIN",
        }
    }

    /// 获取网络描述
    pub fn description(&self) -> &'static str {
        match self {
            Network::Devnet => "Development network for testing",
            Network::Testnet => "Test network with reset cycles",
            Network::Mainnet => "Production network",
        }
    }

    /// 获取区块链浏览器 URL
    pub fn explorer_url(&self) -> &'static str {
        match self {
            Network::Devnet => "https://suiexplorer.com/?network=devnet",
            Network::Testnet => "https://suiexplorer.com/?network=testnet",
            Network::Mainnet => "https://suiexplorer.com",
        }
    }

    /// 获取地址的浏览器链接
    pub fn address_explorer_url(&self, address: &str) -> String {
        match self {
            Network::Devnet => format!("https://suiexplorer.com/address/{}?network=devnet", address),
            Network::Testnet => format!("https://suiexplorer.com/address/{}?network=testnet", address),
            Network::Mainnet => format!("https://suiexplorer.com/address/{}", address),
        }
    }

    /// 获取交易的浏览器链接
    pub fn transaction_explorer_url(&self, tx_hash: &str) -> String {
        match self {
            Network::Devnet => format!("https://suiexplorer.com/txblock/{}?network=devnet", tx_hash),
            Network::Testnet => format!("https://suiexplorer.com/txblock/{}?network=testnet", tx_hash),
            Network::Mainnet => format!("https://suiexplorer.com/txblock/{}", tx_hash),
        }
    }

    /// 获取所有可用网络
    pub fn all() -> Vec<Network> {
        vec![Network::Devnet, Network::Testnet, Network::Mainnet]
    }

    /// 检查是否为生产网络
    pub fn is_mainnet(&self) -> bool {
        matches!(self, Network::Mainnet)
    }

    /// 检查是否为测试网络
    pub fn is_testnet(&self) -> bool {
        matches!(self, Network::Testnet | Network::Devnet)
    }

    /// 获取网络配色
    pub fn color(&self) -> NetworkColor {
        match self {
            Network::Devnet => NetworkColor::Blue,
            Network::Testnet => NetworkColor::Yellow,
            Network::Mainnet => NetworkColor::Green,
        }
    }

    /// 从字符串解析网络
    pub fn from_str(s: &str) -> Option<Network> {
        match s.to_lowercase().as_str() {
            "devnet" | "dev" => Some(Network::Devnet),
            "testnet" | "test" => Some(Network::Testnet),
            "mainnet" | "main" => Some(Network::Mainnet),
            _ => None,
        }
    }

    /// 获取默认网络（通常是 Devnet 用于开发）
    pub fn default() -> Network {
        Network::Devnet
    }

    /// 获取建议的最小余额（用于交易费用）
    pub fn minimum_balance(&self) -> f64 {
        match self {
            Network::Devnet => 0.001,
            Network::Testnet => 0.001,
            Network::Mainnet => 0.01,
        }
    }

    /// 获取预估的交易费用
    pub fn estimated_tx_fee(&self) -> f64 {
        match self {
            Network::Devnet => 0.0001,
            Network::Testnet => 0.0001,
            Network::Mainnet => 0.001,
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::default()
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// 网络配色枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkColor {
    Blue,
    Yellow,
    Green,
}

impl NetworkColor {
    /// 获取对应的 egui 颜色
    #[cfg(feature = "egui")]
    pub fn to_egui_color(&self) -> eframe::egui::Color32 {
        match self {
            NetworkColor::Blue => eframe::egui::Color32::BLUE,
            NetworkColor::Yellow => eframe::egui::Color32::YELLOW,
            NetworkColor::Green => eframe::egui::Color32::GREEN,
        }
    }

    /// 获取 RGB 值
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            NetworkColor::Blue => (0, 100, 255),
            NetworkColor::Yellow => (255, 200, 0),
            NetworkColor::Green => (0, 150, 0),
        }
    }
}

/// 网络配置管理
pub struct NetworkConfig {
    pub current_network: Network,
    pub auto_switch: bool,
    pub preferred_network: Network,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            current_network: Network::default(),
            auto_switch: false,
            preferred_network: Network::default(),
        }
    }

    pub fn switch_to(&mut self, network: Network) {
        self.current_network = network;
    }

    pub fn set_preferred(&mut self, network: Network) {
        self.preferred_network = network;
    }

    pub fn enable_auto_switch(&mut self) {
        self.auto_switch = true;
    }

    pub fn disable_auto_switch(&mut self) {
        self.auto_switch = false;
    }

    pub fn should_auto_switch(&self) -> bool {
        self.auto_switch && self.current_network != self.preferred_network
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 网络工具函数
pub struct NetworkUtils;

impl NetworkUtils {
    /// 检查网络连接状态
    pub async fn check_network_health(network: Network) -> NetworkHealthStatus {
        // 这里可以实现实际的网络健康检查
        // 目前返回假设的状态
        NetworkHealthStatus::Healthy
    }

    /// 获取推荐的网络
    pub fn get_recommended_network() -> Network {
        // 对于开发环境，推荐使用 Devnet
        // 对于生产环境，可以根据需要调整
        #[cfg(debug_assertions)]
        return Network::Devnet;
        
        #[cfg(not(debug_assertions))]
        return Network::Testnet;
    }

    /// 比较网络性能
    pub fn compare_networks(net1: Network, net2: Network) -> NetworkComparison {
        if net1.is_mainnet() && !net2.is_mainnet() {
            NetworkComparison::FirstBetter
        } else if !net1.is_mainnet() && net2.is_mainnet() {
            NetworkComparison::SecondBetter
        } else {
            NetworkComparison::Equal
        }
    }
}

/// 网络健康状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// 网络比较结果
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkComparison {
    FirstBetter,
    SecondBetter,
    Equal,
}