use xcb::Connection;

fn main() -> xcb::Result<()> {
    let (conn, _) = Connection::connect(None)?;
    let root = conn.get_setup().roots().nth(0).unwrap();

    println!(
        "Successfully connected to X server. Root window ID: {:?}",
        root.root()
    );

    Ok(())
}
