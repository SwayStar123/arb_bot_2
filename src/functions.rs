use rust_decimal::{Decimal, prelude::FromPrimitive};
use crate::structs::{Graph, Pair};
use std::cmp::min;

pub fn find_arbitrage(graph: &Graph, start: &String) -> (bool, Vec<Pair>, Vec<Decimal>){
    //convinience variable for one in decimal
    let mut graph = graph.clone();
    let one = Decimal::from(1_u8);

    //the series of pairs to trade
    let mut predecessors = Vec::new();
    //the amounts to trade in each pair
    let mut amounts = Vec::new();

    let mut profitable = false;

    //makes sure the node amounts are 0
    graph.reset();
    //sets the start node to 1
    graph.update_vertex(start, one);

    //for every connected node to start, check if trade is profitable
    'outer: for coin in graph.connections[start].clone().iter() {
        //saving state of graph
        let graph_c1 = graph.clone();

        //simulate trade
        let p1 = graph.get_pair(start.to_string(), coin.to_string()).clone();
        graph.traverse_edge(&p1);
        //append trade to list of trades
        // predecessors.push(format!("{}/{}", start, coin));

        let weight1 = graph.get_price(&p1);
        let min1 = graph.get_amount(&p1);

        //append trade to list of trades
        predecessors.push(p1);

        //for every connected node to coin2 (coin1's successor), check if trade is profitable
        for coin2 in graph.connections[coin].clone().iter() {
            //saving state of graph
            let graph_c2 = graph.clone();

            //simulate trade
            let p2 = graph.get_pair(coin.to_string(), coin2.to_string()).clone();
            graph.traverse_edge(&p2);
            //append trade to list of trades
            // predecessors.push(format!("{}/{}", coin, coin2));

            let weight2 = graph.get_price(&p2);
            let min2 = min(graph.get_amount(&p2), min1 * weight2);

            //append trade to list of trades
            predecessors.push(p2);

            //if starting amount is greater than one, IE profitable, then break the loops
            if graph.vertices[start] > one {
                profitable = true;
                amounts.append(&mut vec![
                    min1,
                    quantity_in(min2, weight1),
                ]);
                break 'outer;
            }

            //for every connected node to coin3 (coin2's successor), check if trade is profitable
            for coin3 in graph.connections[coin2].clone().iter() {
                //saving state of graph
                let graph_c3 = graph.clone();

                //simulate trade
                let p3 = graph.get_pair(coin2.to_string(), coin3.to_string()).clone();
                graph.traverse_edge(&p3);
                //append trade to list of trades
                // predecessors.push(format!("{}/{}", coin2, coin3));

                let weight3 = graph.get_price(&p3);
                let min3 = min(graph.get_amount(&p3), min2 * weight3);

                //append trade to list of trades
                predecessors.push(p3);

                //if starting amount is greater than one, IE profitable, then break the loops
                if graph.vertices[start] > one {
                    profitable = true;
                    amounts.append(&mut vec![
                        min1,
                        quantity_in(min2, weight1),
                        quantity_in(quantity_in(min3, weight2), weight1),
                    ]);
                    break 'outer;
                }

                //for every connected node to coin4 (coin3's successor), check if trade is profitable
                for coin4 in graph.connections[coin3].clone().iter() {
                    //saving state of graph
                    let graph_c4 = graph.clone();

                    //simulate trade
                    let p4 = graph.get_pair(coin3.to_string(), coin4.to_string()).clone();
                    graph.traverse_edge(&p4);
                    //append trade to list of trades
                    // predecessors.push(format!("{}/{}", coin3, coin4));

                    let weight4 = graph.get_price(&p4);
                    let min4 = min(graph.get_amount(&p4), min3 * weight4);

                    //append trade to list of trades
                    predecessors.push(p4);

                    //if starting amount is greater than one, IE profitable, then break the loops
                    if graph.vertices[start] > one {
                        profitable = true;
                        amounts.append(&mut vec![
                            min1,
                            quantity_in(min2, weight1),
                            quantity_in(quantity_in(min3, weight2), weight1),
                            quantity_in(quantity_in(quantity_in(min4, weight3), weight2), weight1),
                        ]);
                        break 'outer;
                    }

                    //if didnt break, ie not profitable, revert the changes, by reassigning to saved state and removing the appended trade
                    graph = graph_c4;
                    predecessors.pop();
                }

                //if didnt break, ie not profitable, revert the changes, by reassigning to saved state and removing the appended trade
                graph = graph_c3;
                predecessors.pop();
            }

            //if didnt break, ie not profitable, revert the changes, by reassigning to saved state and removing the appended trade
            graph = graph_c2;
            predecessors.pop();
        }

        //if didnt break, ie not profitable, revert the changes, by reassigning to saved state and removing the appended trade
        graph = graph_c1;
        predecessors.pop();
    }

    // println!("{:?}", predecessors);
    // println!("{:?}", amounts);
    //return
    (profitable, predecessors, amounts)
}

pub fn quantity_in(original_qt: Decimal, conversion_price: Decimal) -> Decimal {
    let x = original_qt / conversion_price;

    x
}

pub fn fees(amount: Decimal) -> Decimal {
    //order fees are in the quote currency, regardless of whether you are buying or selling. Fees are   0.07%
    //returns the total after fee
    //converting 1 btc to 10,000 usd will have a fee of 7 usd return 1 - 0.07% = 0.9993
    //converting 10000 usd to 1 btc will have a fee of 7 usd return 10000 - 0.07% = 9993

    let fee = Decimal::from_f64(0.07).unwrap();
    let fee_amount = amount * fee;

    amount - fee_amount
}
