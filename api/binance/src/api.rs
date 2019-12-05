use futures::Stream;

use crate::client::{Config, Client, Keys};
use crate::error::*;
use crate::model::*;

const V1_PING: &str = "/api/v1/ping";
const V1_TIME: &str = "/api/v1/time";
const V1_EXCHANGE_INFO: &str = "/api/v1/exchangeInfo";
const V1_DEPTH: &str = "/api/v1/depth";
const V1_TRADES: &str = "/api/v1/trades";
const V1_HISTORICAL_TRADES: &str = "/api/v1/historicalTrades";
const V1_AGG_TRADES: &str = "/api/v1/aggTrades";
const V1_KLINES: &str = "/api/v1/klines";
const V3_AVG_PRICE: &str = "/api/v3/avgPrice";
const V1_TICKER_24HR: &str = "/api/v1/ticker/24hr";
const V3_TICKER_PRICE: &str = "/api/v3/ticker/price";
const V3_TICKER_BOOK_TICKER: &str = "/api/v3/ticker/bookTicker";
const V1_USER_DATA_STREAM: &str = "/api/v1/userDataStream";

#[derive(Default)]
pub struct Api {
    pub client: Client,
}

impl Api {
    pub fn new() -> Self {
        Api::default()
    }

    pub fn with_keys(keys: Keys) -> Self {
        Api::with_config(Config { keys, ..Config::default() })
    }

    pub fn with_config(config: Config) -> Self {
        let client = Client::with_config(config);
        Api { client }
    }

    /// Test connectivity to the Rest API.
    ///
    /// Weight: 1
    pub async fn ping(&self) -> Result<Pong> {
        self.client.get(V1_PING)?
            .send().await
    }

    /// Test connectivity to the Rest API and get the current server time.
    ///
    /// Weight: 1
    pub async fn time(&self) -> Result<ServerTime> {
        self.client.get(V1_TIME)?
            .send().await
    }

