use std::fs::File;
use std::collections::HashMap;
use flate2::read::GzDecoder;

#[derive(Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block_type: String,
}

#[derive(Debug)]
pub struct Schematic {
    pub width: i16,
    pub height: i16,
    pub length: i16,
    pub blocks: Vec<Block>,
}

impl Schematic {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let decoder = GzDecoder::new(file);

        let value: fastnbt::Value = fastnbt::from_reader(decoder)?;

        let schematic_data = if let fastnbt::Value::Compound(map) = &value {
            if let Some(schematic) = map.get("Schematic") {
                schematic
            } else {
                &value
            }
        } else {
            &value
        };


        let width = Self::get_i16(schematic_data, "Width")
            .or_else(|| Self::get_i16(schematic_data, "width"))
            .ok_or("Width field not found")?;
        let height = Self::get_i16(schematic_data, "Height")
            .or_else(|| Self::get_i16(schematic_data, "height"))
            .ok_or("Height field not found")?;
        let length = Self::get_i16(schematic_data, "Length")
            .or_else(|| Self::get_i16(schematic_data, "length"))
            .ok_or("Length field not found")?;

        let mut blocks = Vec::new();

        if let fastnbt::Value::Compound(map) = schematic_data {
            if let Some(fastnbt::Value::Compound(blocks_compound)) = map.get("Blocks") {
                let mut palette_map: HashMap<i32, String> = HashMap::new();

                if let Some(fastnbt::Value::Compound(palette)) = blocks_compound.get("Palette") {
                    for (block_name, id_value) in palette {
                        if let Some(block_id) = match id_value {
                            fastnbt::Value::Int(n) => Some(*n),
                            fastnbt::Value::Short(n) => Some(*n as i32),
                            _ => None,
                        } {
                            palette_map.insert(block_id, block_name.clone());
                        }
                    }
                    println!("  Paleta cargada con {} tipos de bloques", palette_map.len());
                }

                if let Some(data_value) = blocks_compound.get("Data") {
                    let block_data: Vec<i32> = match data_value {
                        fastnbt::Value::ByteArray(arr) => arr.iter().map(|&b| b as i32).collect(),
                        fastnbt::Value::IntArray(arr) => arr.to_vec(),
                        _ => Vec::new(),
                    };

                    let mut index = 0;
                    for y in 0..height {
                        for z in 0..length {
                            for x in 0..width {
                                if index < block_data.len() {
                                    let palette_id = block_data[index];
                                    if let Some(block_type) = palette_map.get(&palette_id) {
                                        if !block_type.contains("air") {
                                            blocks.push(Block {
                                                x: x as i32,
                                                y: y as i32,
                                                z: z as i32,
                                                block_type: block_type.clone(),
                                            });
                                        }
                                    }
                                    index += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Schematic {
            width,
            height,
            length,
            blocks,
        })
    }

    fn get_i32_from_compound(map: &HashMap<String, fastnbt::Value>, key: &str) -> Option<i32> {
        map.get(key).and_then(|v| match v {
            fastnbt::Value::Int(n) => Some(*n),
            fastnbt::Value::Short(n) => Some(*n as i32),
            fastnbt::Value::Byte(n) => Some(*n as i32),
            _ => None,
        })
    }

    fn get_string_from_compound(map: &HashMap<String, fastnbt::Value>, key: &str) -> Option<String> {
        map.get(key).and_then(|v| match v {
            fastnbt::Value::String(s) => Some(s.clone()),
            _ => None,
        })
    }

    fn get_i16(value: &fastnbt::Value, key: &str) -> Option<i16> {
        if let fastnbt::Value::Compound(map) = value {
            map.get(key).and_then(|v| match v {
                fastnbt::Value::Short(n) => Some(*n),
                fastnbt::Value::Int(n) => Some(*n as i16),
                _ => None,
            })
        } else {
            None
        }
    }
}
