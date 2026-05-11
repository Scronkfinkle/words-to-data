#!/usr/bin/env python3
"""Filter dataset_sorted.json to specific titles."""

import json
import argparse
from pathlib import Path


def filter_children(children: list, titles: set) -> list:
    """Keep only children whose number_value is in titles set."""
    return [
        child for child in children
        if child.get("data", {}).get("element_type") == "title"
        and child.get("data", {}).get("number_value") in titles
    ]


def main():
    parser = argparse.ArgumentParser(description="Filter dataset to specific titles")
    parser.add_argument("titles", nargs="+", help="Title numbers to keep (e.g., 1 5 18)")
    parser.add_argument("-i", "--input", default="dataset_sorted.json", help="Input JSON file")
    parser.add_argument("-o", "--output", default="dataset_filtered.json", help="Output JSON file")
    args = parser.parse_args()

    titles = set(args.titles)
    print(f"Filtering to titles: {sorted(titles)}")

    input_path = Path(args.input)
    print(f"Loading {input_path}...")
    with open(input_path) as f:
        data = json.load(f)

    # Filter each version's children
    for version in data.get("versions", []):
        element = version.get("element", {})
        if "children" in element:
            original_count = len(element["children"])
            element["children"] = filter_children(element["children"], titles)
            print(f"Kept {len(element['children'])}/{original_count} titles")

    output_path = Path(args.output)
    print(f"Writing {output_path}...")
    with open(output_path, "w") as f:
        json.dump(data, f, indent=2)

    print("Done.")


if __name__ == "__main__":
    main()
