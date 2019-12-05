use rust_decimal::Decimal;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pong {}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: Atom,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: RateLimitInterval,
    pub interval_num: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RateLimitType {
    #[serde(rename = "REQUEST_WEIGHT")]
    RequestWeight,
    #[serde(rename = "ORDERS")]
    Orders,
    #[serde(rename = "RAW_REQUESTS")]
    RawRequests,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RateLimitInterval {
    #[serde(rename = "SECOND")]
    Second,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "DAY")]
    Day,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: Atom,
    pub status: SymbolStatus,
    pub base_asset: Atom,
    pub base_asset_precision: u16,
    pub quote_asset:Atom,
    pub quote_precision: u16,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SymbolStatus {
    #[serde(rename = "PRE_TRADING")]
    PreTrading,
    #[serde(rename = "TRADING")]
    Trading,
    #[serde(rename = "POST_TRADING")]
    PostTrading,
    #[serde(rename = "END_OF_DAY")]
    EndOfDay,
    #[serde(rename = "HALT")]
    Halt,
    #[serde(rename = "AUCTION_MATCH")]
    AuctionMatch,
    #[serde(rename = "BREAK")]
    Break,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit = 1,
    #[serde(rename = "MARKET")]
    Market = 2,
    #[serde(rename = "STOP_LOSS")]
    StopLoss = 4,
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit = 8,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit = 16,
    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit = 32,
    #[serde(rename = "LIMIT_MAKER")]
    LimitMaker = 64,
}

/// Filters define trading rules on a symbol or an exchange. Filters come in two forms:
/// symbol filters and exchange filters.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filter {
    /// The PRICE_FILTER defines the price rules for a symbol. There are 3 parts:
    ///
    /// * `min_price` defines the minimum `price`/`stop_price` allowed;
    ///   disabled on `min_price` == 0.
    /// * `max_price` defines the maximum `price`/`stop_price` allowed;
    ///   disabled on `max_price` == 0.
    /// * `tick_size` defines the intervals that a `price`/`stop_price`
    ///   can be increased/decreased by; disabled on `tick_size` == 0.
    ///
    /// Any of the above variables can be set to 0, which disables that rule in the price filter.
    /// In order to pass the price filter, the following must be true for `price`/`stop_price`
    /// of the enabled rules:
    ///
    /// * `price` >= `min_price`
    /// * `price` <= `max_price`
    /// * (`price` - `min_price`) % `tick_size` == 0
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter { min_price: Decimal, max_price: Decimal, tick_size: Decimal },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice { multiplier_up: Decimal, multiplier_down: Decimal, avg_price_mins: u64 },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize { min_qty: Decimal, max_qty: Decimal, step_size: Decimal },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional { min_notional: Decimal, apply_to_market: bool, avg_price_mins: u64 },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: u64 },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize { min_qty: Decimal, max_qty: Decimal, step_size: Decimal },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { limit: u64 },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: u64 },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u64 },
}

// FIXME clarify: the documentation is ambiguous; only these values are listed as valid,
//       but below it has a caution about value 0.
//       [https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#order-book]
//       If 0 is valid, add it and specify its weight.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderBookLimit {
    N5 = 5,
    N10 = 10,
    N20 = 20,
    N50 = 50,
    N100 = 100,
    N500 = 500,
    N1000 = 1000,
}

impl OrderBookLimit {
    pub fn weight(self) -> u32 {
        use OrderBookLimit::*;
        match self {
            N5 | N10 | N20 | N50 | N100 => 1,
            N500 => 5,
            N1000 => 10,
        }
    }

    pub fn as_str(self) -> &'static str {
        use OrderBookLimit::*;
        match self {
            N5 => "5",
            N10 => "10",
            N20 => "20",
            N50 => "50",
            N100 => "100",
            N500 => "500",
            N1000 => "1000",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderBookStreamLimit {
    N5 = 5,
    N10 = 10,
    N20 = 20,
}

impl OrderBookStreamLimit {
    pub fn as_str(self) -> &'static str {
        use OrderBookStreamLimit::*;
        match self {
            N5 => "5",
            N10 => "10",
            N20 => "20",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bid>,
    pub asks: Vec<Ask>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Bid {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Ask {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: Decimal,
    pub qty: Decimal,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggTrade {
    #[serde(rename = "a")]
    pub id: u64,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "q")]
    pub qty: Decimal,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ChartInterval {
    #[serde(rename = "1m")]
    Minute1,
    #[serde(rename = "3m")]
    Minute3,
    #[serde(rename = "5m")]
    Minute5,
    #[serde(rename = "15m")]
    Minute15,
    #[serde(rename = "30m")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "2h")]
    Hour2,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "6h")]
    Hour6,
    #[serde(rename = "8h")]
    Hour8,
    #[serde(rename = "12h")]
    Hour12,
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "3d")]
    Day3,
    #[serde(rename = "1w")]
    Week1,
    #[serde(rename = "1M")]
    Month1,
}

impl ChartInterval {
    pub fn as_str(self) -> &'static str {
        use ChartInterval::*;
        match self {
            Minute1 => "1m",
            Minute3 => "3m",
            Minute5 => "5m",
            Minute15 => "15m",
            Minute30 => "30m",
            Hour1 => "1h",
            Hour2 => "2h",
            Hour4 => "4h",
            Hour6 => "6h",
            Hour8 => "8h",
            Hour12 => "12h",
            Day1 => "1d",
            Day3 => "3d",
            Week1 => "1w",
            Month1 => "1M",
        }
    }
}

