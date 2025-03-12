use winrt_notification::init_tracing;
extern crate winrt_notification;
use winrt_notification::{
    Duration,
    Sound,
    Toast,
};

fn main() {
    init_tracing();
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Look at this flip!")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to send notification");
}
