#!/usr/bin/env python3
"""
Filter a Dataset JSON to only include specific titles.

Usage:
    python filter_titles.py input.json output.json --titles 7 26
"""

import argparse
import json
import sys
from pathlib import Path


def path_matches_titles(path: str, titles: set[str]) -> bool:
    """Check if path belongs to one of the specified titles."""
    if not path:
        return False
    parts = path.split("/")
    for i, part in enumerate(parts):
        if part.startswith("title_"):
            title_num = part.replace("title_", "")
            return title_num in titles
    return False


def filter_element(element: dict, titles: set[str]) -> dict | None:
    """Recursively filter element tree to only include matching titles."""
    data = element.get("data", element)
    path = data.get("path", "")

    # Check if this element or any ancestor matches
    if not path_matches_titles(path, titles):
        # Check children - maybe a child matches
        children = element.get("children", [])
        filtered_children = []
        for child in children:
            filtered = filter_element(child, titles)
            if filtered:
                filtered_children.append(filtered)

        if filtered_children:
            # Return element with only matching children
            result = dict(element)
            result["children"] = filtered_children
            return result
        return None

    # This element matches - include it with all children
    return element


def filter_dataset(dataset: dict, titles: set[str]) -> dict:
    """Filter entire dataset to only include specified titles."""

    # Filter versions
    filtered_versions = []
    for version in dataset.get("versions", []):
        element = version.get("element", {})
        filtered_element = filter_element(element, titles)
        if filtered_element:
            filtered_version = dict(version)
            filtered_version["element"] = filtered_element
            filtered_versions.append(filtered_version)

    # Collect all paths in filtered versions
    def collect_paths(el, paths):
        data = el.get("data", el)
        if "path" in data:
            paths.add(data["path"])
        for child in el.get("children", []):
            collect_paths(child, paths)

    all_paths = set()
    for v in filtered_versions:
        collect_paths(v.get("element", {}), all_paths)

    # Filter diff_annotations - handle both tuple array and object formats
    annotations = dataset.get("diff_annotations", {})
    filtered_annotations = []

    if isinstance(annotations, list):
        # Array of tuples format
        for item in annotations:
            if isinstance(item, (list, tuple)) and len(item) == 2:
                key, anns = item
                filtered_anns = [
                    a for a in anns
                    if any(p in all_paths or path_matches_titles(p, titles)
                           for p in a.get("paths", []))
                ]
                if filtered_anns:
                    filtered_annotations.append([key, filtered_anns])
    else:
        # Object format
        for key, anns in annotations.items():
            filtered_anns = [
                a for a in anns
                if any(p in all_paths or path_matches_titles(p, titles)
                       for p in a.get("paths", []))
            ]
            if filtered_anns:
                filtered_annotations.append([key, filtered_anns])

    # Collect referenced bill IDs
    bill_ids = set()
    for item in filtered_annotations:
        _, anns = item
        for a in anns:
            sb = a.get("source_bill", {})
            if sb.get("bill_id"):
                bill_ids.add(sb["bill_id"])

    # Filter bills
    filtered_bills = [
        b for b in dataset.get("bills", [])
        if b.get("bill_id") in bill_ids
    ]

    # Collect referenced member IDs from bills
    member_ids = set()
    for b in filtered_bills:
        if b.get("sponsor"):
            member_ids.add(b["sponsor"])
        for cs in b.get("cosponsors", []):
            if isinstance(cs, dict):
                member_ids.add(cs.get("bioguide_id"))
            else:
                member_ids.add(cs)

    # Filter members - handle both tuple array and object formats
    members = dataset.get("members", {})
    if isinstance(members, list):
        filtered_members = [
            [k, v] for k, v in members
            if k in member_ids
        ]
    else:
        filtered_members = [
            [k, v] for k, v in members.items()
            if k in member_ids
        ]

    # Filter sponsors
    sponsors = dataset.get("sponsors", {})
    if isinstance(sponsors, list):
        filtered_sponsors = [
            [k, v] for k, v in sponsors
            if k in bill_ids
        ]
    else:
        filtered_sponsors = [
            [k, v] for k, v in sponsors.items()
            if k in bill_ids
        ]

    return {
        "metadata": dataset.get("metadata", {}),
        "versions": filtered_versions,
        "bills": filtered_bills,
        "diff_annotations": filtered_annotations,
        "members": filtered_members,
        "sponsors": filtered_sponsors,
    }


def main():
    parser = argparse.ArgumentParser(description="Filter dataset to specific titles")
    parser.add_argument("input", help="Input JSON file")
    parser.add_argument("output", help="Output JSON file")
    parser.add_argument("--titles", nargs="+", required=True, help="Title numbers to keep (e.g., 7 26)")
    args = parser.parse_args()

    titles = set(args.titles)
    print(f"Filtering to titles: {titles}")

    print(f"Loading {args.input}...")
    with open(args.input) as f:
        dataset = json.load(f)

    print(f"Input: {len(dataset.get('versions', []))} versions, {len(dataset.get('bills', []))} bills")

    filtered = filter_dataset(dataset, titles)

    print(f"Output: {len(filtered['versions'])} versions, {len(filtered['bills'])} bills")

    print(f"Writing {args.output}...")
    with open(args.output, "w") as f:
        json.dump(filtered, f, separators=(",", ":"))

    input_size = Path(args.input).stat().st_size
    output_size = Path(args.output).stat().st_size
    print(f"Size: {input_size/1e6:.1f}MB -> {output_size/1e6:.1f}MB ({100*output_size/input_size:.1f}%)")


if __name__ == "__main__":
    main()
