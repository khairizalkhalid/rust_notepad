use xcb::{x, Connection};

fn main() -> xcb::Result<()> {
    let (conn, _) = Connection::connect(None)?;
    let screen = conn.get_setup().roots().nth(0).unwrap();

    let window: x::Window = conn.generate_id();

    let cookie = conn.send_request_checked(&x::CreateWindow {
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
    conn.check_request(cookie)?;

    let cookie = conn.send_request_checked(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        data: "rust_notepad".as_bytes(),
    });
    conn.check_request(cookie)?;

    let cookie = conn.send_request_checked(&x::MapWindow { window });
    conn.check_request(cookie)?;

    loop {
        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::KeyPress(e)) => {
                println!("e {:?}", e)
            }
            _ => {}
        }
    }
}
