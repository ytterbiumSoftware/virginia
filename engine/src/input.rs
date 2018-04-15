//! Handling for user input is provided by this module.

use sfml::window::Key;
use window::GameWindow;

/// Current input state.
#[derive(Debug)]
pub struct Inputs {
    /// Current keyboard state.
    pub keys: KeyStates,
}

impl Inputs {
    /// Create a new ``Inputs`` from the current immediate inputs
    /// reported a ``GameWindow``.
    pub fn current(win: &GameWindow) -> Inputs {
        let keys = if win.has_focus() {
            KeyStates::current()
        } else {
            Default::default()
        };

        Inputs {
            keys,
        }
    }
}

/// State of the keyboard, represeting immediate inputs.
#[derive(Debug, Default)]
pub struct KeyStates {
    /// Up cursor key.
    pub up: bool,

    /// Doen cursor key.
    pub down: bool,

    /// Left cursor key.
    pub left: bool,

    /// Right cursor key.
    pub right: bool,

    /// A key.
    pub a: bool,

    /// D key.
    pub d: bool,
}

impl KeyStates {
    /// Create a new ``KeyStates`` from current inputs.
    pub fn current() -> KeyStates {
        KeyStates {
            up: Key::Up.is_pressed(),
            down: Key::Down.is_pressed(),
            left: Key::Left.is_pressed(),
            right: Key::Right.is_pressed(),
            a: Key::A.is_pressed(),
            d: Key::D.is_pressed(),
        }
    }
}
