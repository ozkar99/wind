use cm_telemetry::f1::f1_2020::F1_2020;
use cm_telemetry::TelemetryServer;

fn main() {
    let mut conn = gpioduino::Conn::new("COM3").expect("failed to open serial port");
    conn.dtr(true).expect("failed to set DTR");

    let output_pin = 13;
    conn.pin_mode(output_pin, gpioduino::PinMode::Output)
        .expect("failed to set pin to OUTPUT mode");

    let server =
        TelemetryServer::<F1_2020>::new("127.0.0.1:20777").expect("failed to bind to address");

    loop {
        let event = server.next();

        if let Err(e) = event {
            println!("error: {:?}", e);
            continue;
        }

        match event.unwrap() {
            F1_2020::CarTelemetry(data) => {
                let speed = data.player_data().speed;
                let pwm = normalize(speed);
                
                if let Err(_) = conn.analog_write(output_pin, pwm) {
                    println!("Errored out trying to write speed to arduino");
                }
                println!("speed: {}, {}", speed, pwm);
            }
            _ => {} // do nothing
        }
    }
}

/// normalize speed into PWM rate
/// speed goes from 0 - 350
/// pwm goes from 0 - 255
fn normalize(speed: u16) -> u8 {
    let val = (speed as f32 / 350.0) * 255.0;
    val.ceil() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizetest() {
        assert_eq!(normalize(0), 0);
        assert_eq!(normalize(350), 255);
        assert_eq!(normalize(175), 128);
    }
}
