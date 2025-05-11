use std::collections::HashSet;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;
use iced::{Color, Command, Element, Font, Length};
use iced::widget::{container, row, Space, Column, text, Row};
use crate::Msg;


#[derive(Debug)]
pub struct MainScene {
    pub hexdata1: Vec<[String; COL_COUNT]>,
    pub hexdata2: Vec<[String; COL_COUNT]>,
    pub scroll_offset: f32,
}


impl MainScene {
    pub fn update_scene(&mut self, message: Msg) -> Command<Msg> {
        log::info!("update {message:?}");
        match message {
            Msg::EditData1(action) => log::info!("edit data1: {}", action.is_edit()),
            Msg::EditData2(action) => log::info!("edit data2: {}", action.is_edit()),
        }
        Command::none()
    }

    pub fn view_scene(&self) -> Element<Msg> {
        log::info!("view");
        let range: Range<usize> = self.scroll_offset as usize .. self.scroll_offset as usize + 100;
        let diffs: HashSet<(usize, usize)> = get_diffs(&self.hexdata1, &self.hexdata2);

        container(
            row![
                render_lines(&self.hexdata1[range.clone()], diffs.clone()),
                Space::with_width(Length::Fill),
                render_lines(&self.hexdata2[range], diffs),
            ]
        )
            .padding(20)
            .into()
    }
}



pub const COL_COUNT: usize = 16;

pub fn load_data_file_hex(path: &PathBuf) -> Result<Vec<[String; COL_COUNT]>, String> {
    const COL_COUNT: usize = 16;

    let data: Vec<u8> = fs::read(path)
        .map_err(|e| format!("Failed to read data file at {path:?}: {e}"))?;

    // Collect all hex byte strings first
    let hex_strs: Vec<String> = data.iter()
        .map(|b| format!("{:02X}", b))
        .collect();

    // Chunk into fixed-size arrays
    let mut chunks: Vec<[String; COL_COUNT]> = Vec::new();
    let mut i = 0;

    while i + COL_COUNT <= hex_strs.len() {
        let mut arr: [String; COL_COUNT] = std::array::from_fn(|_| String::new());
        for j in 0..COL_COUNT {
            arr[j] = hex_strs[i + j].clone();
        }
        chunks.push(arr);
        i += COL_COUNT;
    }

    // Handle final partial chunk if needed
    if i < hex_strs.len() {
        let mut arr: [String; COL_COUNT] = std::array::from_fn(|_| String::new());
        for j in 0..(hex_strs.len() - i) {
            arr[j] = hex_strs[i + j].clone();
        }
        chunks.push(arr);
    }

    Ok(chunks)
}



const HEX_TABLE: &[u8; 767] = b"\
00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F \
10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F \
20 21 22 23 24 25 26 27 28 29 2A 2B 2C 2D 2E 2F \
30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F \
40 41 42 43 44 45 46 47 48 49 4A 4B 4C 4D 4E 4F \
50 51 52 53 54 55 56 57 58 59 5A 5B 5C 5D 5E 5F \
60 61 62 63 64 65 66 67 68 69 6A 6B 6C 6D 6E 6F \
70 71 72 73 74 75 76 77 78 79 7A 7B 7C 7D 7E 7F \
80 81 82 83 84 85 86 87 88 89 8A 8B 8C 8D 8E 8F \
90 91 92 93 94 95 96 97 98 99 9A 9B 9C 9D 9E 9F \
A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 AA AB AC AD AE AF \
B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 BA BB BC BD BE BF \
C0 C1 C2 C3 C4 C5 C6 C7 C8 C9 CA CB CC CD CE CF \
D0 D1 D2 D3 D4 D5 D6 D7 D8 D9 DA DB DC DD DE DF \
E0 E1 E2 E3 E4 E5 E6 E7 E8 E9 EA EB EC ED EE EF \
F0 F1 F2 F3 F4 F5 F6 F7 F8 F9 FA FB FC FD FE FF";


fn get_diffs(lines1: &[[String; COL_COUNT]], lines2: &[[String; COL_COUNT]]) -> HashSet<(usize, usize)> {
    let mut diffs: HashSet<(usize, usize)> = HashSet::new();

    for (i, (line1, line2)) in std::iter::zip(lines1, lines2).enumerate() {
        for (j, (byte1, byte2)) in std::iter::zip(line1, line2).enumerate() {
            if byte1 == byte2 { continue }
            diffs.insert((i, j));
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
                Color::from_rgb(0.96, 0.97, 0.94)
            };

            row = row.push(
                text(byte.to_owned())
                    .style(color)
                    .font(Font::MONOSPACE)
                    .size(16)
            );
            if j < line.len() - 1 {
                row = row.push(
                    text(" ")
                        .font(Font::MONOSPACE)
                        .size(16)
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

