use pwm_dev::*;
use std::{thread::sleep, time::Duration};

fn main() {
    #[cfg(not(feature = "test"))]
    let pwm = {
        let pwm = rppal::pwm::Pwm::new(rppal::pwm::Channel::Pwm0).expect("failed to get PWM");
        pwm.enable().expect("failed to enable pwm");
        // pwm.set_polarity(rppal::pwm::Polarity::Normal)
        // .expect("failed to set polarity");
        // println!("Pin is enabled? {}", pwm.is_enabled().unwrap());
        pwm
        // rppal::gpio::Gpio::new()
        //     .expect("failed to get GPIO")
        //     .get(19)
        //     .expect("failed to get pin")
        //     .into_output()
    };
    let in_three_minutes = chrono::Local::now().time() + chrono::Duration::minutes(3);
    let transition = Transition {
        from: Strength::new(0.0),
        to: Strength::new(1.0),
        time: Duration::from_secs(2),
        interpolation: TransitionInterpolation::Sine,
    };
    let scheduler = scheduler::WeekScheduler::same(in_three_minutes, transition.clone());
    #[cfg(feature = "test")]
    let mut controller = Controller::new(PrintOut, scheduler);
    #[cfg(not(feature = "test"))]
    let mut controller = Controller::new(pwm, scheduler);

    controller.send(Command::Set(Strength::new(0.75)));
    sleep(Duration::from_secs(2));

    println!("Sending transition!");
    controller.send(Command::SetTransition(transition));
    sleep(Duration::from_secs(1));

    println!("Sending set command");
    controller.send(Command::Set(Strength::new(0.25)));

    sleep(Duration::from_secs(3));

    controller.finish();
}
