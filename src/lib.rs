use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlButtonElement, HtmlCanvasElement, CanvasRenderingContext2d, Event};

type Result<T> = std::result::Result<T, JsValue>;

const COLORS: [&str; 12] = [
	"#B8D430", "#3AB745", "#029990", "#3501CB",
	"#2E2C75", "#673A7E", "#CC0071", "#F80120",
	"#F35B20", "#FB9A00", "#FFCC00", "#FEF200"
];

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);

	#[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: i32,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut()
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closuer to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval { closure, token }
    }
}

// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.token);
    }
}

fn spin() -> Result<()> {
	let mut i = 0;
	let speed = 4.0;
	let token = Interval::new(1, move || {
		if i < 5000 {
			draw_wheel((i as f64) * speed / 100.0 * 2.0 * std::f64::consts::PI).unwrap();
			i += 1;
		}
	});

	Ok(())
}

#[wasm_bindgen]
pub fn draw_wheel(rotation: f64) -> Result<()> {

	let canvas = get_canvas()?;

	let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

	context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

	for i in 0..12 {
		// create shape around center point (radius, radius)
		context.begin_path();
		context.arc(
			32.into(),
			32.into(),
			32.into(),
			(2.0 * std::f64::consts::PI * (i as f64) + rotation) / 12.0,
			(2.0 * std::f64::consts::PI * (i as f64 + 1.0) + rotation) / 12.0,
		)?;
		context.line_to(
			32.into(),
			32.into()
		);

		context.set_fill_style(&JsValue::from_str(COLORS[i % COLORS.len()]));
		context.stroke();
		context.fill();
	}

	Ok(())
}

fn attach_listener(document: &Document) -> Result<()> {

    let callback = Closure::wrap(Box::new(move |_evt: Event| {
        spin().expect("Could not spin");
    }) as Box<dyn Fn(_)>);

    document
        .get_element_by_id("spin")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()?
        .set_onclick(Some(callback.as_ref().unchecked_ref()));

    callback.forget(); // leaks memory!

    Ok(())
}

fn get_document() -> Result<Document> {
    let window = web_sys::window().unwrap();
    Ok(window.document().unwrap())
}

fn get_canvas() -> Result<HtmlCanvasElement> {
	let canvas = get_document()?
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>();
	Ok(canvas.unwrap())
}

#[wasm_bindgen]
pub fn run() -> Result<()> {
    let document = get_document()?;

	draw_wheel(std::f64::consts::PI)?;
    attach_listener(&document)?;
    Ok(())
}