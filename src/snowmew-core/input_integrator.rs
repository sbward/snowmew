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

use game::Game;
use input;
use input::Event;
use input::Button;
use common::{Common, CommonData};
use std::collections::{HashSet, HashMap};
use serialize::Encodable;


/// This `wraps` your game to allow the `input integrator` to
/// collect input events to simplify event handling.
#[deriving(Clone)]
pub struct InputIntegrator<Game> {
    game: Game
}

impl<Game> InputIntegrator<Game> {
    fn new(game: Game) -> InputIntegrator<Game> {
        InputIntegrator { game: game }
    }
}

/// Contains the collected input state
#[deriving(Clone, Encodable, Decodable)]
pub struct InputIntegratorState {
    buttons_down: HashMap<Button, uint>,
    buttons_released: HashSet<Button>,
    index: uint,
    time: f64,
    last_mouse: Option<(uint, f64, f64)>,
    mouse: Option<(uint, f64, f64)>,
    scroll: (int, int),
    last_scroll: (int, int)
}

impl InputIntegratorState {
    /// check to see if a button is currently down
    pub fn button_down(&self, button: Button) -> bool {
        self.buttons_down.get(&button).is_some()
    }

    /// check to see if the button was just pressed this frame
    pub fn button_pressed(&self, button: Button) -> bool {
        if let Some(&x) = self.buttons_down.get(&button) {
            self.index - x == 1
        } else {
            false
        }
    }

    /// check to see if the button was release this frame
    pub fn button_released(&self, button: Button) -> bool {
        self.buttons_released.contains(&button)
    }

    /// get the absolute mouse position
    pub fn mouse_position(&self) -> (f64, f64) {
        if let Some((_, x, y)) = self.mouse {
            (x, y)
        } else {
            (0., 0.)
        }
    }

    /// get the mouse position relative to the last frame
    pub fn mouse_delta(&self) -> (f64, f64) {
        match (self.mouse, self.last_mouse) {
            (Some((new_i, new_x, new_y)),
             Some((old_i, old_x, old_y))) if old_i != new_i => {
                (new_x - old_x, new_y - old_y)
            }
            (Some((new_i, _, _)),
             Some((old_i, _, _))) if old_i == new_i => (0., 0.),
            (Some((_, x, y)), _) => (x, y),
            _ => (0., 0.)
        }
    }

    /// get the scroll wheels absolute position (total number of turns)
    pub fn scroll_position(&self) -> (int, int) { self.scroll }

    /// get the change in the scroll wheels position since last frame.
    pub fn scroll_delta(&self) -> (int, int) {
        let (x, y) = self.scroll;
        let (ox, oy) = self.last_scroll;
        (x - ox, y - oy)
    }

    /// get the current frame index
    pub fn index(&self) -> uint { self.index }

    /// get the current frame time
    pub fn time(&self) -> f64 { self.time }
}

/// This wraps the supplied GameData so that it contains
/// both the gamedata and the input integrators current state
#[deriving(Clone)]
pub struct InputIntegratorGameData<GameData> {
    state: InputIntegratorState,
    /// The GameData that is wrapped by the Integrator
    pub inner: GameData
}

impl<GameData> InputIntegratorGameData<GameData> {
    fn new(inner: GameData) -> InputIntegratorGameData<GameData> {
        InputIntegratorGameData {
            state: InputIntegratorState {
                buttons_down: HashMap::new(),
                buttons_released: HashSet::new(),
                index: 0,
                time: 0.,
                last_mouse: None,
                mouse: None,
                scroll: (0, 0),
                last_scroll: (0, 0)
            },
            inner: inner
        }
    }
}

impl<T: Common> Common for InputIntegratorGameData<T> {
    fn get_common<'a>(&'a self) -> &'a CommonData { self.inner.get_common() }
    fn get_common_mut<'a>(&'a mut self) -> &'a mut CommonData { self.inner.get_common_mut() }
}

/// Create an input integrator, this wraps your game and its state
pub fn input_integrator<Game, GameData>(game: Game, inner: GameData)
    -> (InputIntegrator<Game>, InputIntegratorGameData<GameData>) {
    (InputIntegrator::new(game), InputIntegratorGameData::new(inner))
}

impl<GameData,
     InputGame: Game<GameData, InputIntegratorState>>
    Game<InputIntegratorGameData<GameData>, Event> for InputIntegrator<InputGame> {
    fn step(&mut self, event: Event, gd: InputIntegratorGameData<GameData>)
        -> InputIntegratorGameData<GameData> {
        let mut gd = gd;

        match event {
            input::Cadance(index, time) => {
                gd.state.index = index;
                gd.state.time = time;
                // move the internal game
                gd.inner = self.game.step(gd.state.clone(), gd.inner);
                gd.state.last_mouse = gd.state.mouse;
                gd.state.buttons_released.clear();
                gd.state.last_scroll = gd.state.scroll;
            }
            input::ButtonDown(button) => {
                gd.state.buttons_down.insert(button, gd.state.index);
            }
            input::ButtonUp(button) => {
                gd.state.buttons_down.remove(&button);
                gd.state.buttons_released.insert(button);
            }
            input::Move(x, y) => {
                gd.state.mouse = Some((gd.state.index, x, y));
            }
            input::Scroll(dx, dy) => {
                let (x, y) = gd.state.scroll;
                gd.state.scroll = (x + dx, y + dy);
            }
        }

        gd
    }
}