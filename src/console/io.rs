use device_query::{
    DeviceQuery,
    DeviceState,
    Keycode
};

pub fn kbhit() -> bool {
    DeviceState::new().get_keys().is_empty()
}

pub fn getch() -> Option<Keycode> {
    let keys = DeviceState::new().get_keys();
    if keys.is_empty() { None } else { Some(keys[0].clone()) }
}