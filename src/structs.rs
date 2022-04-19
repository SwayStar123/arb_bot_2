use std::collections::HashMap;
use rust_decimal::{Decimal, prelude::Zero};
use crate::functions::fees;
    

#[derive(Clone)]
pub struct Graph {
    pub vertices: HashMap<String, Decimal>,
    pub connections: HashMap<String, Vec<String>>,
    pub pairs: Vec<Pair>,
    pub edges: HashMap<Pair, (Decimal, Decimal)>,
}

impl Graph {
    pub fn new () -> Self {
        Graph {
            vertices: HashMap::new(),
            connections: HashMap::new(),
            edges: HashMap::new(),
            pairs: Vec::new(),
        }
    }

    pub fn add_pair(&mut self, pair: Pair) {
        self.vertices.insert(pair.to.clone(), Decimal::zero());
        self.vertices.insert(pair.from.clone(), Decimal::zero());
        
        self.connections.entry(pair.from.clone()).or_insert(Vec::new()).push(pair.to.clone());
        self.connections.entry(pair.to.clone()).or_insert(Vec::new()).push(pair.from.clone());

        self.edges.insert(pair.clone(), (Decimal::zero(), Decimal::zero()));

        self.pairs.push(pair);
    }

    pub fn add_vertex(&mut self, vertex: &String) {
        self.vertices.insert(vertex.to_string(), Decimal::zero());
    }

    pub fn update_vertex(&mut self, vertex: &String, balance: Decimal) {
        self.vertices.insert(vertex.to_string(), balance);
    }

    pub fn add_edge(&mut self, pair: &Pair) {
        let from = pair.from.clone();
        let to = pair.to.clone();

        self.connections.entry(from.to_string()).or_insert(Vec::new()).push(to.to_string());
        self.connections.entry(to.to_string()).or_insert(Vec::new()).push(from.to_string());
        
        self.edges.insert(pair.clone(), (Decimal::zero(), Decimal::zero()));
    }

    pub fn update_edge(&mut self, pair: &Pair, weight: Decimal, amount: Decimal) {
        let map = self.edges.get_mut(pair).unwrap();
        map.0 = weight;
        map.1 = amount;
    }

    pub fn reset(&mut self) {
        for mut vertex in self.vertices.iter_mut() {
            vertex.1 = &mut Decimal::zero();
        }
    }

    pub fn traverse_edge(&mut self, pair: &Pair) {
        let from = &pair.from;
        let to = &pair.to;

        let weight = self.edges.get(pair).unwrap().0;
        let amount = fees(self.vertices[from]) * weight;

        self.vertices.insert(to.to_string(), amount);
        self.vertices.insert(from.to_string(), Decimal::zero());
    }
    
    pub fn get_amount(&self, pair: &Pair) -> Decimal {
        let amount = self.edges[&pair].1;
        
        amount
    }

    pub fn get_price(&self, pair: &Pair) -> Decimal {
        let weight = self.edges[&pair].0;
        
        weight
    }

    pub fn get_pair(&self, from: String, to: String) -> &Pair {
        let pair = self.pairs.iter().find(|pair| {
            pair.from == from && pair.to == to
        }).unwrap();

        pair
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Pair {
    pub from: String,
    pub to: String,
    pub to_quote: bool,
}

impl Pair {
    pub fn new(from: &str, to: &str, to_quote: bool) -> Self {
        Pair {
            from: from.to_string(),
            to: to.to_string(),
            to_quote,
        }
    }

    pub fn get_symbol(&self) -> String {
        if self.to_quote {
            format!("{}/{}", self.from, self.to)
        } else {
            format!("{}/{}", self.to, self.from)
        }
        
    }
}

