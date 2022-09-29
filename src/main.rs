use std::thread;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::io::Write;
use std::net::TcpStream;
use sdl2::mouse::MouseButton;
use clap::Parser;

struct RemoteEventClient{
    client: Option<TcpStream>
}

impl RemoteEventClient {
    fn new(address: String, port: String) -> Self{
        match TcpStream::connect(format!("{}:{}", address, port)) {
            Ok(c) => {
                return Self{ client: Some(c) }
            },
            Err(_) => {
                // println!("Failed to connect to: {}. Running in echo only mode.", address);
                return Self{ client: None }
            }
        };
    }

    fn send_data(&mut self, protocol: &str, msg: &str){
        match &self.client.as_ref() {
            Some(mut c) => {
                let data = format!("{}|{}", protocol, msg);
                c.write_all(format!("{}\n", data).as_bytes()).unwrap();
            },
            None => {
                 // println!("{}|{}", protocol, msg);
            }
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about = "A simple client implementation for SensorHandler.\nhttps://github.com/OwczarekGit/sensorhandler", long_about = None)]
// #[clap(author, version, about)]
struct Args{
    /// Server ip address.
    address: String,

    /// The server port.
    #[clap(short = 'p', long = "port", default_value = "2137")]
    port: String,

    /// Mouse speed.
    #[clap(short = 's', long = "speed", default_value = "0.06")]
    speed: f32,

    /// The event polling rate in ms.
    #[clap(short = 'r', long = "rate", default_value = "1")]
    rate: u64,

    /// Should mouse events be handled.
    #[clap(short = 'm', long = "mouse", takes_value = false)]
    mouse: bool,

}

fn main() {
    let mut key_state_osu = 0x00;
    let mut key_state_keyboard = 0x00u128;
    let mut mouse_state = 0x00;

    let args = Args::parse();
    let mouse_speed = args.speed;
    let server_ip = args.address;
    let port = args.port;
    let poll_rate = args.rate;
    let mouse_enabled = args.mouse;

    let mut connection = RemoteEventClient::new(server_ip.clone(), port);

    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("SensorHandler Client", 400, 200)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .software()
        .build()
        .unwrap();

    context.mouse().set_relative_mouse_mode(true);
    context.mouse().show_cursor(true);

    let mut events = context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(44,44,44));
    canvas.clear();
    canvas.present();
    update_title(canvas.window_mut(), &server_ip, poll_rate as f32);

    'running: loop {

        let mut should_send_osu_data = false;
        let mut should_send_mouse_data = false;
        let mut should_send_keyboard_data = false;
        let mut delta_mouse_wheel = 0;

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Q), .. } => break 'running,

                // Z
                Event::KeyDown { keycode: Some(Keycode::Z), repeat: false, .. } => {
                    key_state_osu |= 0b0000_0001;
                    should_send_osu_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::Z), repeat: false, .. } => {
                    key_state_osu &= 0b1111_1110;
                    should_send_osu_data = true;
                },

                // X
                Event::KeyDown { keycode: Some(Keycode::X), repeat: false, .. } => {
                    key_state_osu |= 0b0000_0010;
                    should_send_osu_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::X), repeat: false, .. } => {
                    key_state_osu &= 0b1111_1101;
                    should_send_osu_data = true;
                },

                // Space
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    key_state_keyboard |= 1 << 39;
                    should_send_keyboard_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    key_state_keyboard = key_state_keyboard & !(1 << 39);
                    should_send_keyboard_data = true;
                },

                // F1
                Event::KeyDown { keycode: Some(Keycode::F1), repeat: false, .. } => {
                    key_state_keyboard |= 1 << 44;
                    should_send_keyboard_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::F1), repeat: false, .. } => {
                    key_state_keyboard = key_state_keyboard & !(1 << 44);
                    should_send_keyboard_data = true;
                },

                // F2
                Event::KeyDown { keycode: Some(Keycode::F2), repeat: false, .. } => {
                    key_state_keyboard |= 1 << 45;
                    should_send_keyboard_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::F2), repeat: false, .. } => {
                    key_state_keyboard = key_state_keyboard & !(1 << 45);
                    should_send_keyboard_data = true;
                },

                // Return
                Event::KeyDown { keycode: Some(Keycode::Return), repeat: false, .. } => {
                    key_state_keyboard |= 1 << 40;
                    should_send_keyboard_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::Return), repeat: false, .. } => {
                    key_state_keyboard = key_state_keyboard & !(1 << 40);
                    should_send_keyboard_data = true;
                },

                // TILDE
                Event::KeyDown { keycode: Some(Keycode::Backquote), repeat: false, .. } => {
                    key_state_keyboard |= 1 << 38;
                    should_send_keyboard_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::Backquote), repeat: false, .. } => {
                    key_state_keyboard = key_state_keyboard & !(1 << 38);
                    should_send_keyboard_data = true;
                },

                // ESC
                Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
                    key_state_keyboard |= 1 << 37;
                    should_send_keyboard_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::Escape), repeat: false, .. } => {
                    key_state_keyboard = key_state_keyboard & !(1 << 37);
                    should_send_keyboard_data = true;
                },

                // Mouse
                Event::MouseWheel { y, ..} => {
                    delta_mouse_wheel = y;
                    should_send_mouse_data = true;
                },
                Event::MouseMotion {..} | Event::MouseButtonDown {..} | Event::MouseButtonUp {..} => should_send_mouse_data = true,
                _ => {}
            }
        }
        if should_send_osu_data {
            connection.send_data("OSU", key_state_osu.to_string().as_str());
        }

        if should_send_keyboard_data {
            connection.send_data("KEYBOARD", key_state_keyboard.to_string().as_str());
        }

        // MOUSE
        if mouse_enabled && should_send_mouse_data {
            let state = events.relative_mouse_state();
            let dx = state.x() as f32 * mouse_speed;
            let dy = state.y() as f32 * mouse_speed;



            if state.is_mouse_button_pressed(MouseButton::Left){
                mouse_state |= 0b0000_0001;
            }else{
                mouse_state &= 0b1111_1110;
            }

            if state.is_mouse_button_pressed(MouseButton::Right){
                mouse_state |= 0b0000_0010;
            }else{
                mouse_state &= 0b1111_1101;
            }

            let payload = format!("{};{};{};{}",
                                   dx as f32,
                                   -dy as f32,
                                   delta_mouse_wheel.to_string(),
                                   mouse_state.to_string()
            );

            connection.send_data("MOUSE",  payload.as_str());
        }

        thread::sleep(Duration::from_millis(poll_rate));
    }
}

fn update_title(window: &mut sdl2::video::Window, connected_to: &String , rate: f32){
    window.set_title(format!("{}@{}ms", connected_to, rate).as_str()).unwrap();
}
