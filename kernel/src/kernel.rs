/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/
//! # RusPiRo Kernel
//!
//! This is the kernel of the RusPiRo "firmware". It provides the runtime environment for the Raspberry Pi to execute
//! custom functionality. The kernel provides the bare minimum environment to allow modular growth of features the robot
//! will eventually provide.
//!

#![doc(html_root_url = "https://docs.rs/ruspiro-kernel/0.1.0")]
#![cfg_attr(not(any(test, doctest)), no_std)]
#![feature(asm)]
#![no_main]

// need to use extern crate ruspiro_allocator to get the allocator into scope for
// successfull compilation and linking
extern crate alloc;
extern crate ruspiro_allocator;
use alloc::sync::Arc;
use ruspiro_boot::{come_alive_with, run_with};
use ruspiro_brain::*;
use ruspiro_core::{lock::DataLock, singleton::Singleton, *};
use ruspiro_gpio::*;
use ruspiro_interrupt::*;
use ruspiro_mailbox::*;
use ruspiro_talents::*;
use ruspiro_uart::*;

mod talentthought;

/// The context that the kernel is using accross the different cores
struct KernelContext {
    /// The [Talents] the roboter is able to use are managed with this.
    talents: Arc<DataLock<Talents>>,
    /// The [Brain] brings the executing runtime to enable thinking on different [Thought]'s
    brain: Arc<Brain>,
}

/// The static context shared accross the cores of the Raspberry Pi. Wrapped into a [Singleton] to ensure
/// safe mutual exclusive access if required. The context itself is further wrapped into an [Option] as the
/// instantiation of the context is not able to be represented as a `const fn` function. The value of [None]
/// represents the un-initialized context.
static CONTEXT: Singleton<Option<KernelContext>> = Singleton::new(None);

#[cfg(not(any(test, doctest)))]
come_alive_with!(alive);

#[cfg(not(any(test, doctest)))]
run_with!(run);

/// This function is called once on each core after the boot sequence has been completed
/// and the core is ready to take over some processing. This allows for the additional
/// initial setup to be performed to get the RusPiRo kernel into the state where it is able to do
/// some useful stuff. Only after the function has retunred from one core the next one will
/// be kicked off.
fn alive(core: u32) {
    info!("alive... at {}", core);
    // the cores are kicked off in sequence so who ever comes first shall initialize the kernel context
    CONTEXT.take_for(|maybe_context: &mut Option<KernelContext>| {
        if maybe_context.is_none() {
            info!("initialize RusPiRo context...");
            // initializing the kernel context starts with preparing the talents
            let mut talents = Talents::default();
            info!("initialize built-in talents...");
            // TODO: add the built-in talents coming with the kernel
            //       this could be the GPIO, I²C, UART or miniUART
            //       - check as miniUART is used by console and GPIO is currently
            //       provided as a Singleton as part of GPIO crate
            //talents.add_capability(&GPIO);
            // first capability is the CPU<->GPU mailbox that is to be added
            let mut mailbox = Mailbox::default();
            // retrieve the core clock rate before adding the capability
            let core_clock_rate = mailbox.get_clockrate(ClockId::Core).unwrap_or(250_000_000);
            talents.add_capability(mailbox);

            // setting up the miniUART here again is quite likely to break the CONSOLE implementation
            // used to provide println!, info! etc. output macros
            let mut uart = uart1::Uart1::new();
            // not beeing able to initialize the miniUART is fatal and should panic any further processing
            uart.initialize(core_clock_rate, 115_200); //.expect("unable to initialize the miniUART");

            // the talents capabilities now owns the uart - access only via the talents
            // TODO: how to easily provide logging functionality then, re-using this miniUART ?
            talents.add_capability(uart);

            // next step is preparing the brain that will do the processing of all thoughts
            info!("initialize the brain...");
            let brain = Brain::default();

            // wrap the talents into an shareable Arc and secure it with a DataLock to ensure
            // safe mutual exclusive access in case it need to be mutated
            let shareable_talents = Arc::new(DataLock::new(talents));

            // once the brain is prepared we will spawn the initial thought that is chaking for a new talent
            // being uploaded to the roboter. This initial thought will typically never really come to a conclusion
            // as it never knows when no further talents might be uploaded
            brain.spawn(talentthought::TalentThought::new(shareable_talents.clone()));

            // now set the initialized context into the cross core shared static to allow the other cores to
            // see and access the global context
            maybe_context.replace(KernelContext {
                talents: shareable_talents,
                brain: Arc::new(brain),
            });

            enable_interrupts();
        };
    });
    // leaving here will kick-off the next core and start this cores "run" function
}

/// This function is called once on each core and is intended to never return
fn run(core: u32) -> ! {
    // kick-off thinking on the brain on each core
    loop {
        CONTEXT.use_for(|maybe_context: &Option<KernelContext>| {
            if let Some(context) = maybe_context {
                context.brain.think();
            }
        });
        // if this core has finished thinking let him sleep until an event
        // occurs that allows thinking on woken thoughts, this would also reduce
        // energy consumption for this core. Any interrupt or calls to "SEV" would wake this core
        // to continue processing
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        unsafe {
            //info!("core {} is going to sleep.", core);
            asm!("wfe");
            //info!("core {} woken by an event.", core);
        }
    }
}

/*
extern crate alloc;
fn init(context: &Context) {
    info!("call init...");
    unsafe { __init(context) };
}

extern "Rust" {
    fn __init(context: &Context);
}
*/
