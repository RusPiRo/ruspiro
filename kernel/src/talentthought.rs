/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/
//! # Talent Thought
//! This initial thought allows the brain to wait for new talents to be uploaded to the roboter

extern crate core;
use alloc::sync::Arc;
use core::pin::Pin;
use ruspiro_brain::*;
use ruspiro_core::{lock::DataLock, *};
use ruspiro_talents::*;
use ruspiro_uart::{uart1::Uart1, InterruptType};

pub struct TalentThought {
    talents: Arc<DataLock<Talents>>,
}

impl TalentThought {
    pub fn new(talents: Arc<DataLock<Talents>>) -> Self {
        Self { talents }
    }
}

impl Thinkable for TalentThought {
    type Output = ();

    fn think(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Conclusion<Self::Output> {
        info!("think on TalentThought");
        // if we are thinking on this the first time there its quite unlikely data arriving
        // already at the miniUART. So just register our waker with the miniUART interrupt
        // handler to get woken in case of data arrives
        self.talents
            .read()
            .use_capability_mut::<Uart1, _>(|uart: &mut Uart1| {
                let waker = cx.waker().clone();
                uart.register_interrupt_handler(InterruptType::Receive, move || {
                    waker.wake();
                });
                Ok(())
            });

        Conclusion::Pending
    }
}
