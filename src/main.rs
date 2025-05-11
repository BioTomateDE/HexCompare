mod scene;
mod scrollbar;

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use iced::{Application, Command, Element, Event, Font, Pixels, Size, Subscription};
use iced::Settings;
use biologischer_log::{init_logger, CustomLogger};
use iced::keyboard::Key;
use iced::mouse::Event::WheelScrolled;
use crate::scene::{get_diffs, load_data_file_hex, MainScene, COL_COUNT, FONT_SIZE};

#[derive(Debug, Clone)]
enum Msg {
    KeyPress(Key),
    Scroll(f32),
    WindowResized(u32, u32),
    StartScrollbarDrag,
    DragScrollbar(f32),
    EndScrollbarDrag,
}

struct MyApp {
    main_window_id: iced::window::Id,
    logger: Arc<CustomLogger>,
    scene: MainScene,
}

#[derive(Clone)]
struct MyAppFlags {
    main_window_id: iced::window::Id,
    logger: Arc<CustomLogger>,
}

const WINDOW_SIZE: Size = Size { width: 800.0, height: 900.0 };

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Theme = iced::Theme;
    type Flags = MyAppFlags;

    fn new(flags: Self::Flags) -> (MyApp, Command<Msg>) {
        log::info!("Started");

        let start = Instant::now();
        let hexdata1: Vec<[String; COL_COUNT]> = load_data_file_hex(&PathBuf::from("C:/Users/BioTomateDE/Documents/RustProjects/LibGM/data.win"))
            .expect("Could not data file 1");
        log::info!("Loading data file 1 took {:?}", Instant::now() - start);

        let start = Instant::now();
        let hexdata2: Vec<[String; COL_COUNT]> = load_data_file_hex(&PathBuf::from("C:/Users/BioTomateDE/Documents/RustProjects/LibGM/data_out.win"))
            .expect("Could not data file 2");
        log::info!("Loading data file 2 took {:?}", Instant::now() - start);

        let start = Instant::now();
        let diffs: HashSet<(usize, usize)> = get_diffs(&hexdata1, &hexdata2);
        log::info!("Generating diffs took {:?}", Instant::now() - start);

        (
            Self {
                main_window_id: flags.main_window_id,
                logger: flags.logger,
                scene: MainScene {
                    max_scroll_offset: hexdata1.len().min(hexdata2.len()) as f32,
                    hexdata1,
                    hexdata2,
                    scroll_offset: 0.0,
                    window_width: WINDOW_SIZE.width,
                    window_height: WINDOW_SIZE.height,
                    scroll_drag_start: None,
                    diffs,
                }
            },
            Command::none()
        )
    }
    fn title(&self) -> String {
        "HexCompare".to_string()
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.scene.update_scene(message)
    }
    fn view(&self) -> Element<Self::Message> {
        self.scene.view_scene()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::GruvboxDark
    }
    fn subscription(&self) -> Subscription<Msg> {
        if self.scene.scroll_drag_start.is_some() {
            return iced::event::listen_with(|event, _status| {
                match event {
                    Event::Mouse(iced::mouse::Event::CursorMoved {position}) => {
                        Some(Msg::DragScrollbar(position.y))
                    }
                    Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                        Some(Msg::EndScrollbarDrag)
                    }
                    _ => None,
                }
            })
        }

        iced::event::listen_with(|event, _status| {
            match event {
                Event::Keyboard(key_event) => {
                    if let iced::keyboard::Event::KeyPressed { key, .. } = key_event {
                        Some(Msg::KeyPress(key))
                    } else {
                        None
                    }
                }

                Event::Mouse(WheelScrolled { delta, .. }) => {
                    let amount = match delta {
                        iced::mouse::ScrollDelta::Lines { y, .. } => y,
                        iced::mouse::ScrollDelta::Pixels { y, .. } => y / FONT_SIZE,
                    };
                    Some(Msg::Scroll(amount))
                }

                Event::Window(_id, iced::window::Event::Resized { width, height }) => {
                    Some(Msg::WindowResized(width, height))
                }

                Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                    Some(Msg::StartScrollbarDrag)
                }

                _ => None,
            }
        })
    }
}

pub fn main() -> iced::Result {
    let logger: Arc<CustomLogger> = init_logger(env!("CARGO_PKG_NAME"));

    let window_settings = iced::window::Settings {
        size: WINDOW_SIZE,
        position: iced::window::Position::Centered,
        min_size: Some(Size{ width: 500.0, height: 500.0 }),
        max_size: None,
        visible: true,
        resizable: true,
        decorations: true,
        transparent: false,
        level: iced::window::Level::Normal,
        icon: None,
        platform_specific: iced::window::settings::PlatformSpecific::default(),
        exit_on_close_request: true,
    };

    let settings = Settings {
        id: Some("HexCompare".to_string()),
        window: window_settings,
        flags: MyAppFlags {
            main_window_id: iced::window::Id::unique(),
            logger,
        },
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels(10.0),
        antialiasing: true,
    };

    MyApp::run(settings)
}

