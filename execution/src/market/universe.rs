use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarketType {
    Stock,
    ETF,
    Crypto,
    Future,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMetadata {
    pub symbol: String,
    pub name: String,
    pub market_type: MarketType,
    pub sector: Option<String>,
    pub exchange: String,
}

/// Symbol universe containing all tradeable assets
pub struct SymbolUniverse;

impl SymbolUniverse {
    /// Get all symbols in the universe
    pub fn all() -> Vec<SymbolMetadata> {
        let mut symbols = Vec::new();
        symbols.extend(Self::sp500());
        symbols.extend(Self::nasdaq100());
        symbols.extend(Self::dow30());
        symbols.extend(Self::russell_top200());
        symbols.extend(Self::leveraged_etfs());
        symbols.extend(Self::top_etfs());
        symbols.extend(Self::crypto());
        symbols
    }

    /// S&P 500 components (subset of most active)
    pub fn sp500() -> Vec<SymbolMetadata> {
        let symbols = vec![
            ("AAPL", "Apple Inc", "Technology"),
            ("MSFT", "Microsoft Corp", "Technology"),
            ("GOOGL", "Alphabet Inc Class A", "Technology"),
            ("AMZN", "Amazon.com Inc", "Consumer Cyclical"),
            ("NVDA", "NVIDIA Corp", "Technology"),
            ("META", "Meta Platforms Inc", "Technology"),
            ("TSLA", "Tesla Inc", "Consumer Cyclical"),
            ("BRK.B", "Berkshire Hathaway Inc Class B", "Financial"),
            ("V", "Visa Inc Class A", "Financial"),
            ("JPM", "JPMorgan Chase & Co", "Financial"),
            ("WMT", "Walmart Inc", "Consumer Defensive"),
            ("MA", "Mastercard Inc Class A", "Financial"),
            ("PG", "Procter & Gamble Co", "Consumer Defensive"),
            ("HD", "Home Depot Inc", "Consumer Cyclical"),
            ("CVX", "Chevron Corp", "Energy"),
            ("MRK", "Merck & Co Inc", "Healthcare"),
            ("KO", "Coca-Cola Co", "Consumer Defensive"),
            ("PEP", "PepsiCo Inc", "Consumer Defensive"),
            ("ABBV", "AbbVie Inc", "Healthcare"),
            ("AVGO", "Broadcom Inc", "Technology"),
            ("COST", "Costco Wholesale Corp", "Consumer Defensive"),
            ("LLY", "Eli Lilly and Co", "Healthcare"),
            ("MCD", "McDonald's Corp", "Consumer Cyclical"),
            ("NKE", "Nike Inc Class B", "Consumer Cyclical"),
            ("DIS", "Walt Disney Co", "Communication Services"),
            ("ABT", "Abbott Laboratories", "Healthcare"),
            ("TMO", "Thermo Fisher Scientific Inc", "Healthcare"),
            ("VZ", "Verizon Communications Inc", "Communication Services"),
            ("ACN", "Accenture PLC Class A", "Technology"),
            ("ADBE", "Adobe Inc", "Technology"),
            ("CSCO", "Cisco Systems Inc", "Technology"),
            ("ORCL", "Oracle Corp", "Technology"),
            ("CRM", "Salesforce Inc", "Technology"),
            ("INTC", "Intel Corp", "Technology"),
            ("AMD", "Advanced Micro Devices Inc", "Technology"),
            ("NFLX", "Netflix Inc", "Communication Services"),
            ("CMCSA", "Comcast Corp Class A", "Communication Services"),
            ("T", "AT&T Inc", "Communication Services"),
            ("UNH", "UnitedHealth Group Inc", "Healthcare"),
            ("BAC", "Bank of America Corp", "Financial"),
        ];

        symbols
            .into_iter()
            .map(|(symbol, name, sector)| SymbolMetadata {
                symbol: symbol.to_string(),
                name: name.to_string(),
                market_type: MarketType::Stock,
                sector: Some(sector.to_string()),
                exchange: "NASDAQ/NYSE".to_string(),
            })
            .collect()
    }

    /// NASDAQ 100 top components
    pub fn nasdaq100() -> Vec<SymbolMetadata> {
        let symbols = vec![
            ("TSLA", "Tesla Inc", "Consumer Cyclical"),
            ("GOOG", "Alphabet Inc Class C", "Technology"),
            ("AMGN", "Amgen Inc", "Healthcare"),
            ("PYPL", "PayPal Holdings Inc", "Financial"),
            ("QCOM", "QUALCOMM Inc", "Technology"),
            ("TXN", "Texas Instruments Inc", "Technology"),
            ("AMAT", "Applied Materials Inc", "Technology"),
            ("BKNG", "Booking Holdings Inc", "Consumer Cyclical"),
            ("MDLZ", "Mondelez International Inc", "Consumer Defensive"),
        ];

        symbols
            .into_iter()
            .map(|(symbol, name, sector)| SymbolMetadata {
                symbol: symbol.to_string(),
                name: name.to_string(),
                market_type: MarketType::Stock,
                sector: Some(sector.to_string()),
                exchange: "NASDAQ".to_string(),
            })
            .collect()
    }

    /// Dow Jones 30 components
    pub fn dow30() -> Vec<SymbolMetadata> {
        let symbols = vec![
            ("UNH", "UnitedHealth Group", "Healthcare"),
            ("GS", "Goldman Sachs", "Financial"),
            ("CAT", "Caterpillar", "Industrials"),
            ("BA", "Boeing", "Industrials"),
            ("AMZN", "Amazon.com", "Consumer Cyclical"),
            ("WMT", "Walmart", "Consumer Defensive"),
            ("CRM", "Salesforce", "Technology"),
            ("IBM", "IBM", "Technology"),
            ("DIS", "Walt Disney", "Communication Services"),
            ("NKE", "Nike", "Consumer Cyclical"),
        ];

        symbols
            .into_iter()
            .map(|(symbol, name, sector)| SymbolMetadata {
                symbol: symbol.to_string(),
                name: name.to_string(),
                market_type: MarketType::Stock,
                sector: Some(sector.to_string()),
                exchange: "NYSE".to_string(),
            })
            .collect()
    }

    /// Russell 2000 - top 200 most active small caps
    pub fn russell_top200() -> Vec<SymbolMetadata> {
        let symbols = vec![
            ("IWM", "iShares Russell 2000 ETF", "Small Cap"),
            ("GME", "GameStop Corp", "Consumer Cyclical"),
            ("AMC", "AMC Entertainment", "Consumer Cyclical"),
            ("PLUG", "Plug Power Inc", "Energy"),
            ("LCID", "Lucid Group Inc", "Consumer Cyclical"),
            ("RIVN", "Rivian Automotive Inc", "Consumer Cyclical"),
        ];

        symbols
            .into_iter()
            .map(|(symbol, name, sector)| SymbolMetadata {
                symbol: symbol.to_string(),
                name: name.to_string(),
                market_type: MarketType::Stock,
                sector: Some(sector.to_string()),
                exchange: "NYSE/NASDAQ".to_string(),
            })
            .collect()
    }

    /// Leveraged 3x ETFs
    pub fn leveraged_etfs() -> Vec<SymbolMetadata> {
        let symbols = vec![
            ("TQQQ", "ProShares UltraPro QQQ"),
            ("SQQQ", "ProShares UltraPro Short QQQ"),
            ("SPXL", "Direxion Daily S&P 500 Bull 3X"),
            ("SPXS", "Direxion Daily S&P 500 Bear 3X"),
            ("SOXL", "Direxion Daily Semiconductor Bull 3X"),
            ("SOXS", "Direxion Daily Semiconductor Bear 3X"),
            ("LABU", "Direxion Daily S&P Biotech Bull 3X"),
            ("LABD", "Direxion Daily S&P Biotech Bear 3X"),
            ("FNGU", "MicroSectors FANG+ Index 3X Leveraged"),
            ("FNGD", "MicroSectors FANG+ Index -3X Inverse"),
            ("TNA", "Direxion Daily Small Cap Bull 3X"),
            ("TZA", "Direxion Daily Small Cap Bear 3X"),
            ("UPRO", "ProShares UltraPro S&P500"),
            ("SDOW", "ProShares UltraPro Short Dow30"),
            ("UDOW", "ProShares UltraPro Dow30"),
            ("TECL", "Direxion Daily Technology Bull 3X"),
            ("TECS", "Direxion Daily Technology Bear 3X"),
            ("FAS", "Direxion Daily Financial Bull 3X"),
            ("FAZ", "Direxion Daily Financial Bear 3X"),
            ("ERX", "Direxion Daily Energy Bull 2X"),
            ("ERY", "Direxion Daily Energy Bear 2X"),
            ("NUGT", "Direxion Daily Gold Miners Bull 2X"),
            ("DUST", "Direxion Daily Gold Miners Bear 2X"),
        ];

        symbols
            .into_iter()
            .map(|( symbol, name)| SymbolMetadata {
                symbol: symbol.to_string(),
                name: name.to_string(),
                market_type: MarketType::ETF,
                sector: Some("Leveraged".to_string()),
                exchange: "NYSE/NASDAQ".to_string(),
            })
            .collect()
    }

    /// Top ETFs
    pub fn top_etfs() -> Vec<SymbolMetadata> {
        let symbols = vec![
            ("SPY", "SPDR S&P 500 ETF Trust"),
            ("QQQ", "Invesco QQQ Trust"),
            ("IWM", "iShares Russell 2000 ETF"),
            ("DIA", "SPDR Dow Jones Industrial Average ETF"),
            ("XLF", "Financial Select Sector SPDR Fund"),
            ("XLE", "Energy Select Sector SPDR Fund"),
            ("XLK", "Technology Select Sector SPDR Fund"),
            ("XLB", "Materials Select Sector SPDR Fund"),
            ("XLI", "Industrial Select Sector SPDR Fund"),
            ("XLU", "Utilities Select Sector SPDR Fund"),
            ("XLP", "Consumer Staples Select Sector SPDR Fund"),
            ("XLV", "Health Care Select Sector SPDR Fund"),
            ("XLY", "Consumer Discretionary Select Sector SPDR Fund"),
            ("XLRE", "Real Estate Select Sector SPDR Fund"),
            ("XLC", "Communication Services Select Sector SPDR Fund"),
            ("ARKK", "ARK Innovation ETF"),
            ("ARKG", "ARK Genomic Revolution ETF"),
            ("ARKW", "ARK Next Generation Internet ETF"),
            ("GLD", "SPDR Gold Trust"),
            ("SLV", "iShares Silver Trust"),
            ("TLT", "iShares 20+ Year Treasury Bond ETF"),
            ("HYG", "iShares iBoxx $ High Yield Corporate Bond ETF"),
            ("LQD", "iShares iBoxx $ Investment Grade Corporate Bond ETF"),
            ("UVXY", "ProShares Ultra VIX Short-Term Futures ETF"),
            ("VXX", "iPath Series B S&P 500 VIX Short-Term Futures ETN"),
            ("USO", "United States Oil Fund"),
            ("UNG", "United States Natural Gas Fund"),
        ];

        symbols
            .into_iter()
            .map(|(symbol, name)| SymbolMetadata {
                symbol: symbol.to_string(),
                name: name.to_string(),
                market_type: MarketType::ETF,
                sector: None,
                exchange: "NYSE/NASDAQ".to_string(),
            })
            .collect()
    }

    /// Cryptocurrency pairs (Binance USDT pairs)
    pub fn crypto() -> Vec<SymbolMetadata> {
        let symbols = vec![
            ("BTCUSDT", "Bitcoin"),
            ("ETHUSDT", "Ethereum"),
            ("BNBUSDT", "Binance Coin"),
            ("XRPUSDT", "Ripple"),
            ("ADAUSDT", "Cardano"),
            ("DOGEUSDT", "Dogecoin"),
            ("SOLUSDT", "Solana"),
            ("DOTUSDT", "Polkadot"),
            ("MATICUSDT", "Polygon"),
            ("AVAXUSDT", "Avalanche"),
            ("LINKUSDT", "Chainlink"),
            ("UNIUSDT", "Uniswap"),
            ("ATOMUSDT", "Cosmos"),
            ("LTCUSDT", "Litecoin"),
            ("ETCUSDT", "Ethereum Classic"),
            ("XLMUSDT", "Stellar"),
            ("NEARUSDT", "NEAR Protocol"),
            ("APTUSDT", "Aptos"),
            ("ARBUSDT", "Arbitrum"),
            ("OPUSDT", "Optimism"),
            ("AAVEUSDT", "Aave"),
            ("FTMUSDT", "Fantom"),
            ("SUIUSDT", "Sui"),
            ("SHIBUSDT", "Shiba Inu"),
            ("TRXUSDT", "TRON"),
            ("LDOUSDT", "Lido DAO"),
            ("INJUSDT", "Injective"),
            ("STXUSDT", "Stacks"),
            ("FILUSDT", "Filecoin"),
            ("ICPUSDT", "Internet Computer"),
        ];

        symbols
            .into_iter()
            .map(|(symbol, name)| SymbolMetadata {
                symbol: symbol.to_string(),
                name: name.to_string(),
                market_type: MarketType::Crypto,
                sector: None,
                exchange: "Binance".to_string(),
            })
            .collect()
    }

    /// Get symbols filtered by market type
    pub fn by_market_type(market_type: MarketType) -> Vec<SymbolMetadata> {
        Self::all()
            .into_iter()
            .filter(|s| s.market_type == market_type)
            .collect()
    }

    /// Get just the symbol strings for quick access
    pub fn symbols_only() -> Vec<String> {
        Self::all().into_iter().map(|s| s.symbol).collect()
    }

    /// Get crypto symbols only
    pub fn crypto_symbols() -> Vec<String> {
        Self::crypto().into_iter().map(|s| s.symbol).collect()
    }

    /// Get stock symbols only
    pub fn stock_symbols() -> Vec<String> {
        let mut symbols = Vec::new();
        symbols.extend(Self::sp500().into_iter().map(|s| s.symbol));
        symbols.extend(Self::nasdaq100().into_iter().map(|s| s.symbol));
        symbols.extend(Self::dow30().into_iter().map(|s| s.symbol));
        symbols.extend(Self::russell_top200().into_iter().map(|s| s.symbol));
        symbols.sort();
        symbols.dedup();
        symbols
    }

    /// Get ETF symbols only
    pub fn etf_symbols() -> Vec<String> {
        let mut symbols = Vec::new();
        symbols.extend(Self::leveraged_etfs().into_iter().map(|s| s.symbol));
        symbols.extend(Self::top_etfs().into_iter().map(|s| s.symbol));
        symbols.sort();
        symbols.dedup();
        symbols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_has_symbols() {
        let all = SymbolUniverse::all();
        assert!(!all.is_empty());
        assert!(all.len() > 100);
    }

    #[test]
    fn test_crypto_symbols() {
        let crypto = SymbolUniverse::crypto();
        assert!(crypto.iter().any(|s| s.symbol == "BTCUSDT"));
        assert!(crypto.iter().any(|s| s.symbol == "ETHUSDT"));
    }

    #[test]
    fn test_filter_by_market_type() {
        let stocks = SymbolUniverse::by_market_type(MarketType::Stock);
        assert!(!stocks.is_empty());
        assert!(stocks.iter().all(|s| s.market_type == MarketType::Stock));
    }
}
