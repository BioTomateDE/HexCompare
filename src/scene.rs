use std::collections::HashSet;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;
use iced::{Application, Color, Command, Element, Font, Length};
use iced::keyboard::Key;
use iced::keyboard::key::Named::{ArrowDown, ArrowUp};
use iced::widget::{container, row, Space, Column, text, Row, column};
use crate::Msg;


#[derive(Debug)]
pub struct MainScene {
    pub hexdata1: Vec<[String; COL_COUNT]>,
    pub hexdata2: Vec<[String; COL_COUNT]>,
    pub scroll_drag_start: Option<f32>,
    pub max_scroll_offset: f32,
    pub scroll_offset: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub diffs: HashSet<(usize, usize)>,
}


impl MainScene {
    pub fn update_scene(&mut self, message: Msg) -> Command<Msg> {
        match message {
            Msg::KeyPress(Key::Named(ArrowDown)) => {
                self.scroll_offset += 1.0;
                self.clamp_viewport();
            }

            Msg::KeyPress(Key::Named(ArrowUp)) => {
                self.scroll_offset -= 1.0;
                self.clamp_viewport();
            }

            Msg::Scroll(amount) => {
                self.scroll_offset -= amount * 10.0;
                self.clamp_viewport();
            }

            Msg::WindowResized(width, height) => {
                self.window_width = width as f32;
                self.window_height = height as f32;
            }

            Msg::StartScrollbarDrag => {
                self.scroll_drag_start = Some(self.scroll_offset);
            }

            Msg::EndScrollbarDrag => {
                self.scroll_drag_start = None;
            }

            Msg::DragScrollbar(y) => {
                self.scroll_offset = y/self.window_height * self.max_scroll_offset;
                self.clamp_viewport();
            }

            _ => {}
        }
        Command::none()
    }

    pub fn clamp_viewport(&mut self) {
        self.scroll_offset = self.scroll_offset.min(self.max_scroll_offset - self.window_height);
        self.scroll_offset = self.scroll_offset.max(0.0);
    }

    pub fn view_scene(&self) -> Element<Msg> {
        let range: Range<usize> = self.scroll_offset as usize .. (self.scroll_offset + 100.0).min(self.max_scroll_offset) as usize;

        let mut columns_display: Column<Msg> = Column::new();
        columns_display = columns_display.push(text("").font(Font::MONOSPACE).size(FONT_SIZE));
        for i in range.clone() {
            columns_display = columns_display.push(
                text(format!("{:>8}", i*COL_COUNT))
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

        let rendered_lines1: Element<Msg> = render_lines(&self.hexdata1, &self.diffs, range.clone());
        let rendered_lines2: Element<Msg> = render_lines(&self.hexdata2, &self.diffs, range.clone());

        let scrollbar: Element<Msg> = self.render_scrollbar();

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
                Space::with_width(Length::Fill),
                scrollbar,
            ]
        )
            .padding(20)
            .into()
    }
}



pub const COL_COUNT: usize = 16;
pub const FONT_SIZE: f32 = 11.0;

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

pub fn get_diffs(lines1: &[[String; COL_COUNT]], lines2: &[[String; COL_COUNT]]) -> HashSet<(usize, usize)> {
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

fn render_lines<'a>(lines: &'a Vec<[String; COL_COUNT]>, diffs: &'a HashSet<(usize, usize)>, range: Range<usize>) -> Element<'a, Msg> {
    let mut column: Column<Msg> = Column::new();

    for (i, line) in lines[range.clone()].iter().enumerate() {
        let mut row: Row<Msg> = Row::new();

        for (j, byte) in line.iter().enumerate() {
            let color = if diffs.contains(&(i + range.start, j)) {
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

