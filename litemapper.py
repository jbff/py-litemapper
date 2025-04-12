#!/usr/bin/env python3

import json
import argparse
import math
from typing import Dict, List, Any

class Note:
    def __init__(self, note: Dict[str, Any], next_note: Dict[str, Any] = None):
        self.raw = note
        self.padding = (next_note["_time"] if next_note else note["_time"] + 1) - note["_time"]

def calculate_laser_speed(padding: float) -> int:
    return math.ceil((math.ceil((2 / padding) + 1) ** 2) / 4)

def add_lighting_events(beatmap: Dict[str, Any]) -> Dict[str, Any]:
    last_padding = 0
    last_time = None
    left_laser_next = True
    pace_changes = []
    beatmap["_events"] = []

    for i in range(len(beatmap["_notes"])):
        # Find next note that isn't on the same beat
        next_note = None
        n = i
        added_ring_rotation = False
        double_lasers = False
        
        while next_note is None:
            n += 1
            if n >= len(beatmap["_notes"]):
                next_note = {"_time": beatmap["_notes"][n-1]["_time"] * 2}
                break
            
            next_up = beatmap["_notes"][n]
            if next_up["_time"] == beatmap["_notes"][i]["_time"]:
                if not added_ring_rotation:
                    beatmap["_events"].append({
                        "_time": beatmap["_notes"][i]["_time"],
                        "_type": 8,
                        "_value": 0
                    })
                    added_ring_rotation = True
                double_lasers = True
                continue
            next_note = next_up

        # Skip stacked events
        if last_time == beatmap["_notes"][i]["_time"]:
            continue

        note = Note(beatmap["_notes"][i], next_note)
        light_value = None
        light_type = None
        pace_prefix = None

        # Determine lighting effects based on note type and timing
        if note.raw["_cutDirection"] == 8 or note.raw["_type"] == 3:
            # Add back light effects for bombs or blocks cut in any direction
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 0,
                "_value": 6 if note.padding < 1 else 2
            })
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 4,
                "_value": 0
            })
            if note.raw["_type"] == 3:  # Skip if bomb
                continue
        elif note.padding >= 2:
            if last_padding < 2 or i < 1:
                beatmap["_events"].append({
                    "_time": note.raw["_time"],
                    "_type": 9,
                    "_value": 0
                })
                pace_prefix = "0"
            light_type = 4
            light_value = 3
        elif note.padding >= 1:
            if last_padding < 1 or last_padding >= 2 or i < 1:
                beatmap["_events"].append({
                    "_time": note.raw["_time"],
                    "_type": 9,
                    "_value": 0
                })
                pace_prefix = "a"
            light_type = 4
            light_value = 2
        else:
            if last_padding >= 1 or i < 1:
                beatmap["_events"].append({
                    "_time": note.raw["_time"],
                    "_type": 9,
                    "_value": 0
                })
                pace_prefix = "b"
            light_type = 4
            light_value = 6

        if pace_prefix is not None:
            pace_changes.append(f"{pace_prefix}{note.raw['_time']}")

        if note.raw["_cutDirection"] != 8:
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": light_type,
                "_value": light_value
            })
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 0,
                "_value": 0
            })

        # Handle laser effects
        laser_color = 7 if note.padding < 1 else 3
        laser_side = None

        if double_lasers and note.padding >= 2:
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 3,
                "_value": laser_color
            })
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 2,
                "_value": laser_color
            })
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 12,
                "_value": calculate_laser_speed(note.padding)
            })
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 13,
                "_value": calculate_laser_speed(note.padding)
            })
        elif left_laser_next:
            left_laser_next = False
            laser_side = 2
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 3,
                "_value": 0
            })
        else:
            left_laser_next = True
            laser_side = 3
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 2,
                "_value": 0
            })

        if not double_lasers or note.padding < 2:
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": 12 if laser_side == 2 else 13,
                "_value": calculate_laser_speed(note.padding)
            })
            beatmap["_events"].append({
                "_time": note.raw["_time"],
                "_type": laser_side,
                "_value": laser_color
            })

        last_padding = note.padding
        last_time = note.raw["_time"]

    # Add ring lights for paced sections
    for i in range(len(pace_changes)):
        ring_value = 0
        # Skip empty strings or strings without at least one character
        if not pace_changes[i] or len(pace_changes[i]) < 1:
            continue
            
        prefix = pace_changes[i][0]
        
        if prefix == "a":
            ring_value = 3
        elif prefix == "b":
            ring_value = 7
        
        if ring_value == 0 or i == len(pace_changes) - 1:
            continue


        # Safely parse the current timestamp
        try:
            current_timestamp = math.ceil(float(pace_changes[i][1:]))
            next_timestamp = math.ceil(float(pace_changes[i+1][1:]))
        except (ValueError, IndexError) as e:
            continue  # Skip this pace change if there's an error parsing timestamps
        # Get the original timestamp as float for precise comparison
        original_timestamp = float(pace_changes[i][1:])
        if math.ceil(original_timestamp) != original_timestamp:
            beatmap["_events"].append({
                "_time": original_timestamp,
                "_type": 1,
                "_value": ring_value
            })

        while current_timestamp < next_timestamp:
            beatmap["_events"].append({
                "_time": current_timestamp,
                "_type": 1,
                "_value": ring_value
            })
            current_timestamp += 1

    return beatmap

def main():
    parser = argparse.ArgumentParser(description='Add automatic lighting to Beat Saber beatmaps')
    parser.add_argument('-i', '--input', required=True, help='Input beatmap file')
    parser.add_argument('-o', '--output', required=True, help='Output beatmap file')
    parser.add_argument('-f', '--force', action='store_true', help='Force overwrite existing lighting events')
    args = parser.parse_args()

    try:
        # Read input beatmap
        with open(args.input, 'r') as f:
            beatmap = json.load(f)

        # Validate beatmap
        if "_version" not in beatmap:
            raise ValueError("Invalid beatmap version! V3 mapping is not supported yet!")
        if "_notes" not in beatmap:
            raise ValueError("Not a valid beatmap!")
        if "_events" in beatmap and beatmap["_events"] and len(beatmap["_events"]) > 0 and not args.force:
            raise ValueError("Beatmap already contains lighting events! Use --force to overwrite.")

        # Add lighting events
        beatmap = add_lighting_events(beatmap)

        # Write output beatmap
        with open(args.output, 'w') as f:
            json.dump(beatmap, f)

        print(f"Successfully added lighting to {args.input} and saved to {args.output}")

    except Exception as e:
        print(f"Error: {str(e)}")
        exit(1)

if __name__ == "__main__":
    main()
