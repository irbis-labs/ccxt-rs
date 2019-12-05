use ccxt_binance::Api;
use ccxt_binance::Error;
//use ccxt_binance::ErrorKind::*;
use futures::{FutureExt, TryFutureExt};
use tokio::runtime::current_thread::run;

fn main() {
    let api = Api::new();

    let future = async move {
        let res = async move {
            println!("{:?}", api.ping().await?);
            println!("Server Time: {}", api.time().await?.server_time);
            println!("Exchenge Info: {:#?}", api.exchange_info().await?);
            Ok(())
        };
        if let Err::<(), Error>(e) = res.await {
            println!("Error: {}", e)
        }
    };

    run(future.unit_error().boxed_local().compat())
}

//fn main() {
//    let api = Api::new();
//
//    let future = api.ping()
//        .and_then(|answer| {
//            println!("{:?}", answer);
//            api.time()
//        })
//        .and_then(|answer| {
//            println!("Server Time: {}", answer.server_time);
//            api.exchange_info()
//        })
//        .and_then(|answer| {
//            println!("Exchenge Info: {:#?}", answer);
//            Ok(())
//        });
//
//    if let Err(e) = block_on_all(future) {
//        println!("Error: {}", e)
//    }
//}
