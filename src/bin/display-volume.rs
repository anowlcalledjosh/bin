use libpulse_binding::{
    self as pulse,
    callbacks::ListResult,
    context::{self, Context},
    mainloop::standard::{IterateResult, Mainloop},
    operation,
    volume,
};
use notify_rust::hints::NotificationHint;

use bin::toast::Toast;

const VOLUME_0: u32 = volume::VOLUME_MUTED.0;
const VOLUME_100: u32 = volume::VOLUME_NORM.0;
const BAR_WIDTH: u32 = 20;

fn main() {
    let mut mainloop = Mainloop::new().expect("Could not create mainloop");
    let mut context = Context::new(&mainloop, "display-volume").expect("Could not create context");
    context
        .connect(None, pulse::context::flags::NOFLAGS, None)
        .expect("Could not initiate context connection");
    // XXX: this busyloop is meh
    loop {
        match mainloop.iterate(false) {
            IterateResult::Quit(n) => panic!("Mainloop quit: {:?}", n),
            IterateResult::Err(e) => panic!("Mainloop error: {}", e),
            IterateResult::Success(_) => {}
        }
        match context.get_state() {
            context::State::Ready => break,
            e @ context::State::Failed | e @ context::State::Terminated => {
                panic!("Context disconnected: {:?}", e)
            }
            _ => {}
        }
    }
    let introspector = context.introspect();
    let op = introspector.get_sink_info_by_index(0, |result| match result {
        ListResult::Item(si) => next(si.volume.values[0], si.mute),
        ListResult::End => {}
        ListResult::Error => panic!("Error getting sink info"),
    });
    // XXX: this busyloop is meh
    loop {
        match mainloop.iterate(false) {
            IterateResult::Quit(n) => panic!("Mainloop quit: {:?}", n),
            IterateResult::Err(e) => panic!("Mainloop error: {}", e),
            IterateResult::Success(_) => {}
        }
        match op.get_state() {
            operation::State::Done => break,
            operation::State::Running => {}
            operation::State::Cancelled => panic!("get_sink_info operation cancelled"),
        }
    }
}

fn next(volume: volume::Volume, muted: bool) {
    let display_volume = (((volume.0 - VOLUME_0) as f64 / (VOLUME_100 - VOLUME_0) as f64)
        * BAR_WIDTH as f64)
        .round() as usize;

    // TODO: make this generic wrt BAR_WIDTH.
    let bar = match display_volume {
        0 => "╶┤                   │".to_string(),
        1..=19 => format!(
            "╶┼{}╴{}│",
            "─".repeat(display_volume - 1),
            " ".repeat(19 - display_volume)
        ),
        20 => "╶┼───────────────────┤".to_string(),
        _ => format!(
            "╶┼───────────────────┼{}╴",
            "─".repeat(display_volume - 21)
        ),
    };

    let icon = if muted {
        "audio-volume-muted"
    } else if display_volume < 2 {
        "audio-volume-low"
    } else if display_volume < 20 {
        "audio-volume-medium"
    } else {
        "audio-volume-high"
    };

    Toast::new("volume")
        .summary(&format!(
            "{}{}",
            volume.print().trim(),
            if muted { " (muted)" } else { "" }
        ))
        .body(&bar)
        .icon(icon)
        .hint(NotificationHint::SoundName("audio-volume-change".into()))
        .show()
        .expect("Couldn't show notification");
}
