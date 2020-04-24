#![no_std]
#![no_main]

#[macro_use]
extern crate ruspiro_boot;
extern crate ruspiro_allocator;
use ruspiro_gpio::{*, hal::*};
use ruspiro_interrupt::*;
//use ruspiro_talents::*;

extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use ruspiro_singleton::*;

//struct GpioCapability(Box<dyn Gpio>);

//impl capability::Capability for GpioCapability {}

come_alive_with!(alive);
run_with!(run);

fn alive(_: u32) {
    //let mut talents = Talents::default();
    let gpio = Arc::new(
        Singleton::new(
            Gpio::new().unwrap()
        )
    );
    let gpio_clone = gpio.clone();

    gpio.take_for(|gpio| {
        let in_pin = gpio.use_pin(17).unwrap().into_input();
        //let out_pin2 = Arc::new(gpio.use_pin(19).unwrap().into_output());
        gpio.register_event_handler_always(
            &*in_pin,
            GpioEvent::High,
            Box::new(
                move || {
                    //out_pin2.toggle();
                    gpio_clone.take_for(|gpio| {
                        let out_pin = gpio.use_pin(18).unwrap().into_output();
                        out_pin.toggle();
                        gpio.release_pin(18);
                    });
                }
            )
        );
    });

    // enable interrupts for GPIO and globally activate them
    IRQ_MANAGER.take_for(|irq| irq.activate(Interrupt::GpioBank0));
    enable_interrupts();




    //pin.high();
    //gpio.release_pin(17);

    /*
    talents.add_capability(GpioCapability(Box::new(gpio)));

    talents.use_capability_mut::<GpioCapability, _, _>(|gpio| {
        let pin = gpio.0.use_pin(17).unwrap().into_output();
        pin.high();
        gpio.0.release_pin(17);
    }).unwrap();
    */
}

fn run(_: u32) -> ! {
    loop {}
}