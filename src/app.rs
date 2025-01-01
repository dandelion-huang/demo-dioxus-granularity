use dioxus::prelude::*;
use rand::Rng;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
	let mut count = use_signal(|| 0);

	rsx! {
	  document::Title { "Demo Dioxus Granularity" }
	  document::Link { rel: "stylesheet", href: TAILWIND_CSS }
	  div { class: "flex justify-center items-center bg-black h-dvh",
		BgContainer { class_name: Some("p-16 text-center"), count,
		  BgContainer { class_name: Some("p-16 text-center"), count }
		}
		button {
		  class: "py-2 px-4 bg-white rounded-md",
		  onclick: move |_| count += 1,
		  "Click me"
		}
	  }
	}
}

const COLORS: [&str; 9] = [
	"lime", "amber", "cyan", "teal", "rose", "fuchsia", "red", "yellow", "indigo",
];

const COLOR_CODES: [&str; 5] = ["200", "300", "400", "500", "600"];

pub fn random_bg_color() -> String {
	let mut rng = rand::thread_rng();

	let from_color = COLORS[rng.gen_range(0..COLORS.len())];
	let from_code = COLOR_CODES[rng.gen_range(0..COLOR_CODES.len())];

	let to_color = COLORS[rng.gen_range(0..COLORS.len())];
	let to_code = COLOR_CODES[rng.gen_range(0..COLOR_CODES.len())];

	format!(
		"bg-gradient-to-br from-{}-{} to-{}-{}",
		from_color, from_code, to_color, to_code
	)
}

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn BgContainer(
	children: Element,
	class_name: Option<&'static str>,
	count: Signal<i32>,
) -> Element {
	rsx! {
	  div { class: format!("{} {}", class_name.unwrap_or(""), random_bg_color()),
		p { class: "mb-0.5", "Count: {count}" }
		{children}
	  }
	}
}