// FIXME serialize as tuple
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Kline {
    pub open_time: u64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub close_time: u64,
    pub quote_asset_volume: Decimal,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
    pub ignore: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AvgPrice {
    pub mins: u16,
    pub price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TickerStats {
    pub symbol: Atom,
    pub price_change: Decimal,
    pub price_change_percent: Decimal,
    pub weighted_avg_price: Decimal,
    pub prev_close_price: Decimal,
    pub last_price: Decimal,
    pub last_qty: Decimal,
    pub bid_price: Decimal,
    pub ask_price: Decimal,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub volume: Decimal,
    pub open_time: u64,
    pub close_time: u64,
    // FIXME Option<u64> when value is -1
    pub first_id: i64,
    // FIXME Option<u64> when value is -1
    pub last_id: i64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct PriceTicker {
    pub symbol: Atom,
    pub price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    pub symbol: Atom,
    pub bid_price: Decimal,
    pub bid_qty: Decimal,
    pub ask_price: Decimal,
    pub ask_qty: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ListenKey {
    pub listen_key: String,
}

//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct TradeHistory {
//    pub id: u64,
//    pub price: Decimal,
//    pub qty: Decimal,
//    pub commission: String,
//    pub commission_asset: String,
//    pub time: u64,
//    pub is_buyer: bool,
//    pub is_maker: bool,
//    pub is_best_match: bool,
//}

//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct AccountInformation {
//    pub maker_commission: f32,
//    pub taker_commission: f32,
//    pub buyer_commission: f32,
//    pub seller_commission: f32,
//    pub can_trade: bool,
//    pub can_withdraw: bool,
//    pub can_deposit: bool,
//    pub balances: Vec<Balance>,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct Balance {
//    pub asset: String,
//    pub free: String,
//    pub locked: String,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct Order {
//    pub symbol: String,
//    pub order_id: u64,
//    pub client_order_id: String,
//    #[serde(with = "string_or_float")] pub price: f64,
//    pub orig_qty: String,
//    pub executed_qty: String,
//    pub status: String,
//    pub time_in_force: String,
//    #[serde(rename = "type")] pub type_name: String,
//    pub side: String,
//    #[serde(with = "string_or_float")] pub stop_price: f64,
//    pub iceberg_qty: String,
//    pub time: u64,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct OrderCanceled {
//    pub symbol: String,
//    pub orig_client_order_id: String,
//    pub order_id: u64,
//    pub client_order_id: String,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct Transaction {
//    pub symbol: String,
//    pub order_id: u64,
//    pub client_order_id: String,
//    pub transact_time: u64,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct UserDataStream {
//    pub listen_key: String,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct Success {}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//#[serde(untagged)]
//pub enum Prices {
//    AllPrices(Vec<SymbolPrice>),
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct SymbolPrice {
//    pub symbol: String,
//    #[serde(with = "string_or_float")] pub price: f64,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//#[serde(untagged)]
//pub enum BookTickers {
//    AllBookTickers(Vec<Tickers>),
//}
//
//#[derive(Debug, Clone)]
//pub enum KlineSummaries {
//    AllKlineSummaries(Vec<KlineSummary>),
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct AccountUpdateEvent {
//    #[serde(rename = "e")] pub event_type: String,
//
//    #[serde(rename = "E")] pub event_time: u64,
//
//    m: u64,
//    t: u64,
//    b: u64,
//    s: u64,
//
//    #[serde(rename = "T")] t_ignore: bool,
//    #[serde(rename = "W")] w_ignore: bool,
//    #[serde(rename = "D")] d_ignore: bool,
//
//    #[serde(rename = "B")] pub balance: Vec<EventBalance>,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct EventBalance {
//    #[serde(rename = "a")] pub asset: String,
//    #[serde(rename = "f")] pub free: String,
//    #[serde(rename = "l")] pub locked: String,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct OrderTradeEvent {
//    #[serde(rename = "e")] pub event_type: String,
//
//    #[serde(rename = "E")] pub event_time: u64,
//
//    #[serde(rename = "s")] pub symbol: String,
//
//    #[serde(rename = "c")] pub new_client_order_id: String,
//
//    #[serde(rename = "S")] pub side: String,
//
//    #[serde(rename = "o")] pub order_type: String,
//
//    #[serde(rename = "f")] pub time_in_force: String,
//
//    #[serde(rename = "q")] pub qty: String,
//
//    #[serde(rename = "p")] pub price: String,
//
//    #[serde(skip_serializing, rename = "P")] pub p_ignore: String,
//
//    #[serde(skip_serializing, rename = "F")] pub f_ignore: String,
//
//    #[serde(skip_serializing)] pub g: i32,
//
//    #[serde(skip_serializing, rename = "C")] pub c_ignore: Option<String>,
//
//    #[serde(rename = "x")] pub execution_type: String,
//
//    #[serde(rename = "X")] pub order_status: String,
//
//    #[serde(rename = "r")] pub order_reject_reason: String,
//
//    #[serde(rename = "i")] pub order_id: u64,
//
//    #[serde(rename = "l")] pub qty_last_filled_trade: String,
//
//    #[serde(rename = "z")] pub accumulated_qty_filled_trades: String,
//
//    #[serde(rename = "L")] pub price_last_filled_trade: String,
//
//    #[serde(rename = "n")] pub commission: String,
//
//    #[serde(skip_serializing, rename = "N")] pub asset_commisioned: Option<String>,
//
//    #[serde(rename = "T")] pub trade_order_time: u64,
//
//    #[serde(rename = "t")] pub trade_id: i64,
//
//    #[serde(skip_serializing, rename = "I")] pub i_ignore: u64,
//
//    #[serde(skip_serializing)] pub w: bool,
//
//    #[serde(rename = "m")] pub is_buyer_maker: bool,
//
//    #[serde(skip_serializing, rename = "M")] pub m_ignore: bool,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct DayTickerEvent {
//    #[serde(rename = "e")] pub event_type: String,
//
//    #[serde(rename = "E")] pub event_time: u64,
//
//    #[serde(rename = "s")] pub symbol: String,
//
//    #[serde(rename = "p")] pub price_change: String,
//
//    #[serde(rename = "P")] pub price_change_percent: String,
//
//    #[serde(rename = "w")] pub average_price: String,
//
//    #[serde(rename = "x")] pub prev_close: String,
//
//    #[serde(rename = "c")] pub current_close: String,
//
//    #[serde(rename = "Q")] pub current_close_qty: String,
//
//    #[serde(rename = "b")] pub best_bid: String,
//
//    #[serde(rename = "B")] pub best_bid_qty: String,
//
//    #[serde(rename = "a")] pub best_ask: String,
//
//    #[serde(rename = "A")] pub best_ask_qty: String,
//
//    #[serde(rename = "o")] pub open: String,
//
//    #[serde(rename = "h")] pub high: String,
//
//    #[serde(rename = "l")] pub low: String,
//
//    #[serde(rename = "v")] pub volume: String,
//
//    #[serde(rename = "q")] pub quote_volume: String,
//
//    #[serde(rename = "O")] pub open_time: u64,
//
//    #[serde(rename = "C")] pub close_time: u64,
//
//    #[serde(rename = "F")] pub first_trade_id: u64,
//
//    #[serde(rename = "L")] pub last_trade_id: u64,
//
//    #[serde(rename = "n")] pub num_trades: u64,
//}
//

pub enum StreamEvent {
    AggTrade(AggTradeEvent),
    Trade(TradeEvent),
    Kline(KlineEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct AggTradeEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "a")]
    pub id: u64,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "q")]
    pub qty: Decimal,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TradeEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "t")]
    pub id: u64,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "q")]
    pub qty: Decimal,
    #[serde(rename = "b")]
    pub buyer_order_id: u64,
    #[serde(rename = "a")]
    pub seller_order_id: u64,
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M")]
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct KlineEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "k")]
    pub kline: WSKline,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct WSKline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "i")]
    pub interval: ChartInterval,
    #[serde(rename = "f")]
    pub first_trade_id: i32,
    #[serde(rename = "L")]
    pub last_trade_id: i32,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub volume: Decimal,
    #[serde(rename = "n")]
    pub number_of_trades: i32,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
    #[serde(rename = "V")]
    pub active_buy_volume: Decimal,
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: Decimal,
    #[serde(skip, rename = "B")]
    pub ignore: (),
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct MiniTickerEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub base_volume: Decimal,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TickerEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "p")]
    pub price_change: Decimal,
    #[serde(rename = "P")]
    pub price_change_percent: Decimal,
    #[serde(rename = "w")]
    pub weighted_avg_price: Decimal,
    #[serde(rename = "x")]
    pub first_trade: Decimal,
    #[serde(rename = "c")]
    pub last_price: Decimal,
    #[serde(rename = "Q")]
    pub last_qty: Decimal,
    #[serde(rename = "b")]
    pub best_bid_price: Decimal,
    #[serde(rename = "B")]
    pub best_bid_qty: Decimal,
    #[serde(rename = "a")]
    pub best_ask_price: Decimal,
    #[serde(rename = "A")]
    pub best_ask_qty: Decimal,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub base_volume: Decimal,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
    #[serde(rename = "O")]
    pub stats_open_time: u64,
    #[serde(rename = "C")]
    pub stats_close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub number_of_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DiffOrderBookEvent {
    #[serde(skip, rename = "e")]
    pub event_type: (),
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: Atom,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    #[serde(rename = "a")]
    pub asks: Vec<Ask>
}
