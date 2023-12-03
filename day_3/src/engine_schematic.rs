use std::{rc::Rc, cell::RefCell};

use crate::{engine_schematic_symbol::EngineSchematicSymbol, maybe_part::MaybePart};

pub struct EngineSchematic {
    all_maybe_parts: Vec<Rc<RefCell<MaybePart>>>,
    schematic: Vec<Vec<EngineSchematicSymbol>>,
}

impl EngineSchematic {
    pub fn from_raw_schematic(raw_schematic: &str) -> Self {
        let mut all_maybe_parts = vec![];
        let mut width: usize = 0;
        let mut schematic = raw_schematic.lines().map(|line| {
            let mut schematic_line = vec![];
            let mut maybe_part_start = None;

            for (i, char) in line.chars().enumerate() {
                width = i;
                if char.is_ascii_digit() {
                    if maybe_part_start.is_none() {
                        maybe_part_start = Some(i)
                    }
                    continue;
                }

                if let Some(start) = maybe_part_start {
                    let maybe_part_number = line[start..i].parse::<u32>().expect("Could not parse maybe_part_number");
                    let maybe_part = Rc::new(RefCell::new(MaybePart::new(maybe_part_number)));
                    (start..i).for_each(|_| schematic_line.push(EngineSchematicSymbol::PartRef(maybe_part.clone())));
                    all_maybe_parts.push(maybe_part);
                    maybe_part_start = None;
                }

                if char == '.' {
                    schematic_line.push(EngineSchematicSymbol::Empty)
                }
                else {
                    schematic_line.push(EngineSchematicSymbol::Symbol(char))
                }
            }
            if let Some(start) = maybe_part_start {
                let maybe_part_number = line[start..].parse::<u32>().expect("Could not parse maybe_part_number");
                let maybe_part = Rc::new(RefCell::new(MaybePart::new(maybe_part_number)));
                (start..line.len()).for_each(|_| schematic_line.push(EngineSchematicSymbol::PartRef(maybe_part.clone())));
                all_maybe_parts.push(maybe_part);
            }
            schematic_line
        }).collect();

        Self::mark_all_real_parts_and_set_gears(&mut schematic, width);

        Self {
            all_maybe_parts,
            schematic,
        }
    }

    fn mark_all_real_parts_and_set_gears(schematic: &mut Vec<Vec<EngineSchematicSymbol>>, width: usize ) {
        let height = schematic.len();
        let mut gears = vec![];
        for (y, line) in schematic.iter().enumerate() {
            for (x, symbol) in line.iter().enumerate() {
                if let EngineSchematicSymbol::Symbol(s) = symbol {
                    const DIRECTIONS: [(i32, i32); 8] = [
                        (1, -1),
                        (1, 0),
                        (1, 1),
                        (0, 1),
                        (-1, 1),
                        (-1, 0),
                        (-1, -1),
                        (0, -1),
                    ];
                    let mut unique_adjacent_parts: Vec<&Rc<RefCell<MaybePart>>> = vec![];
                    for direction in DIRECTIONS {
                        let y = (y as i32) + direction.0;
                        let x = (x as i32) + direction.1;
                        if x < 0 || x >= (width as i32) {
                            continue;
                        }
                        if y < 0 || y >= (height as i32) {
                            continue;
                        }
                        if let EngineSchematicSymbol::PartRef(part_ref) = &schematic[y as usize][x as usize] {
                            if let None = unique_adjacent_parts.iter().find(|other_part_ref| {
                                std::ptr::eq(part_ref.as_ref(), other_part_ref.as_ref())
                            }) {
                                unique_adjacent_parts.push(part_ref);
                            }
                            part_ref.as_ref().borrow_mut().mark_as_real();
                        }
                    }
                    if unique_adjacent_parts.len() == 2 && *s == '*' {
                        let part_0 = unique_adjacent_parts[0].as_ref().borrow().get_number();
                        let part_1 = unique_adjacent_parts[1].as_ref().borrow().get_number();
                        let gear_ratio = part_0 * part_1;
                        gears.push((y, x, gear_ratio));
                    }
                }
            }
        }
        for (y, x, gear_ratio) in gears {
            schematic[y][x] = EngineSchematicSymbol::Gear(gear_ratio);
        }
    }

    pub fn get_real_parts(&self) -> Vec<u32> {
        self.all_maybe_parts.iter().filter_map(|maybe_part| {
            let maybe_part = maybe_part.borrow();
            if maybe_part.is_marked_as_real() {
                Some(maybe_part.get_number())
            } else {
                None
            }
        }).collect()
    }

    pub fn get_gear_ratios(&self) -> Vec<u32> {
        self.schematic.iter().flatten().filter_map(|symbol| {
            if let EngineSchematicSymbol::Gear(gr) = *symbol {
                Some(gr)
            } else {
                None
            }
        }).collect()
    }
}

impl std::fmt::Display for EngineSchematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        for line in &self.schematic {
            for symbol in line {
                let symbol = match symbol {
                    EngineSchematicSymbol::Empty => '.',
                    EngineSchematicSymbol::PartRef(_) => 'P',
                    EngineSchematicSymbol::Symbol(_) => 'S',
                    EngineSchematicSymbol::Gear(_) => 'G',
                };
                text.push(symbol);
            }
            text.push('\n');
        }
        write!(f, "{}", text)
    }
}
