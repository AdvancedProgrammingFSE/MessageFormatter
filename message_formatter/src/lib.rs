use serde::{Serialize, Deserialize};
use serde_json::to_string;

pub enum Operation {
    TraderCapital(f64, f64, f64, f64),  // eur, usd, yuan, yen
    SellBuyRates(Vec<(String, (f64, f64), (f64, f64), (f64, f64), (f64, f64))>), // market_name, eur, usd, yuan, yen
    SellBuyTransaction(SellBuy, String, Currency, f64), // type of operation, market name, currency, amount
    Other(String),
}

#[derive(Serialize, Deserialize)]
pub enum SellBuy {
    Sell,
    Buy,
}

#[derive(Serialize, Deserialize)]
pub enum Currency {
    EUR,
    USD,
    YEN,
    YUAN
}

#[derive(Serialize, Deserialize)]
pub struct TraderCapitalStruct {
    pub eur: f64,
    pub usd: f64,
    pub yuan: f64,
    pub yen: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SellBuyRatesStruct {
    pub name: String,
    //sell,buy
    pub eur: (f64, f64),
    pub usd: (f64, f64),
    pub yen: (f64, f64),
    pub yuan: (f64, f64),
}

#[derive(Serialize, Deserialize)]
pub struct SellBuyRatesVec {
    pub v: Vec<SellBuyRatesStruct>,
}

#[derive(Serialize, Deserialize)]
pub struct SellBuyTransactionStruct {
    pub transaction: SellBuy,
    pub name: String,
    pub currency: Currency,
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
struct OtherStruct {
    pub s: String,
}


pub fn create_trader_capital(eur: f64, usd: f64, yuan: f64, yen: f64) {
    let structure = TraderCapitalStruct {
        eur,
        usd,
        yuan,
        yen
    };
    let s = to_string(&structure);
    match s {
        Ok(s) => println!("{}", s),
        _ => {}
    }
}

pub fn create_sell_buy_rates(name: String, eur: (f64, f64), usd: (f64, f64), yuan: (f64, f64), yen: (f64, f64)) -> SellBuyRatesStruct {
    SellBuyRatesStruct {
        name,
        eur,
        usd,
        yuan,
        yen
    }
}

pub fn create_sell_buy_rates_vec(v_tuple: Vec<(String, (f64, f64), (f64, f64), (f64, f64), (f64, f64))>) {
    let mut structure = SellBuyRatesVec {
        v: Vec::new(),
    };
    for tuple in v_tuple {
        let st = create_sell_buy_rates(tuple.0, tuple.1, tuple.2, tuple.3, tuple.4);
        structure.v.push(st);
    }

    let s = to_string(&structure);
    match s {
        Ok(s) => println!("{}", s),
        _ => {}
    }
}

pub fn create_sell_buy_transaction(transaction: SellBuy, name: String, currency: Currency, amount: f64) {
    let structure = SellBuyTransactionStruct {
        transaction,
        name,
        currency,
        amount
    };
    let s = to_string(&structure);
    match s {
        Ok(s) => println!("{}", s),
        _ => {}
    }
}

fn create_other(s: String) {
    let structure = OtherStruct {
        s,
    };
    let s = to_string(&structure);
    match s {
        Ok(s) => println!("{}", s),
        _ => {}
    }
}
pub fn send_message(op: Operation) {
    match op {
        Operation::TraderCapital(eur, usd, yuan, yen) => create_trader_capital(eur, usd, yuan, yen),
        Operation::SellBuyRates(v) => create_sell_buy_rates_vec(v),
        Operation::SellBuyTransaction(transaction, name, currency, amount) => create_sell_buy_transaction(transaction, name, currency, amount),
        Operation::Other(s) => create_other(s),
    }
}


