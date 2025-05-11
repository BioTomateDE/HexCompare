use std::collections::HashSet;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;
use std::time::Instant;
use iced::{mouse, Application, Color, Command, Element, Font, Length};
use iced::Event::{Keyboard, Mouse};
use iced::keyboard::Key;
use iced::keyboard::key::Named::{ArrowDown, ArrowUp};
use iced::mouse::ScrollDelta;
use iced::widget::{container, row, Space, Column, text, Row, column, Container};
use crate::Msg;


#[derive(Debug)]
pub struct MainScene {
    pub hexdata1: Vec<[String; COL_COUNT]>,
    pub hexdata2: Vec<[String; COL_COUNT]>,
    pub max_scroll_offset: f32,
    pub scroll_offset: f32,
    pub window_width: f32,
    pub window_height: f32,
}


impl MainScene {
    pub fn update_scene(&mut self, message: Msg) -> Command<Msg> {
        log::info!("update {message:?}");
        match message {
            Msg::KeyPress(Key::Named(ArrowDown)) => {
                self.scroll_offset += 1.0;
                self.scroll_offset = self.scroll_offset.min(self.max_scroll_offset);
            }

            Msg::KeyPress(Key::Named(ArrowUp)) => {
                self.scroll_offset -= 1.0;
                self.scroll_offset = self.scroll_offset.max(0.0);
            }

            Msg::Scroll(amount) => {
                self.scroll_offset -= amount * 10.0;
                self.scroll_offset = self.scroll_offset.min(self.max_scroll_offset);
                self.scroll_offset = self.scroll_offset.max(0.0);
            }

            Msg::WindowResized(width, height) => {
                self.window_width = width as f32;
                self.window_height = height as f32;
            }

            _ => {}
        }
        Command::none()
    }

    pub fn view_scene(&self) -> Element<Msg> {
        log::info!("view");
        let range: Range<usize> = self.scroll_offset as usize .. self.scroll_offset as usize + 100;
        let now = Instant::now();
        let diffs: HashSet<(usize, usize)> = get_diffs(&self.hexdata1[range.clone()], &self.hexdata2[range.clone()]);
        log::info!("Getting diffs took {:?}", Instant::now()-now);

        let mut columns_display: Column<Msg> = Column::new();
        columns_display = columns_display.push(text("").font(Font::MONOSPACE).size(FONT_SIZE));
        for i in range.clone() {
            columns_display = columns_display.push(
                text(format!("{i:03}"))
                    .font(Font::MONOSPACE)
                    .size(FONT_SIZE)
                    .style(Color::from_rgb(0.69, 0.71, 0.72))
            )
        }

        let mut rows_display1: Row<Msg> = Row::new();
        let mut rows_display2: Row<Msg> = Row::new();

        for i in 0..COL_COUNT {
            let elem = text(format!("{i:02} "))
                .font(Font::MONOSPACE)
                .size(FONT_SIZE)
                .style(Color::from_rgb(0.69, 0.71, 0.72));

            rows_display1 = rows_display1.push(elem.clone());
            rows_display2 = rows_display2.push(elem);
        }

        let now = Instant::now();
        let rendered_lines1: Element<Msg> = render_lines(&self.hexdata1[range.clone()], diffs.clone());
        let rendered_lines2: Element<Msg> = render_lines(&self.hexdata2[range.clone()], diffs.clone());
        log::info!("Rendering hexdumps took {:?}", Instant::now()-now);


        container(
            row![
                column![
                    Space::with_height(8),
                    columns_display,
                ],
                Space::with_width(15),
                column![
                    rows_display1,
                    Space::with_height(8),
                    rendered_lines1,
                ],
                Space::with_width(18),
                column![
                    rows_display2,
                    Space::with_height(8),
                    rendered_lines2,
                ],
            ]
        )
            .padding(20)
            .into()
    }
}



pub const COL_COUNT: usize = 16;
pub const FONT_SIZE: f32 = 14.0;

pub fn load_data_file_hex(path: &PathBuf) -> Result<Vec<[String; COL_COUNT]>, String> {
    let data: Vec<u8> = fs::read(path)
        .map_err(|e| format!("Failed to read data file at {path:?}: {e}"))?;

    let mut chunks = Vec::with_capacity((data.len() + COL_COUNT - 1) / COL_COUNT);
    let mut row = [const { String::new() }; COL_COUNT];
    let mut col = 0;

    for byte in data {
        row[col] = format!("{:02X}", byte);
        col += 1;

        if col == COL_COUNT {
            chunks.push(row);
            row = [const { String::new() }; COL_COUNT];
            col = 0;
        }
    }

    if col > 0 {
        // Only partially filled row — leave empty strings in remaining slots
        chunks.push(row);
    }

    Ok(chunks)
}

fn get_diffs(lines1: &[[String; COL_COUNT]], lines2: &[[String; COL_COUNT]]) -> HashSet<(usize, usize)> {
    let mut diffs = HashSet::with_capacity(lines1.len() * COL_COUNT / 10); // heuristic

    for (i, (line1, line2)) in lines1.iter().zip(lines2).enumerate() {
        for (j, (b1, b2)) in line1.iter().zip(line2).enumerate() {
            if b1 != b2 {
                diffs.insert((i, j));
            }
        }
    }

    diffs
}

fn render_lines(lines: &[[String; COL_COUNT]], diffs: HashSet<(usize, usize)>) -> Element<Msg> {
    let mut column: Column<Msg> = Column::new();

    for (i, line) in lines.iter().enumerate() {
        let mut row: Row<Msg> = Row::new();

        for (j, byte) in line.iter().enumerate() {
            let color = if diffs.contains(&(i, j)) {
                Color::from_rgb(0.95, 0.11, 0.09)
            } else {
                Color::from_rgb(0.94, 0.93, 0.91)
            };

            row = row.push(
                text(byte.to_owned())
                    .style(color)
                    .font(Font::MONOSPACE)
                    .size(FONT_SIZE)
            );
            if j < line.len() - 1 {
                row = row.push(
                    text(" ")
                        .font(Font::MONOSPACE)
                        .size(FONT_SIZE)
                );
            }
        }
        column = column.push(row);
    }

    column
        .spacing(0)
        .padding(0)
        .into()
}

