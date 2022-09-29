use std::collections::HashMap;
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
#[command(author, version, about = "A simple client implementation for SensorHandler.\nhttps://github.com/OwczarekGit/sensorhandler", long_about = None)]
// #[clap(author, version, about)]
struct Args{
    /// Server ip address.
    address: String,

    /// The server port.
    #[arg(short = 'p', long = "port", default_value_t = String::from("2137") )]
    port: String,

    /// Mouse speed.
    #[arg(short = 's', long = "speed", default_value_t = 0.06 )]
    speed: f32,

    /// The event polling rate in ms.
    #[arg(short = 'r', long = "rate", default_value_t = 1 )]
    rate: u64,

    /// Disable mouse event handling.
    #[arg(short = 'm', long = "mouse")]
    mouse: bool,

}

fn main() {
    let mut key_state_osu = 0x00;
    let mut key_state_keyboard = 0x00u128;
    let mut mouse_state = 0x00;

    let args: Args = Args::parse();

    // println!("{:?}",args);

    let mouse_speed = args.speed;
    let server_ip = args.address;
    let port = args.port;
    let poll_rate = args.rate;
    let mouse_enabled = !args.mouse;


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

    if mouse_enabled {
        context.mouse().set_relative_mouse_mode(true);
        context.mouse().show_cursor(true);
    }

    let mut events = context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(44,44,44));
    canvas.clear();
    canvas.present();
    update_title(canvas.window_mut(), &server_ip, poll_rate as f32);

    let mut keymap = HashMap::new();

    keymap.insert("0", 0);
    keymap.insert("1", 1);
    keymap.insert("2", 2);
    keymap.insert("3", 3);
    keymap.insert("4", 4);
    keymap.insert("5", 5);
    keymap.insert("6", 6);
    keymap.insert("7", 7);
    keymap.insert("8", 8);
    keymap.insert("9", 9);

    keymap.insert("A", 10);
    keymap.insert("B", 11);
    keymap.insert("C", 12);
    keymap.insert("D", 13);
    keymap.insert("E", 14);
    keymap.insert("F", 15);
    keymap.insert("G", 16);
    keymap.insert("H", 17);
    keymap.insert("I", 18);
    keymap.insert("J", 19);
    keymap.insert("K", 20);
    keymap.insert("L", 21);
    keymap.insert("M", 22);
    keymap.insert("N", 23);
    keymap.insert("O", 24);
    keymap.insert("P", 25);
    keymap.insert("Q", 26);
    keymap.insert("R", 27);
    keymap.insert("S", 28);
    keymap.insert("T", 29);
    keymap.insert("U", 30);
    keymap.insert("V", 31);
    keymap.insert("W", 32);
    keymap.insert("X", 33);
    keymap.insert("Y", 34);
    keymap.insert("Z", 35);

    keymap.insert( "F1", 36);
    keymap.insert( "F2", 37);
    keymap.insert( "F3", 38);
    keymap.insert( "F4", 39);
    keymap.insert( "F5", 40);
    keymap.insert( "F6", 41);
    keymap.insert( "F7", 42);
    keymap.insert( "F8", 43);
    keymap.insert( "F9", 44);
    keymap.insert("F10", 45);
    keymap.insert("F11", 46);
    keymap.insert("F12", 47);

    keymap.insert("Escape", 48);
    keymap.insert("`", 49);
    keymap.insert("Space", 50);
    keymap.insert("Return", 51);

    keymap.insert("Left Alt", 53);
    keymap.insert("Left Ctrl", 55);
    keymap.insert("Left Shift", 56);
    keymap.insert("Backspace", 57);
    keymap.insert("Tab", 58);
    keymap.insert("CapsLock", 59);



    'running: loop {

        let mut should_send_osu_data = false;
        let mut should_send_mouse_data = false;
        let mut should_send_keyboard_data = false;
        let mut delta_mouse_wheel = 0;

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,

                // osu! Z key.
                Event::KeyDown { keycode: Some(Keycode::Z), repeat: false, .. } => {
                    key_state_osu |= 0b0000_0001;
                    should_send_osu_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::Z), repeat: false, .. } => {
                    key_state_osu &= 0b1111_1110;
                    should_send_osu_data = true;
                },

                // osu! X key
                Event::KeyDown { keycode: Some(Keycode::X), repeat: false, .. } => {
                    key_state_osu |= 0b0000_0010;
                    should_send_osu_data = true;
                },
                Event::KeyUp   { keycode: Some(Keycode::X), repeat: false, .. } => {
                    key_state_osu &= 0b1111_1101;
                    should_send_osu_data = true;
                },

                // Keyboard down
                Event::KeyDown {keycode, repeat: false, ..} =>{
                    if let Some(k) = keycode {
                        if let Some(bit) = keymap.get(k.name().as_str()) {
                            key_state_keyboard |= 1 << bit;
                        };
                        // println!("{}", k.name().as_str());
                    }
                    should_send_keyboard_data = true;
                },

                // Keyboard up
                Event::KeyUp {keycode, repeat: false, ..} =>{
                    if let Some(k) = keycode {
                        if let Some(bit) = keymap.get(k.name().as_str()) {
                            key_state_keyboard = key_state_keyboard & !(1 << bit);
                        };
                    }
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
