use evdev::KeyCode;

// Every key that Big Picture allows binding.

pub const KEYS: &[Key] = &[
    KeyCode::KEY_ESC,
    KeyCode::KEY_F1,
    KeyCode::KEY_F2,
    KeyCode::KEY_F3,
    KeyCode::KEY_F4,
    KeyCode::KEY_F5,
    KeyCode::KEY_F6,
    KeyCode::KEY_F7,
    KeyCode::KEY_F8,
    KeyCode::KEY_F9,
    KeyCode::KEY_F10,
    KeyCode::KEY_F11,
    KeyCode::KEY_F12,

    KeyCode::KEY_GRAVE, // `
    KeyCode::KEY_1,
    KeyCode::KEY_2,
    KeyCode::KEY_3,
    KeyCode::KEY_4,
    KeyCode::KEY_5,
    KeyCode::KEY_6,
    KeyCode::KEY_7,
    KeyCode::KEY_8,
    KeyCode::KEY_9,
    KeyCode::KEY_0,
    KeyCode::KEY_MINUS,
    KeyCode::KEY_EQUAL,
    KeyCode::KEY_BACKSPACE,

    KeyCode::KEY_TAB,
    KeyCode::KEY_Q,
    KeyCode::KEY_W,
    KeyCode::KEY_E,
    KeyCode::KEY_R,
    KeyCode::KEY_T,
    KeyCode::KEY_Y,
    KeyCode::KEY_U,
    KeyCode::KEY_I,
    KeyCode::KEY_O,
    KeyCode::KEY_P,
    KeyCode::KEY_LEFTBRACE,
    KeyCode::KEY_RIGHTBRACE,
    KeyCode::KEY_BACKSLASH,

    KeyCode::KEY_CAPSLOCK,
    KeyCode::KEY_A,
    KeyCode::KEY_S,
    KeyCode::KEY_D,
    KeyCode::KEY_F,
    KeyCode::KEY_G,
    KeyCode::KEY_H,
    KeyCode::KEY_J,
    KeyCode::KEY_K,
    KeyCode::KEY_L,
    KeyCode::KEY_SEMICOLON,
    KeyCode::KEY_APOSTROPHE,
    KeyCode::KEY_ENTER,

    KeyCode::KEY_LEFTSHIFT,
    KeyCode::KEY_Z,
    KeyCode::KEY_X,
    KeyCode::KEY_C,
    KeyCode::KEY_V,
    KeyCode::KEY_B,
    KeyCode::KEY_N,
    KeyCode::KEY_M,
    KeyCode::KEY_COMMA,
    KeyCode::KEY_DOT,
    KeyCode::KEY_SLASH,
    KeyCode::KEY_RIGHTSHIFT,

    KeyCode::KEY_LEFTCTRL,
    KeyCode::KEY_RIGHTCTRL,
    KeyCode::KEY_LEFTMETA,
    KeyCode::KEY_LEFTALT,
    KeyCode::KEY_SPACE,
    KeyCode::KEY_RIGHTALT,

    KeyCode::KEY_VOLUMEUP,
    KeyCode::KEY_VOLUMEDOWN,
    KeyCode::KEY_MUTE,
    KeyCode::KEY_PLAY,
    KeyCode::KEY_STOP,
    KeyCode::KEY_NEXT,
    KeyCode::KEY_PREVIOUS,
    KeyCode::KEY_PREVIOUSSONG,
    KeyCode::KEY_NEXTSONG,
    KeyCode::KEY_PLAYPAUSE,
    KeyCode::KEY_INSERT,
    KeyCode::KEY_HOME,
    KeyCode::KEY_PAGEUP,
    KeyCode::KEY_DELETE,
    KeyCode::KEY_END,
    KeyCode::KEY_PAGEDOWN,

    KeyCode::KEY_UP,
    KeyCode::KEY_LEFT,
    KeyCode::KEY_RIGHT,
    KeyCode::KEY_DOWN,

    KeyCode::KEY_NUMLOCK,
    KeyCode::KEY_KPSLASH,
    KeyCode::KEY_KPASTERISK,
    KeyCode::KEY_KPMINUS,

    KeyCode::KEY_KP7,
    KeyCode::KEY_KP8,
    KeyCode::KEY_KP9,
    KeyCode::KEY_KPPLUS,

    KeyCode::KEY_KP4,
    KeyCode::KEY_KP5,
    KeyCode::KEY_KP6,

    KeyCode::KEY_KP1,
    KeyCode::KEY_KP2,
    KeyCode::KEY_KP3,
    KeyCode::KEY_KPENTER,

    KeyCode::KEY_KP0,
    KeyCode::KEY_KPDOT,
    KeyCode::KEY_KPLEFTPAREN,
    KeyCode::KEY_KPRIGHTPAREN,
    KeyCode::KEY_102ND, // Output by < (less than key)
];
