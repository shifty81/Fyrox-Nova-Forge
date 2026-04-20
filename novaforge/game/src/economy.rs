//! Economy and market system for NovaForge.
//!
//! Implements buy/sell order books, broker fees, and item trading. The market
//! is station-local and inspired by EVE Online's player-driven market.

use fyrox::core::reflect::prelude::*;
use fyrox::core::visitor::prelude::*;
use std::collections::HashMap;

/// Unique identifier for a market item / commodity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Visit, Reflect)]
pub struct ItemType(pub String);

impl ItemType {
    pub fn new(name: impl Into<String>) -> Self {
        ItemType(name.into())
    }
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType("Unknown".into())
    }
}

/// Direction of a market order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Visit, Reflect)]
pub enum OrderSide {
    /// Buyer wants to purchase at or below `price_per_unit`.
    Buy,
    /// Seller wants to sell at or above `price_per_unit`.
    Sell,
}

impl Default for OrderSide {
    fn default() -> Self {
        OrderSide::Sell
    }
}

/// A single open market order.
#[derive(Debug, Clone, Visit, Reflect)]
pub struct MarketOrder {
    pub order_id: u64,
    pub owner: String,
    pub item: ItemType,
    pub side: OrderSide,
    pub price_per_unit: f64,
    pub quantity: u64,
    pub station: String,
}

impl Default for MarketOrder {
    fn default() -> Self {
        MarketOrder {
            order_id: 0,
            owner: String::new(),
            item: ItemType::default(),
            side: OrderSide::Sell,
            price_per_unit: 0.0,
            quantity: 0,
            station: String::new(),
        }
    }
}

/// Station-local order book.
#[derive(Debug, Default, Clone, Visit, Reflect)]
pub struct OrderBook {
    orders: Vec<MarketOrder>,
    next_id: u64,
}

impl OrderBook {
    /// Posts a new order and returns its assigned ID.
    pub fn post(&mut self, mut order: MarketOrder) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        order.order_id = id;
        self.orders.push(order);
        id
    }

    /// Cancels the order with the given ID. Returns `true` if found.
    pub fn cancel(&mut self, order_id: u64) -> bool {
        let before = self.orders.len();
        self.orders.retain(|o| o.order_id != order_id);
        self.orders.len() < before
    }

    /// Returns all sell orders for an item at the given station, cheapest first.
    pub fn best_sells(&self, item: &ItemType, station: &str) -> Vec<&MarketOrder> {
        let mut orders: Vec<&MarketOrder> = self
            .orders
            .iter()
            .filter(|o| o.side == OrderSide::Sell && &o.item == item && o.station == station)
            .collect();
        orders.sort_by(|a, b| a.price_per_unit.partial_cmp(&b.price_per_unit).unwrap());
        orders
    }

    /// Returns all buy orders for an item at the given station, highest first.
    pub fn best_buys(&self, item: &ItemType, station: &str) -> Vec<&MarketOrder> {
        let mut orders: Vec<&MarketOrder> = self
            .orders
            .iter()
            .filter(|o| o.side == OrderSide::Buy && &o.item == item && o.station == station)
            .collect();
        orders.sort_by(|a, b| b.price_per_unit.partial_cmp(&a.price_per_unit).unwrap());
        orders
    }

    /// Attempts an immediate buy: consumes from cheapest sell orders.
    /// Returns the total ISK spent and units actually purchased.
    pub fn immediate_buy(
        &mut self,
        item: &ItemType,
        station: &str,
        mut quantity: u64,
        max_price: f64,
    ) -> (f64, u64) {
        let mut isk_spent = 0.0;
        let mut units_bought = 0u64;

        let matching: Vec<u64> = self
            .orders
            .iter()
            .filter(|o| {
                o.side == OrderSide::Sell
                    && &o.item == item
                    && o.station == station
                    && o.price_per_unit <= max_price
            })
            .map(|o| o.order_id)
            .collect();

        for id in matching {
            if quantity == 0 {
                break;
            }
            if let Some(order) = self.orders.iter_mut().find(|o| o.order_id == id) {
                let can_buy = order.quantity.min(quantity);
                isk_spent += can_buy as f64 * order.price_per_unit;
                units_bought += can_buy;
                quantity -= can_buy;
                order.quantity -= can_buy;
            }
        }

        // Remove fully fulfilled orders
        self.orders.retain(|o| o.quantity > 0);

        (isk_spent, units_bought)
    }
}

/// Player wallet.
#[derive(Debug, Clone, Visit, Reflect)]
pub struct Wallet {
    pub isk: f64,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet { isk: 50_000.0 }
    }
}

impl Wallet {
    pub fn credit(&mut self, amount: f64) {
        self.isk += amount;
    }

    /// Debits ISK. Returns `false` if the wallet has insufficient funds.
    pub fn debit(&mut self, amount: f64) -> bool {
        if self.isk >= amount {
            self.isk -= amount;
            true
        } else {
            false
        }
    }
}

/// Player inventory: quantities of each item type held in cargo.
#[derive(Debug, Clone, Default, Visit, Reflect)]
pub struct Inventory {
    items: HashMap<String, u64>,
}

impl Inventory {
    pub fn quantity(&self, item: &ItemType) -> u64 {
        *self.items.get(&item.0).unwrap_or(&0)
    }

    pub fn add(&mut self, item: &ItemType, qty: u64) {
        *self.items.entry(item.0.clone()).or_insert(0) += qty;
    }

    /// Removes items from inventory. Returns `false` if there are not enough.
    pub fn remove(&mut self, item: &ItemType, qty: u64) -> bool {
        let current = self.quantity(item);
        if current >= qty {
            let entry = self.items.get_mut(&item.0).unwrap();
            *entry -= qty;
            if *entry == 0 {
                self.items.remove(&item.0);
            }
            true
        } else {
            false
        }
    }
}
