use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockType {
    Empty,
    Cobblestone,
    Glass,
    Dirt,
}

pub struct Layer {
    pub level: usize,
    pub grid: Vec<Vec<BlockType>>,
}

pub struct Structure {
    pub layers: Vec<Layer>,
}

impl Structure {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let mut layers = Vec::new();
        let sections: Vec<&str> = content.split("---").collect();

        for section in sections {
            let lines: Vec<&str> = section.trim().lines().collect();
            if lines.is_empty() {
                continue;
            }

            let level = lines[0].trim().parse::<usize>()
                .map_err(|_| format!("Invalid level number: {}", lines[0]))?;

            let mut grid = Vec::new();
            for line in &lines[1..] {
                let row: Vec<BlockType> = line.chars()
                    .map(|c| match c {
                        'm' => BlockType::Empty,
                        'C' => BlockType::Cobblestone,
                        'V' => BlockType::Glass,
                        'T' => BlockType::Dirt,
                        _ => BlockType::Empty,
                    })
                    .collect();

                if !row.is_empty() {
                    grid.push(row);
                }
            }

            if !grid.is_empty() {
                layers.push(Layer { level, grid });
            }
        }

        Ok(Structure { layers })
    }

    pub fn width(&self) -> usize {
        self.layers.first()
            .and_then(|layer| layer.grid.first())
            .map(|row| row.len())
            .unwrap_or(0)
    }

    pub fn depth(&self) -> usize {
        self.layers.first()
            .map(|layer| layer.grid.len())
            .unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.layers.len()
    }
}
