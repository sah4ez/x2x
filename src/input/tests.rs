//! Input module tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_handler_trait() {
        // Test that InputHandler trait is defined
        // This is a compile-time check
        fn assert_implements<T: crate::input::InputHandler>() {}
        assert_implements::<crate::input::MouseHandler>();
        assert_implements::<crate::input::KeyboardHandler>();
    }

    #[test]
    fn test_fake_event_variants() {
        // Test that FakeEvent has the expected variants
        use crate::input::FakeEvent;

        // Create Key event
        let _key_event = FakeEvent::Key {
            keysym: 0x61,
            keycode: 10,
            is_press: true,
        };

        // Create Button event
        let _button_event = FakeEvent::Button {
            button: 1,
            is_press: false,
        };

        // Test matches
        match _key_event {
            FakeEvent::Key { .. } => {},
            FakeEvent::Button { .. } => panic!("Should be Key variant"),
        }

        match _button_event {
            FakeEvent::Button { .. } => {},
            FakeEvent::Key { .. } => panic!("Should be Button variant"),
        }
    }
}
