pub mod clipboard;
pub mod layer_shell;
pub mod portal;

use crate::types::CaptureResult;
use async_trait::async_trait;

#[async_trait]
pub trait CaptureMethod {
	async fn capture_frame(&self) -> Result<CaptureResult, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait ScreenOverlay {
	fn show_screenshot(&self, captured: CaptureResult) -> Result<(), Box<dyn std::error::Error>>;
	//fn show_overlay(&self); todo
	//fn update_pixels(&self); todo
}

pub fn initialize_capture() -> Box<dyn CaptureMethod> {
	let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

	match desktop.as_str() {
		"KDE" | "GNOME" => Box::new(portal::PortalMethod) as Box<dyn CaptureMethod>,
		_ => Box::new(portal::PortalMethod) as Box<dyn CaptureMethod>, // TODO : For now it needs to stop the app from running
	}
}

pub fn initialize_overlay(conn: wayland_client::Connection) -> Box<dyn ScreenOverlay> {
	let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

	match desktop.as_str() {
		"KDE" => Box::new(layer_shell::KdeOverlay { connection: conn }) as Box<dyn ScreenOverlay>,
		_ => Box::new(layer_shell::KdeOverlay { connection: conn }) as Box<dyn ScreenOverlay>, // TODO : For now it needs to stop the app from running
	}
}


