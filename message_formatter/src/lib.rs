use serde::{Serialize, Deserialize};
use serde_json::to_string;

pub enum Operation {
    TraderCapital(f32, f32, f32, f32),  // eur, usd, yuan, yen
    SellBuyRates(Vec<(String, f32, f32, f32, f32)>), // market_name, eur, usd, yuan, yen
    SellBuyTransaction(SellBuy, String, Currency, f32), // type of operation, market name, currency, amount
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
struct TraderCapitalStruct {
    eur: f32,
    usd: f32,
    yuan: f32,
    yen: f32,
}

#[derive(Serialize, Deserialize)]
struct SellBuyRatesStruct {
    name: String,
    eur: f32,
    usd: f32,
    yen: f32,
    yuan: f32,
}

#[derive(Serialize, Deserialize)]
struct SellBuyRatesVec {
    v: Vec<SellBuyRatesStruct>,
}

#[derive(Serialize, Deserialize)]
struct SellBuyTransactionStruct {
    transaction: SellBuy,
    name: String,
    currency: Currency,
    amount: f32,
}

#[derive(Serialize, Deserialize)]
struct OtherStruct {
    s: String,
}


fn create_trader_capital(eur: f32, usd: f32, yuan: f32, yen: f32) {
    let structure = TraderCapitalStruct {
        eur,
        usd,
        yuan,
        yen
    };
    let s = serde_json::to_string(&structure);
    match s {
        Ok(s) => println!("TraderCapital-{}", s),
        _ => {}
    }
}

fn create_sell_buy_rates(name: String, eur: f32, usd: f32, yuan: f32, yen: f32) -> SellBuyRatesStruct {
    SellBuyRatesStruct {
        name,
        eur,
        usd,
        yuan,
        yen
    }
}

fn create_sell_buy_rates_vec(v_tuple: Vec<(String, f32, f32, f32, f32)>) {
    let mut structure = SellBuyRatesVec {
        v: Vec::new(),
    };
    for tuple in v_tuple {
        let st = create_sell_buy_rates(tuple.0, tuple.1, tuple.2, tuple.3, tuple.4);
        structure.v.push(st);
    }

    let s = serde_json::to_string(&structure);
    match s {
        Ok(s) => println!("SellBuyRates-{}", s),
        _ => {}
    }
}

fn create_sell_buy_transaction(transaction: SellBuy, name: String, currency: Currency, amount: f32) {
    let structure = SellBuyTransactionStruct {
        transaction,
        name,
        currency,
        amount
    };
    let s = serde_json::to_string(&structure);
    match s {
        Ok(s) => println!("SellBuyTransaction-{}", s),
        _ => {}
    }
}

fn create_other(s: String) {
    let structure = OtherStruct {
        s,
    };
    let s = serde_json::to_string(&structure);
    match s {
        Ok(s) => println!("Other-{}", s),
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


