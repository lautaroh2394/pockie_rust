use macroquad::{self, window::Conf};

pub fn default_conf() -> Conf {
    let conf = Conf {
        window_title: "Pock".to_string(),
        window_resizable: false,
        window_height: 720,
        window_width: 1080,
        ..Default::default()
    };
    conf
}