use serde::{Deserialize, Serialize};
use std::fs;
use std::process;
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[command(author, version, about = "Add automatic lighting to Beat Saber beatmaps")]
struct Args {
    #[arg(short, long, help = "Input beatmap file")]
    input: String,
    
    #[arg(short, long, help = "Output beatmap file")]
    output: String,
    
    #[arg(short, long, help = "Force overwrite existing lighting events")]
    force: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct BeatMap {
    _version: String,
    _notes: Vec<Note>,
    #[serde(default)]
    _events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Note {
    _time: f64,
    _type: i32,
    _cutDirection: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Event {
    _time: f64,
    _type: i32,
    _value: i32,
}

struct NoteWithPadding {
    raw: Note,
    padding: f64,
}

impl NoteWithPadding {
    fn new(note: Note, next_note: Option<&Note>) -> Self {
        let padding = match next_note {
            Some(next) => next._time - note._time,
            None => note._time + 1.0 - note._time,
        };
        Self { raw: note, padding }
    }
}

fn calculate_laser_speed(padding: f64) -> i32 {
    ((((2.0 / padding) + 1.0) as i32).pow(2) / 4) as i32
}

fn add_lighting_events(mut beatmap: BeatMap) -> BeatMap {
    let mut last_padding = 0.0;
    let mut last_time = None;
    let mut left_laser_next = true;
    let mut pace_changes = Vec::new();
    beatmap._events = Vec::new();

    for i in 0..beatmap._notes.len() {
        // Find next note that isn't on the same beat
        let mut next_note = None;
        let mut n = i;
        let mut added_ring_rotation = false;
        let mut double_lasers = false;

        while next_note.is_none() {
            n += 1;
            if n >= beatmap._notes.len() {
                next_note = Some(Note {
                    _time: beatmap._notes[n-1]._time * 2.0,
                    _type: 0,
                    _cutDirection: 0,
                });
                break;
            }

            let next_up = &beatmap._notes[n];
            if (next_up._time - beatmap._notes[i]._time).abs() < f64::EPSILON {
                if !added_ring_rotation {
                    beatmap._events.push(Event {
                        _time: beatmap._notes[i]._time,
                        _type: 8,
                        _value: 0,
                    });
                    added_ring_rotation = true;
                }
                double_lasers = true;
                continue;
            }
            next_note = Some(next_up.clone());
        }

        // Skip stacked events
        if let Some(last) = last_time {
            if ((last - beatmap._notes[i]._time) as f64).abs() < f64::EPSILON {
                continue;
            }
        }

        let note = NoteWithPadding::new(beatmap._notes[i].clone(), next_note.as_ref());
        let mut light_value = None;
        let mut light_type = None;
        let mut pace_prefix = None;

        // Determine lighting effects based on note type and timing
        if note.raw._cutDirection == 8 || note.raw._type == 3 {
            beatmap._events.push(Event {
                _time: note.raw._time,
                _type: 0,
                _value: if note.padding < 1.0 { 6 } else { 2 },
            });
            beatmap._events.push(Event {
                _time: note.raw._time,
                _type: 4,
                _value: 0,
            });
            if note.raw._type == 3 {
                continue;
            }
        } else if note.padding >= 2.0 {
            if last_padding < 2.0 || i < 1 {
                beatmap._events.push(Event {
                    _time: note.raw._time,
                    _type: 9,
                    _value: 0,
                });
                pace_prefix = Some("0");
            }
            light_type = Some(4);
            light_value = Some(3);
        } else if note.padding >= 1.0 {
            if last_padding < 1.0 || last_padding >= 2.0 || i < 1 {
                beatmap._events.push(Event {
                    _time: note.raw._time,
                    _type: 9,
                    _value: 0,
                });
                pace_prefix = Some("a");
            }
            light_type = Some(4);
            light_value = Some(2);
        } else {
            if last_padding >= 1.0 || i < 1 {
                beatmap._events.push(Event {
                    _time: note.raw._time,
                    _type: 9,
                    _value: 0,
                });
                pace_prefix = Some("b");
            }
            light_type = Some(4);
            light_value = Some(6);
        }

        if let Some(prefix) = pace_prefix {
            pace_changes.push(format!("{}{}", prefix, note.raw._time));
        }

        if note.raw._cutDirection != 8 {
            if let (Some(lt), Some(lv)) = (light_type, light_value) {
                beatmap._events.push(Event {
                    _time: note.raw._time,
                    _type: lt,
                    _value: lv,
                });
                beatmap._events.push(Event {
                    _time: note.raw._time,
                    _type: 0,
                    _value: 0,
                });
            }
        }

        // Handle laser effects
        let laser_color = if note.padding < 1.0 { 7 } else { 3 };
        let mut laser_side = None;

        if double_lasers && note.padding >= 2.0 {
            beatmap._events.extend_from_slice(&[
                Event {
                    _time: note.raw._time,
                    _type: 3,
                    _value: laser_color,
                },
                Event {
                    _time: note.raw._time,
                    _type: 2,
                    _value: laser_color,
                },
                Event {
                    _time: note.raw._time,
                    _type: 12,
                    _value: calculate_laser_speed(note.padding),
                },
                Event {
                    _time: note.raw._time,
                    _type: 13,
                    _value: calculate_laser_speed(note.padding),
                },
            ]);
        } else if left_laser_next {
            left_laser_next = false;
            laser_side = Some(2);
            beatmap._events.push(Event {
                _time: note.raw._time,
                _type: 3,
                _value: 0,
            });
        } else {
            left_laser_next = true;
            laser_side = Some(3);
            beatmap._events.push(Event {
                _time: note.raw._time,
                _type: 2,
                _value: 0,
            });
        }

        if !double_lasers || note.padding < 2.0 {
            if let Some(side) = laser_side {
                beatmap._events.extend_from_slice(&[
                    Event {
                        _time: note.raw._time,
                        _type: if side == 2 { 12 } else { 13 },
                        _value: calculate_laser_speed(note.padding),
                    },
                    Event {
                        _time: note.raw._time,
                        _type: side,
                        _value: laser_color,
                    },
                ]);
            }
        }

        last_padding = note.padding;
        last_time = Some(note.raw._time);
    }

    // Add ring lights for paced sections
    for i in 0..pace_changes.len() {
        let mut ring_value = 0;
        
        if pace_changes[i].is_empty() {
            continue;
        }

        let prefix = pace_changes[i].chars().next().unwrap();
        
        ring_value = match prefix {
            'a' => 3,
            'b' => 7,
            _ => 0,
        };

        if ring_value == 0 || i == pace_changes.len() - 1 {
            continue;
        }

        if let (Ok(current_timestamp), Ok(next_timestamp)) = (
            pace_changes[i][1..].parse::<f64>(),
            pace_changes[i + 1][1..].parse::<f64>(),
        ) {
            let current_int = current_timestamp as i32;
            if (current_timestamp - (current_int as f64)).abs() > f64::EPSILON {
                beatmap._events.push(Event {
                    _time: current_timestamp,
                    _type: 1,
                    _value: ring_value,
                });
            }

            let mut current = current_int;
            while (current as f64) < next_timestamp {
                beatmap._events.push(Event {
                    _time: current as f64,
                    _type: 1,
                    _value: ring_value,
                });
                current += 1;
            }
        }
    }

    beatmap
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read input beatmap
    let beatmap_str = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {}", args.input))?;
    
    let mut beatmap: BeatMap = serde_json::from_str(&beatmap_str)
        .with_context(|| "Failed to parse beatmap JSON")?;

    // Validate beatmap
    if !beatmap._events.is_empty() && !args.force {
        eprintln!("Beatmap already contains lighting events! Use --force to overwrite.");
        process::exit(1);
    }

    // Add lighting events
    beatmap = add_lighting_events(beatmap);

    // Write output beatmap
    let output_json = serde_json::to_string(&beatmap)
        .with_context(|| "Failed to serialize beatmap to JSON")?;
    
    fs::write(&args.output, output_json)
        .with_context(|| format!("Failed to write output file: {}", args.output))?;

    println!("Successfully added lighting to {} and saved to {}", args.input, args.output);
    Ok(())
}