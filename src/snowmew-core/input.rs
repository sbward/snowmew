//   Copyright 2014 Colin Sherratt
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use glfw;

#[deriving(Clone, Show, Eq, PartialEq, Hash, Decodable, Encodable)]
pub enum Button {
    KeyboardSpace,
    KeyboardApostrophe,
    KeyboardComma,
    KeyboardMinus,
    KeyboardPeriod,
    KeyboardSlash,
    Keyboard0,
    Keyboard1,
    Keyboard2,
    Keyboard3,
    Keyboard4,
    Keyboard5,
    Keyboard6,
    Keyboard7,
    Keyboard8,
    Keyboard9,
    KeyboardSemicolon,
    KeyboardEqual,
    KeyboardA,
    KeyboardB,
    KeyboardC,
    KeyboardD,
    KeyboardE,
    KeyboardF,
    KeyboardG,
    KeyboardH,
    KeyboardI,
    KeyboardJ,
    KeyboardK,
    KeyboardL,
    KeyboardM,
    KeyboardN,
    KeyboardO,
    KeyboardP,
    KeyboardQ,
    KeyboardR,
    KeyboardS,
    KeyboardT,
    KeyboardU,
    KeyboardV,
    KeyboardW,
    KeyboardX,
    KeyboardY,
    KeyboardZ,
    KeyboardLeftBracket,
    KeyboardBackslash,
    KeyboardRightBracket,
    KeyboardGraveAccent,
    KeyboardWorld1,
    KeyboardWorld2,
    KeyboardEscape,
    KeyboardEnter,
    KeyboardTab,
    KeyboardBackspace,
    KeyboardInsert,
    KeyboardDelete,
    KeyboardRight,
    KeyboardLeft,
    KeyboardDown,
    KeyboardUp,
    KeyboardPageUp,
    KeyboardPageDown,
    KeyboardHome,
    KeyboardEnd,
    KeyboardCapsLock,
    KeyboardScrollLock,
    KeyboardNumLock,
    KeyboardPrintScreen,
    KeyboardPause,
    KeyboardF1,
    KeyboardF2,
    KeyboardF3,
    KeyboardF4,
    KeyboardF5,
    KeyboardF6,
    KeyboardF7,
    KeyboardF8,
    KeyboardF9,
    KeyboardF10,
    KeyboardF11,
    KeyboardF12,
    KeyboardF13,
    KeyboardF14,
    KeyboardF15,
    KeyboardF16,
    KeyboardF17,
    KeyboardF18,
    KeyboardF19,
    KeyboardF20,
    KeyboardF21,
    KeyboardF22,
    KeyboardF23,
    KeyboardF24,
    KeyboardF25,
    KeyboardKp0,
    KeyboardKp1,
    KeyboardKp2,
    KeyboardKp3,
    KeyboardKp4,
    KeyboardKp5,
    KeyboardKp6,
    KeyboardKp7,
    KeyboardKp8,
    KeyboardKp9,
    KeyboardKpDecimal,
    KeyboardKpDivide,
    KeyboardKpMultiply,
    KeyboardKpSubtract,
    KeyboardKpAdd,
    KeyboardKpEnter,
    KeyboardKpEqual,
    KeyboardLeftShift,
    KeyboardLeftControl,
    KeyboardLeftAlt,
    KeyboardLeftSuper,
    KeyboardRightShift,
    KeyboardRightControl,
    KeyboardRightAlt,
    KeyboardRightSuper,
    KeyboardMenu,
    MouseLeft,
    MouseRight,
    MouseCenter,
    MouseExt0,
    MouseExt1,
    MouseExt2,
    MouseExt3,
    MouseExt4
}

