use std::f32::consts::E;

use arb_bot_2::{
    structs::{Graph, Pair},
    functions::find_arbitrage
};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use ftx::ws;
use ftx::ws::{Channel, Data, Orderbook, Ws};
use ftx::{
    options::Options,
    rest::{PlaceOrder, Side, OrderType, Rest},
};
use futures::stream::StreamExt;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> ws::Result<()> {
    #![allow(non_snake_case)]

    dotenv::dotenv().ok();

    let mut websocket = Ws::connect(Options::from_env()).await?;
    let api = Rest::new(Options::from_env());

    let mut graph = Graph::new();

    let solbtc = Pair::new("SOL", "BTC", true);
    let btcsol = Pair::new("BTC", "SOL", false);
    let btcusdt = Pair::new("BTC", "USDT", true);
    let usdtbtc = Pair::new("USDT", "BTC", false);
    let solusdt = Pair::new("SOL", "USDT", true);
    let usdtsol = Pair::new("USDT", "SOL", false);
    let btcusd = Pair::new("BTC", "USD", true);
    let usdbtc = Pair::new("USD", "BTC", false);
    let usdtusd = Pair::new("USDT", "USD", true);
    let usdusdt = Pair::new("USD", "USDT", false);
    let fttbtc = Pair::new("FTT", "BTC", true);
    let btcftt = Pair::new("BTC", "FTT", false);
    let fttusdt = Pair::new("FTT", "USDT", true);
    let usdtftt = Pair::new("USDT", "FTT", false);
    let fttusd = Pair::new("FTT", "USD", true);
    let usdftt = Pair::new("USD", "FTT", false);
    let solusd = Pair::new("SOL", "USD", true);
    let usdsol = Pair::new("USD", "SOL", false);

    //all pairs with to_quote = true
    let mut solbtcorderbook = Orderbook::new(solbtc.get_symbol());
    let mut btcusdtorderbook = Orderbook::new(btcusdt.get_symbol());
    let mut solusdtorderbook = Orderbook::new(solusdt.get_symbol());
    let mut btcusdorderbook = Orderbook::new(btcusd.get_symbol());
    let mut usdtusdorderbook = Orderbook::new(usdtusd.get_symbol());
    let mut fttbtcorderbook = Orderbook::new(fttbtc.get_symbol());
    let mut fttusdtorderbook = Orderbook::new(fttusdt.get_symbol());
    let mut fttusdorderbook = Orderbook::new(fttusd.get_symbol());
    let mut solusdorderbook = Orderbook::new(solusd.get_symbol());

    websocket.subscribe(vec![
        Channel::Orderbook(solbtc.get_symbol()),
        Channel::Orderbook(btcusdt.get_symbol()),
        Channel::Orderbook(solusdt.get_symbol()),
        Channel::Orderbook(btcusd.get_symbol()),
        Channel::Orderbook(usdtusd.get_symbol()),
        Channel::Orderbook(fttbtc.get_symbol()),
        Channel::Orderbook(fttusdt.get_symbol()),
        Channel::Orderbook(fttusd.get_symbol()),
        Channel::Orderbook(solusd.get_symbol()),
    ]).await?;

    graph.add_pair(solbtc.clone());
    graph.add_pair(btcsol.clone());
    graph.add_pair(btcusdt.clone());
    graph.add_pair(usdtbtc.clone());
    graph.add_pair(solusdt.clone());
    graph.add_pair(usdtsol.clone());
    graph.add_pair(btcusd.clone());
    graph.add_pair(usdbtc.clone());
    graph.add_pair(usdtusd.clone());
    graph.add_pair(usdusdt.clone());
    graph.add_pair(fttbtc.clone());
    graph.add_pair(btcftt.clone());
    graph.add_pair(fttusdt.clone());
    graph.add_pair(usdtftt.clone());
    graph.add_pair(fttusd.clone());
    graph.add_pair(usdftt.clone());
    graph.add_pair(solusd.clone());
    graph.add_pair(usdsol.clone());

    let sb = solbtc.clone();
    let but = btcusdt.clone();
    let sut = solusdt.clone();
    let bu = btcusd.clone();
    let utu = usdtusd.clone();
    let fb = fttbtc.clone();
    let fut = fttusdt.clone();
    let fu = fttusd.clone();
    let su = solusd.clone();

    let one = Decimal::from(1);

    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        loop {
            let data = websocket.next().await.expect("No data");

            match data {
                Ok(da) => tx.send(da).await.unwrap(),
                Err(e) => {
                    println!("Error: {}", e);
                    websocket = match Ws::connect(Options::from_env()).await {
                        Ok(mut ws) => {
                            match ws.subscribe(vec![
                                Channel::Orderbook(sb.get_symbol()),
                                Channel::Orderbook(but.get_symbol()),
                                Channel::Orderbook(sut.get_symbol()),
                                Channel::Orderbook(bu.get_symbol()),
                                Channel::Orderbook(utu.get_symbol()),
                                Channel::Orderbook(fb.get_symbol()),
                                Channel::Orderbook(fut.get_symbol()),
                                Channel::Orderbook(fu.get_symbol()),
                                Channel::Orderbook(su.get_symbol()),
                        ]).await {
                            Ok(()) => {},
                            Err(err) => {
                                println!("Error: {}", err);
                                std::thread::sleep(std::time::Duration::from_secs(5));
                                continue;
                            },
                        };
                            ws
                        },
                        Err(er) => {
                            println!("{:?}", er);
                            std::thread::sleep(std::time::Duration::from_secs(5));
                            continue;
                        },
                    };
                },
            }
        }
    });

    while let Some(obdata) = rx.recv().await {
        match obdata {
            (obsymbol, Data::OrderbookData(orderbook_data)) => {
                match obsymbol {
                    Some(sym) => {
                        match sym.as_str() {
                            "SOL/BTC" => {
                                solbtcorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = solbtcorderbook.best_ask().unwrap();
                                graph.update_edge(&solbtc, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = solbtcorderbook.best_bid().unwrap();
                                graph.update_edge(&btcsol, one/bidprice, bidquantity);
                            }
                            "BTC/USDT" => {
                                btcusdtorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = btcusdtorderbook.best_ask().unwrap();
                                graph.update_edge(&btcusdt, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = btcusdtorderbook.best_bid().unwrap();
                                graph.update_edge(&usdtbtc, one/bidprice, bidquantity);
                            }
                            "SOL/USDT" => {
                                solusdtorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = solusdtorderbook.best_ask().unwrap();
                                graph.update_edge(&solusdt, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = solusdtorderbook.best_bid().unwrap();
                                graph.update_edge(&usdtsol, one/bidprice, bidquantity);
                            }
                            "BTC/USD" => {
                                btcusdorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = btcusdorderbook.best_ask().unwrap();
                                graph.update_edge(&btcusd, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = btcusdorderbook.best_bid().unwrap();
                                graph.update_edge(&usdbtc, one/bidprice, bidquantity);
                            }
                            "USDT/USD" => {
                                usdtusdorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = usdtusdorderbook.best_ask().unwrap();
                                graph.update_edge(&usdtusd, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = usdtusdorderbook.best_bid().unwrap();
                                graph.update_edge(&usdusdt, one/bidprice, bidquantity);
                            }
                            "FTT/BTC" => {
                                fttbtcorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = fttbtcorderbook.best_ask().unwrap();
                                graph.update_edge(&fttbtc, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = fttbtcorderbook.best_bid().unwrap();
                                graph.update_edge(&btcftt, one/bidprice, bidquantity);
                            }
                            "FTT/USDT" => {
                                fttusdtorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = fttusdtorderbook.best_ask().unwrap();
                                graph.update_edge(&fttusdt, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = fttusdtorderbook.best_bid().unwrap();
                                graph.update_edge(&usdtftt, one/bidprice, bidquantity);
                            }
                            "FTT/USD" => {
                                fttusdorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = fttusdorderbook.best_ask().unwrap();
                                graph.update_edge(&fttusd, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = fttusdorderbook.best_bid().unwrap();
                                graph.update_edge(&usdftt, one/bidprice, bidquantity);
                            }
                            "SOL/USD" => {
                                solusdorderbook.update(&orderbook_data);
                                let (askprice, askquantity) = solusdorderbook.best_ask().unwrap();
                                graph.update_edge(&solusd, askprice, askquantity * askprice);
                                let (bidprice, bidquantity) = solusdorderbook.best_bid().unwrap();
                                graph.update_edge(&usdsol, one/bidprice, bidquantity);
                            }
                            _ => panic!("unexpected symbol"),

                        }
                    },
                    None => panic!("No symbol received"),
                }
            },
            _ => panic!("Unexpected data received: {:?}", obdata)
        }

        let usd = "USD".to_string();
        let (profitable, path, amounts) = find_arbitrage(&graph, &usd);

        if profitable {
            println!("Arbitrage found: {:?}", path);
            println!("Amounts: {:?}", amounts);
        }
    }


    Ok(())
}
