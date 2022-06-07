extern crate reqwest;
extern crate rand;
extern crate image;
extern crate sdl2;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use image::Pixel;

const SITE:&str = "https://i.imgur.com";
const SITE_ERR:&str = "https://i.imgur.com/removed.png";

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Image", 1280, 720)
				.position_centered()
				.resizable()
				.opengl()
				.build().unwrap();

	let mut canvas = window.into_canvas().present_vsync().build().unwrap();
	let (img, width, height) = fetch();
	canvas.set_logical_size(width, height).unwrap();
	paint(&mut canvas, img);

	let mut sdl_quit = false;
	while !sdl_quit {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					sdl_quit = true;
				},
				Event::KeyUp { keycode: key, .. } => {
					match key.unwrap() {
						Keycode::Q => {
							sdl_quit = true;
						},
						Keycode::N => {
							let (img, width, height) = fetch();
							canvas.set_logical_size(width, height).unwrap();
							paint(&mut canvas, img);
						},
						_ => {}
					}
				},
				_ => {}
			}
		}
	}

}

fn fetch() -> (image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>, u32, u32) {
	let url_len = vec![5];
	let mut imurl = String::from("");
	let mut rng = thread_rng();

	for _ in 0..*url_len.choose(&mut thread_rng()).unwrap() {
		imurl.push_str(thread_rng().sample(Alphanumeric).to_string().as_str());
	}

	println!("Trying url at {}/{}.png", SITE, imurl);
	let mut response = reqwest::blocking::get(format!("{}/{}.png", SITE, imurl).as_str()).unwrap();

	while !response.status().is_success() || response.url().as_str() == SITE_ERR {
		imurl = (&mut rng).sample_iter(Alphanumeric).take(5).map(char::from).collect();
		println!("Trying url at {}/{}.png", SITE, imurl);
		response = reqwest::blocking::get(format!("{}/{}.png", SITE, imurl).as_str()).unwrap();
	}

	println!("Found valid imgur url at {}/{}.png", SITE, imurl);

	let img = image::load_from_memory(&response.bytes().unwrap()).unwrap().to_rgb8();
	let width = img.width();
	let height = img.height();

	(img, width, height)
	
}

fn paint(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, img: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) {
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();
	let pixels = img.enumerate_pixels();
	for (x, y, pix) in pixels {
		let color = pix.channels();
		canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));
		let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
	}
	canvas.present();
}