fn from_glfw_key(key: glfw::Key) -> Button {
    return match key {
        glfw::KeySpace => KeyboardSpace,
        glfw::KeyApostrophe => KeyboardApostrophe,
        glfw::KeyComma => KeyboardComma,
        glfw::KeyMinus => KeyboardMinus,
        glfw::KeyPeriod => KeyboardPeriod,
        glfw::KeySlash => KeyboardSlash,
        glfw::Key0 => Keyboard0,
        glfw::Key1 => Keyboard1,
        glfw::Key2 => Keyboard2,
        glfw::Key3 => Keyboard3,
        glfw::Key4 => Keyboard4,
        glfw::Key5 => Keyboard5,
        glfw::Key6 => Keyboard6,
        glfw::Key7 => Keyboard7,
        glfw::Key8 => Keyboard8,
        glfw::Key9 => Keyboard9,
        glfw::KeySemicolon => KeyboardSemicolon,
        glfw::KeyEqual => KeyboardEqual,
        glfw::KeyA => KeyboardA,
        glfw::KeyB => KeyboardB,
        glfw::KeyC => KeyboardC,
        glfw::KeyD => KeyboardD,
        glfw::KeyE => KeyboardE,
        glfw::KeyF => KeyboardF,
        glfw::KeyG => KeyboardG,
        glfw::KeyH => KeyboardH,
        glfw::KeyI => KeyboardI,
        glfw::KeyJ => KeyboardJ,
        glfw::KeyK => KeyboardK,
        glfw::KeyL => KeyboardL,
        glfw::KeyM => KeyboardM,
        glfw::KeyN => KeyboardN,
        glfw::KeyO => KeyboardO,
        glfw::KeyP => KeyboardP,
        glfw::KeyQ => KeyboardQ,
        glfw::KeyR => KeyboardR,
        glfw::KeyS => KeyboardS,
        glfw::KeyT => KeyboardT,
        glfw::KeyU => KeyboardU,
        glfw::KeyV => KeyboardV,
        glfw::KeyW => KeyboardW,
        glfw::KeyX => KeyboardX,
        glfw::KeyY => KeyboardY,
        glfw::KeyZ => KeyboardZ,
        glfw::KeyLeftBracket => KeyboardLeftBracket,
        glfw::KeyBackslash => KeyboardBackslash,
        glfw::KeyRightBracket => KeyboardRightBracket,
        glfw::KeyGraveAccent => KeyboardGraveAccent,
        glfw::KeyWorld1 => KeyboardWorld1,
        glfw::KeyWorld2 => KeyboardWorld2,
        glfw::KeyEscape => KeyboardEscape,
        glfw::KeyEnter => KeyboardEnter,
        glfw::KeyTab => KeyboardTab,
        glfw::KeyBackspace => KeyboardBackspace,
        glfw::KeyInsert => KeyboardInsert,
        glfw::KeyDelete => KeyboardDelete,
        glfw::KeyRight => KeyboardRight,
        glfw::KeyLeft => KeyboardLeft,
        glfw::KeyDown => KeyboardDown,
        glfw::KeyUp => KeyboardUp,
        glfw::KeyPageUp => KeyboardPageUp,
        glfw::KeyPageDown => KeyboardPageDown,
        glfw::KeyHome => KeyboardHome,
        glfw::KeyEnd => KeyboardEnd,
        glfw::KeyCapsLock => KeyboardCapsLock,
        glfw::KeyScrollLock => KeyboardScrollLock,
        glfw::KeyNumLock => KeyboardNumLock,
        glfw::KeyPrintScreen => KeyboardPrintScreen,
        glfw::KeyPause => KeyboardPause,
        glfw::KeyF1 => KeyboardF1,
        glfw::KeyF2 => KeyboardF2,
        glfw::KeyF3 => KeyboardF3,
        glfw::KeyF4 => KeyboardF4,
        glfw::KeyF5 => KeyboardF5,
        glfw::KeyF6 => KeyboardF6,
        glfw::KeyF7 => KeyboardF7,
        glfw::KeyF8 => KeyboardF8,
        glfw::KeyF9 => KeyboardF9,
        glfw::KeyF10 => KeyboardF10,
        glfw::KeyF11 => KeyboardF11,
        glfw::KeyF12 => KeyboardF12,
        glfw::KeyF13 => KeyboardF13,
        glfw::KeyF14 => KeyboardF14,
        glfw::KeyF15 => KeyboardF15,
        glfw::KeyF16 => KeyboardF16,
        glfw::KeyF17 => KeyboardF17,
        glfw::KeyF18 => KeyboardF18,
        glfw::KeyF19 => KeyboardF19,
        glfw::KeyF20 => KeyboardF20,
        glfw::KeyF21 => KeyboardF21,
        glfw::KeyF22 => KeyboardF22,
        glfw::KeyF23 => KeyboardF23,
        glfw::KeyF24 => KeyboardF24,
        glfw::KeyF25 => KeyboardF25,
        glfw::KeyKp0 => KeyboardKp0,
        glfw::KeyKp1 => KeyboardKp1,
        glfw::KeyKp2 => KeyboardKp2,
        glfw::KeyKp3 => KeyboardKp3,
        glfw::KeyKp4 => KeyboardKp4,
        glfw::KeyKp5 => KeyboardKp5,
        glfw::KeyKp6 => KeyboardKp6,
        glfw::KeyKp7 => KeyboardKp7,
        glfw::KeyKp8 => KeyboardKp8,
        glfw::KeyKp9 => KeyboardKp9,
        glfw::KeyKpDecimal => KeyboardKpDecimal,
        glfw::KeyKpDivide => KeyboardKpDivide,
        glfw::KeyKpMultiply => KeyboardKpMultiply,
        glfw::KeyKpSubtract => KeyboardKpSubtract,
        glfw::KeyKpAdd => KeyboardKpAdd,
        glfw::KeyKpEnter => KeyboardKpEnter,
        glfw::KeyKpEqual => KeyboardKpEqual,
        glfw::KeyLeftShift => KeyboardLeftShift,
        glfw::KeyLeftControl => KeyboardLeftControl,
        glfw::KeyLeftAlt => KeyboardLeftAlt,
        glfw::KeyLeftSuper => KeyboardLeftSuper,
        glfw::KeyRightShift => KeyboardRightShift,
        glfw::KeyRightControl => KeyboardRightControl,
        glfw::KeyRightAlt => KeyboardRightAlt,
        glfw::KeyRightSuper => KeyboardRightSuper,
        glfw::KeyMenu => KeyboardMenu,
    };
}

