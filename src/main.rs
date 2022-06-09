extern crate reqwest;
extern crate rand;
extern crate image;
extern crate sdl2;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::env;
use image::Pixel;

const SITE:&str = "https://i.imgur.com";
const SITE_ERR:&str = "https://i.imgur.com/removed.png";

fn main() {
	env::set_var("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");
	let sdl_context = sdl2::init().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Image", 1280, 720)
				.position_centered()
				.resizable()
				.opengl()
				.build().unwrap();

	let mut canvas = window.into_canvas().present_vsync().build().unwrap();
	let (mut img, mut width, mut height) = fetch();
	canvas.set_logical_size(width, height).unwrap();
	paint(&mut canvas, &img);

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
							(img, width, height) = fetch();
							canvas.set_logical_size(width, height).unwrap();
							paint(&mut canvas, &img);
						},
						_ => {}
					}
				},
				Event::Window { win_event: wevent, .. } => {
					match wevent {
						WindowEvent::Resized {..} | WindowEvent::SizeChanged {..} => {
							canvas.set_logical_size(width, height).unwrap();
							paint(&mut canvas, &img);
						},
						_ => {}
					}
				}
				_ => {}
			}
		}
	}

}

fn fetch() -> (image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>, u32, u32) {
	let mut imurl: String;
	let mut rng = thread_rng();

	imurl = (&mut rng).sample_iter(Alphanumeric).take(5).map(char::from).collect();

	let mut response = reqwest::blocking::get(format!("{}/{}.png", SITE, imurl).as_str()).unwrap();

	while !response.status().is_success() || response.url().as_str() == SITE_ERR {
		imurl = (&mut rng).sample_iter(Alphanumeric).take(5).map(char::from).collect();
		response = reqwest::blocking::get(format!("{}/{}.png", SITE, imurl).as_str()).unwrap();
	}

	println!("Found valid imgur url at {}/{}.png", SITE, imurl);

	let img = image::load_from_memory(&response.bytes().unwrap()).unwrap().to_rgb8();
	let width = img.width();
	let height = img.height();

	(img, width, height)
	
}

fn paint(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, img: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) {
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();
	let pixels = img.enumerate_pixels();
	for (x, y, pix) in pixels {
		let color = pix.channels();
		canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));
		canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32)).unwrap();
	}
	canvas.present();
}