    /// Current exchange trading rules and symbol information.
    ///
    /// Weight: 1
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get(V1_EXCHANGE_INFO)?
            .send().await
    }

    /// Order book.
    ///
    /// Weight: Adjusted based on the limit:
    ///
    /// Limit | Weight
    /// | ---- | ---- |
    /// 5, 10, 20, 50, 100 | 1
    /// 500 | 5
    /// 1000 | 10
    ///
    /// The default `limit` value is `N100`.
    pub async fn depth<S: AsRef<str>>(&self, symbol: S, limit: Option<OrderBookLimit>)
        -> Result<OrderBook>
    {
        self.client.get(V1_DEPTH)?
            .query_arg("symbol", symbol.as_ref())
            .try_query_arg("limit", &limit.map(OrderBookLimit::as_str))
            .send().await
    }

    /// Recent trades list.
    ///
    /// Get recent trades (up to last 500).
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    /// * `limit` - default 500; max 1000.
    pub async fn trades<S: AsRef<str>>(&self, symbol: S, limit: Option<usize>)
        -> Result<Vec<Trade>>
    {
        self.client.get(V1_TRADES)?
            .query_arg("symbol", symbol.as_ref())
            .try_query_arg("limit", &limit)
            .send().await
    }

    /// Old trade lookup (MARKET_DATA).
    ///
    /// Get older trades.
    ///
    /// Weight: 5
    ///
    /// Parameters:
    /// * `symbol`
    /// * `from_id` - trade id to fetch from. Default gets most recent trades.
    /// * `limit` - default 500; max 1000.
    pub async fn historical_trades<S: AsRef<str>>(
        &self,
        symbol: S,
        from_id: Option<u64>,
        limit: Option<usize>,
    )
        -> Result<Vec<Trade>>
    {
        self.client.get(V1_HISTORICAL_TRADES)?
            .auth_header()?
            .query_arg("symbol", symbol.as_ref())
            .try_query_arg("fromId", &from_id)
            .try_query_arg("limit", &limit)
            .send().await
    }

    /// Compressed/Aggregate trades list.
    ///
    /// Get compressed, aggregate trades. Trades that fill at the time, from the same order,
    /// with the same price will have the quantity aggregated.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    /// * `from_id` - trade id to fetch from. Default gets most recent trades.
    /// * `start_time` - timestamp in ms to get aggregate trades from INCLUSIVE.
    /// * `end_time` - timestamp in ms to get aggregate trades until INCLUSIVE.
    /// * `limit` - default 500; max 1000.
    ///
    ///
    /// * If both startTime and endTime are sent, time between startTime and endTime
    ///   must be less than 1 hour.
    /// * If fromId, startTime, and endTime are not sent, the most recent aggregate trades
    ///   will be returned.
    pub async fn agg_trades<S: AsRef<str>>(
        &self,
        symbol: S,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<usize>,
    )
        -> Result<Vec<AggTrade>>
    {
        self.client.get(V1_AGG_TRADES)?
            .query_arg("symbol", symbol.as_ref())
            .try_query_arg("fromId", &from_id)
            .try_query_arg("startTime", &start_time)
            .try_query_arg("endTime", &end_time)
            .try_query_arg("limit", &limit)
            .send().await
    }

    /// Kline/Candlestick data.
    ///
    /// Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    /// * `interval`
    /// * `start_time`
    /// * `end_time`
    /// * `limit` - default 500; max 1000.
    ///
    ///
    /// * If `start_time` and `end_time` are not sent, the most recent klines are returned.
    pub async fn klines<S: AsRef<str>>(
        &self,
        symbol: S,
        interval: ChartInterval,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<usize>,
    )
        -> Result<Vec<Kline>>
    {
        self.client.get(V1_KLINES)?
            .query_args(&[
                ("symbol", symbol.as_ref()),
                ("interval", interval.as_str())
            ])
            .try_query_arg("startTime", &start_time)
            .try_query_arg("endTime", &end_time)
            .try_query_arg("limit", &limit)
            .send().await
    }

    /// Current average price.
    ///
    /// Current average price for a symbol.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    pub async fn avg_price<S: AsRef<str>>(&self, symbol: S) -> Result<AvgPrice> {
        self.client.get(V3_AVG_PRICE)?
            .query_arg("symbol", symbol.as_ref())
            .send().await
    }

    /// 24hr ticker price change statistics.
    ///
    /// 24 hour rolling window price change statistics.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    pub async fn ticker_24hr<S: AsRef<str>>(&self, symbol: S) -> Result<TickerStats> {
        self.client.get(V1_TICKER_24HR)?
            .query_arg("symbol", symbol.as_ref())
            .send().await
    }

    /// 24hr ticker price change statistics.
    ///
    /// 24 hour rolling window price change statistics.
    ///
    /// Weight: 40
    pub async fn ticker_24hr_all(&self) -> Result<Vec<TickerStats>> {
        self.client.get(V1_TICKER_24HR)?
            .send().await
    }

    /// Symbol price ticker.
    ///
    /// Latest price for a symbol or symbols.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    pub async fn ticker_price<S: AsRef<str>>(&self, symbol: S) -> Result<PriceTicker> {
        self.client.get(V3_TICKER_PRICE)?
            .query_arg("symbol", symbol.as_ref())
            .send().await
    }

    /// Symbol price ticker.
    ///
    /// Latest price for a symbol or symbols.
    ///
    /// Weight: 2
    pub async fn ticker_price_all(&self) -> Result<Vec<PriceTicker>> {
        self.client.get(V3_TICKER_PRICE)?
            .send().await
    }

    /// Symbol order book ticker.
    ///
    /// Best price/qty on the order book for a symbol or symbols.
    ///
    /// Weight: 1
    ///
    /// Parameters:
    /// * `symbol`
    pub async fn ticker_book<S: AsRef<str>>(&self, symbol: S) -> Result<BookTicker> {
        self.client.get(V3_TICKER_BOOK_TICKER)?
            .query_arg("symbol", symbol.as_ref())
            .send().await
    }

    /// Symbol order book ticker.
    ///
    /// Best price/qty on the order book for a symbol or symbols.
    pub async fn ticker_book_all(&self) -> Result<Vec<BookTicker>> {

        self.client.get(V3_TICKER_BOOK_TICKER)?
            .send().await
    }

    /// Create a listenKey.
    ///
    /// Start a new user data stream.
    /// The stream will close after 60 minutes unless a keepalive is sent.
    ///
    /// Weight: 1
    pub async fn user_data_stream(&self) -> Result<ListenKey> {
        self.client.post(V1_USER_DATA_STREAM)?
            .auth_header()?
            .send().await
    }

    /// Aggregate Trade Streams.
    ///
    /// The Aggregate Trade Streams push trade information that is aggregated for a single taker
    /// order.
    ///
    /// Stream Name: `<symbol>@aggTrade`
    pub async fn ws_agg_trade<S>(&self, symbol: S) -> Result<impl Stream<Item=Result<AggTradeEvent>>>
    where
        S: AsRef<str>,
    {
        let name = format!("{}@aggTrade", symbol.as_ref());
        self.client
            .web_socket2(&name).await?
            .connect().await
    }

    /// Trade Streams.
    ///
    /// The Trade Streams push raw trade information; each trade has a unique buyer and seller.
    ///
    /// Stream Name: `<symbol>@trade`
    pub async fn ws_trade<S>(&self, symbol: S) -> Result<impl Stream<Item=Result<TradeEvent>>>
    where
        S: AsRef<str>,
    {
        let name = format!("{}@trade", symbol.as_ref());
        self.client
            .web_socket2(&name).await?
            .connect().await
    }

    /// Kline/Candlestick Streams.
    ///
    /// The Kline/Candlestick Stream push updates to the current klines/candlestick every second.
    ///
    /// Stream Name: `<symbol>@kline_<interval>`
    pub async fn ws_kline<S>(&self, symbol: S, interval: ChartInterval)
        -> Result<impl Stream<Item=Result<KlineEvent>>>
    where
        S: AsRef<str>,
    {
        let name = format!("{}@kline_{}", symbol.as_ref(), interval.as_str());
        self.client
            .web_socket2(&name).await?
            .connect().await
    }

    /// Individual Symbol Mini Ticker Stream.
    ///
    /// 24hr rolling window mini-ticker statistics for a single symbol pushed every second.
    /// These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime
    /// to 24hrs before.
    ///
    /// Stream Name: `<symbol>@miniTicker`
    pub async fn ws_mini_ticker<S>(&self, symbol: S) -> Result<impl Stream<Item=Result<MiniTickerEvent>>>
    where
        S: AsRef<str>,
    {
        let name = format!("{}@miniTicker", symbol.as_ref());
        self.client
            .web_socket2(&name).await?
            .connect().await
    }

    /// All Market Mini Tickers Stream.
    ///
    /// 24hr rolling window mini-ticker statistics for all symbols that changed in an array pushed
    /// every second. These are NOT the statistics of the UTC day, but a 24hr rolling window from
    /// requestTime to 24hrs before.
    ///
    /// Stream Name: `!miniTicker@arr`
    pub async fn ws_mini_ticker_all(&self) -> Result<impl Stream<Item=Result<Vec<MiniTickerEvent>>>>
    {
        let name = "!miniTicker@arr";
        self.client
            .web_socket2(&name).await?
            .connect().await
    }

    /// Individual Symbol Ticker Streams.
    ///
    /// 24hr rollwing window ticker statistics for a single symbol pushed every second. These are
    /// NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs
    /// before.
    ///
    /// Stream Name: `<symbol>@ticker`
    pub async fn ws_ticker<S>(&self, symbol: S) -> Result<impl Stream<Item=Result<TickerEvent>>>
    where
        S: AsRef<str>,
    {
        let name = format!("{}@ticker", symbol.as_ref());
        self.client
            .web_socket2(&name).await?
            .connect().await
    }

    /// All Market Tickers Stream.
    ///
    /// 24hr rolling window ticker statistics for all symbols that changed in an array pushed every
    /// second. These are NOT the statistics of the UTC day, but a 24hr rolling window from
    /// requestTime to 24hrs before.
    ///
    /// Stream Name: `!ticker@arr`
    pub async fn ws_ticker_all(&self) -> Result<impl Stream<Item=Result<TickerEvent>>>
    {
        let name = "!ticker@arr";
        self.client
            .web_socket2(&name).await?
            .connect().await
    }

    /// Partial Book Depth Streams.
    ///
    /// Top `levels` bids and asks, pushed every second.
    ///
    /// Stream Name: `<symbol>@depth<levels>`
    pub async fn ws_partial_depth<S>(&self, symbol: S, levels: OrderBookStreamLimit)
        -> Result<impl Stream<Item=Result<OrderBook>>>
    where
        S: AsRef<str>,
    {
        let name = format!("{}@depth{}", symbol.as_ref(), levels.as_str());
        self.client
            .web_socket2(&name).await?
            .connect().await
    }

    /// Diff. Depth Stream.
    ///
    /// Order book price and quantity depth updates used to locally manage an order book pushed
    /// every second.
    ///
    /// Stream Name: `<symbol>@depth`
    pub async fn ws_diff_depth<S>(&self, symbol: S) -> Result<impl Stream<Item=Result<DiffOrderBookEvent>>>
    where
        S: AsRef<str>,
    {
        let name = format!("{}@depth", symbol.as_ref());
        self.client
            .web_socket2(&name).await?
            .connect().await
    }
}
