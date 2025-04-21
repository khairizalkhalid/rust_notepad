use xcb::{x, Connection, Xid};
use xkbcommon::xkb;

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

    let (wm_protocols, wm_del_window) = {
        let cookies = (
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: "WM_PROTOCOLS".as_bytes(),
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: "WM_DELETE_WINDOW".as_bytes(),
            }),
        );
        (
            conn.wait_for_reply(cookies.0)?.atom(),
            conn.wait_for_reply(cookies.1)?.atom(),
        )
    };

    conn.check_request(conn.send_request_checked(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[wm_del_window],
    }))?;

    let gc = conn.generate_id();
    conn.check_request(conn.send_request_checked(&x::CreateGc {
        cid: gc,
        drawable: x::Drawable::Window(window),
        value_list: &[
            x::Gc::Foreground(screen.black_pixel()),
            x::Gc::Background(screen.white_pixel()),
        ],
    }))?;

    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let keymap =
        xkb::Keymap::new_from_names(&context, "", "", "", "", None, xkb::KEYMAP_COMPILE_NO_FLAGS)
            .unwrap();
    let state = xkb::State::new(&keymap);

    let mut string_buffer: Vec<u8> = Vec::new();

    loop {
        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::KeyPress(ev)) => {
                let keycode: xkb::Keycode = ev.detail().into();
                let keysym = state.key_get_one_sym(keycode);

                if let Some(ch) = std::char::from_u32(keysym.into()) {
                    let ch_u8 = ch as u8;
                    if ch_u8 == 8 {
                        string_buffer.pop();
                    } else {
                        string_buffer.push(ch_u8);
                    }

                    conn.send_request(&x::ImageText8 {
                        drawable: x::Drawable::Window(window),
                        gc,
                        x: 10,
                        y: 20,
                        string: &string_buffer,
                    });

                    let _ = conn.flush();
                }
            }
            xcb::Event::X(x::Event::ClientMessage(ev)) => {
                if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                    if atom == wm_del_window.resource_id() {
                        break Ok(());
                    }
                }
            }
            _ => {}
        }
    }
}
