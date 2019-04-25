#![feature(async_await, await_macro)]

use deribit::models::{AuthRequest, BuyRequest, GetOrderStateRequest, SellRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use failure::{Error, Fallible};
use fluid::prelude::*;
use futures::compat::Future01CompatExt;
use futures::{FutureExt, TryFutureExt};
use std::env::var;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tokio::timer::Delay;


struct TradingTest;

impl Default for TradingTest {
    fn default() -> Self {
        let _ = dotenv();
        TradingTest
    }
}

#[session]
impl TradingTest {
    #[fact]
    fn get_order_state(self) -> Fallible<()> {
        let key = var("DERIBIT_KEY").unwrap();
        let secret = var("DERIBIT_SECRET").unwrap();

        let drb = DeribitBuilder::default().testnet(true).build().unwrap();
        let mut rt = Runtime::new()?;

        let fut = async move {
            let (mut client, _) = await!(drb.connect())?;
            let req = AuthRequest::credential_auth(&key, &secret);
            let _ = await!(await!(client.call(req))?)?;

            let req = GetOrderStateRequest::new("2260120935");
            Ok::<_, Error>(await!(await!(client.call(req))?)?)
        };

        let fut = fut.boxed().compat();
        let _ = rt.block_on(fut)?;
        Ok(())
    }

    #[fact]
    fn buy_and_sell(self) -> Fallible<()> {
        let _ = dotenv();

        let key = var("DERIBIT_KEY").unwrap();
        let secret = var("DERIBIT_SECRET").unwrap();
        let drb = DeribitBuilder::default().testnet(true).build().unwrap();
        let mut rt = Runtime::new()?;

        let fut = async move {
            let (mut client, _) = await!(drb.connect())?;
            let req = AuthRequest::credential_auth(&key, &secret);
            let _ = await!(await!(client.call(req))?)?;

            await!(await!(
                client.call(BuyRequest::market("BTC-PERPETUAL", 10.))
            )?)?;
            await!(Delay::new(Instant::now() + Duration::from_secs(1)).compat())?;
            await!(await!(
                client.call(SellRequest::market("BTC-PERPETUAL", 10.))
            )?)?;
            Ok::<_, Error>(())
        };

        let fut = fut.boxed().compat();
        let _ = rt.block_on(fut)?;
        Ok(())
    }
}