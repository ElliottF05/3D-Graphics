use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::js_sys;
use web_sys::Event;
use web_sys::EventTarget;
use web_sys::ImageData;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

use crate::graphics::game::Game;
use crate::utils::math::Vec3;


// WASM UTIL EXPORTS
#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

pub fn get_time() -> f64 {
    return web_sys::window()
        .expect("No global window exists")
        .performance()
        .expect("Window doesn't have Performance")
        .now();
}


#[wasm_bindgen]
pub fn init_and_begin_game_loop() {

    let game = Rc::new(RefCell::new(Game::new()));

    // Access the window, document, and peformance objects
    let window = web_sys::window().expect("No global window exists");
    let document = window.document().expect("No document on the window");
    // let performance = window.performance().expect("Window doesn't have performance");

    // Get the canvas element by its ID
    let canvas_id = "main-canvas";
    let canvas = document
        .get_element_by_id(canvas_id)
        .expect(&format!("Element with id {canvas_id} element couldn't be found"))
        .dyn_into::<HtmlCanvasElement>()
        .expect("Element couldn't be cast as HTML canvas element");

    let width = canvas.width();
    let height = canvas.height();

    // Get the 2D rendering context
    let ctx = canvas
        .get_context("2d")
        .expect("Couldn't get 2d context for canvas")
        .expect("Also couldn't get 2d context for canvas")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("Context couldn't be cast as CanvasRenderingContext2d");


    // Add event listeners
    let game_clone = game.clone();
    add_event_listener(&window, "keydown", move |event: Event| {
        let event = event.dyn_ref::<KeyboardEvent>().expect("Failed to cast keydown event to KeyboardEvent");
        // console_log!("Key pressed: {}", event.key());
        game_clone.borrow_mut().keys_currently_pressed.insert(event.key());
        game_clone.borrow_mut().keys_pressed_last_frame.insert(event.key());
    });

    let game_clone = game.clone();
    add_event_listener(&window, "keyup", move |event: Event| {
        let event = event.dyn_ref::<KeyboardEvent>().expect("Failed to cast keyup event to KeyboardEvent");
        game_clone.borrow_mut().keys_currently_pressed.remove(&event.key());
    });

    let canvas_clone = canvas.clone();
    add_event_listener(&canvas, "click",  move |event: Event| {
        let event = event.dyn_ref::<MouseEvent>().expect("Failed to cast click to MouseEvent");
        if event.button() == 0 {
            canvas_clone.request_pointer_lock();
        }
    });

    let game_clone = game.clone();
    add_event_listener(&canvas, "mousemove",  move |event: Event| {
        let event = event.dyn_ref::<MouseEvent>().expect("Failed to cast mousemove to MouseEvent");
        if document.pointer_lock_element().is_some() {
            game_clone.borrow_mut().mouse_move.x = event.movement_x() as f32;
            game_clone.borrow_mut().mouse_move.y = event.movement_y() as f32;
        } else {
            game_clone.borrow_mut().mouse_move = Vec3::new(0.0, 0.0, 0.0);
        }
    });

    // Start the main game loop
    start_game_loop(&window, move || {

        game.borrow_mut().game_loop();

        // Draw the pixel buffer onto the canvas
        let image_data = ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(&game.borrow().pixel_buf.get_buf()), width)
            .expect("Failed to create ImageData");
        ctx.put_image_data(&image_data, 0.0, 0.0)
            .expect("Failed to put image data on canvas");
    });
    

}

/// A helper function to run a game loop, calling `update_fn` each frame.
fn start_game_loop<F>(window: &Window, mut update_fn: F)
where
    F: 'static + FnMut(),
{
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let window_clone = window.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Call the user-provided update function each frame
        update_fn();

        // Request the next frame
        window_clone
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("Failed to request animation frame");
    }) as Box<dyn FnMut()>));

    // Start the loop
    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("Failed to start animation frame");
}

// Generic function to add an event listener to any target
fn add_event_listener<F>(target: &EventTarget, event_type: &str, callback: F)
where
    F: 'static + FnMut(Event),
{
    let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(Event)>);

    target
        .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
        .expect("Failed to add event listener");

    closure.forget(); // Prevents the closure from being dropped
}