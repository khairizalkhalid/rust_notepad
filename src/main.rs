use xcb::{x, Connection};

fn main() -> xcb::Result<()> {
    let (conn, _) = Connection::connect(None)?;
    let screen = conn.get_setup().roots().nth(0).unwrap();

    let window: x::Window = conn.generate_id();

    conn.send_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 150,
        height: 150,
        border_width: 1,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        value_list: &[
            x::Cw::BackPixel(screen.white_pixel()),
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
        ],
    });

    conn.send_request(&x::MapWindow { window });

    conn.flush()?;

    loop {
        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::KeyPress(e)) => {
                println!("e {:?}", e)
            }
            _ => {}
        }
    }
}