pub fn from_glfw_mouse_button(key: glfw::MouseButton) -> Button {
    return match key {
        glfw::MouseButton1 => MouseLeft,
        glfw::MouseButton2 => MouseRight,
        glfw::MouseButton3 => MouseCenter,
        glfw::MouseButton4 => MouseExt0,
        glfw::MouseButton5 => MouseExt1,
        glfw::MouseButton6 => MouseExt2,
        glfw::MouseButton7 => MouseExt3,
        glfw::MouseButton8 => MouseExt4,
    };
}

#[deriving(Clone, Show, PartialEq)]
pub enum Event {
    ButtonDown(Button),
    ButtonUp(Button),
    Move(f64, f64),
    Scroll(int, int),
    Cadance(uint, f64)
}

#[deriving(Clone, Show, PartialEq)]
pub enum WindowEvent {
    MouseOver(bool),
    Position(int, int),
    Size(int, int)
}

#[deriving(Clone, Show, PartialEq)]
pub enum EventGroup {
    Game(Event),
    Window(WindowEvent),
    NopEvent
}

impl Event {
    pub fn from_glfw(evt: glfw::WindowEvent) -> EventGroup {
        match evt {
            glfw::MouseButtonEvent(button, glfw::Press, _) => {
                Game(ButtonDown(from_glfw_mouse_button(button)))
            }
            glfw::MouseButtonEvent(button, glfw::Release, _) => {
                Game(ButtonUp(from_glfw_mouse_button(button)))
            }
            glfw::KeyEvent(button, _, glfw::Press, _) => {
                Game(ButtonDown(from_glfw_key(button)))
            }
            glfw::KeyEvent(button, _, glfw::Release, _) => {
                Game(ButtonUp(from_glfw_key(button)))
            }
            glfw::CursorPosEvent(x, y) => {
                Game(Move(x, y))
            }
            glfw::ScrollEvent(x, y) => {
                Game(Scroll(x as int, y as int))
            }
            glfw::CursorEnterEvent(x) => {
                Window(MouseOver(x))
            }
            glfw::PosEvent(x, y) => {
                Window(Position(x as int, y as int))
            }
            glfw::FramebufferSizeEvent(x, y) => {
                Window(Size(x as int, y as int))
            }
            x => {
                println!("unhandled {}", x);
                NopEvent
            }
        }
    }
}