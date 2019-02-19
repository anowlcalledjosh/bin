use std::ops::{Deref, DerefMut};

use notify_rust::{hints::NotificationHint, Notification, Timeout};

pub struct Toast(Notification);

impl Toast {
    pub fn new(tag: &str) -> Toast {
        Toast(
            Notification::new()
                .timeout(Timeout::Milliseconds(2000))
                .hint(NotificationHint::Custom(
                    "x-canonical-private-synchronous".into(),
                    tag.to_string(),
                ))
                .hint(NotificationHint::Custom("x-dunst-stack-tag".into(), tag.to_string()))
                .hint(NotificationHint::Custom("synchronous".into(), tag.to_string()))
                .finalize(),
        )
    }
}

impl Deref for Toast {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.0
    }
}

impl DerefMut for Toast {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.0
    }
}
