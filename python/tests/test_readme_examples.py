"""
README Example Tests (Python)

These tests replicate the exact Python examples shown in README.md.
If any of these tests fail, the README examples need to be updated.
"""

import tempfile
import os
from words_to_data import Dataset, DatasetMetadata, parse_bill_amendments

PL_XML_PATH = "tests/test_data/congress_client_cache/bill/119/hr/1/public_law.xml"


def test_readme_example_dataset_workflow():
    """
    Tests the Dataset Workflow example from README.md Quick Start section.
    This is the primary example showing the full workflow.
    """
    # -- Exact code from README (with real test paths) --

    metadata = DatasetMetadata(
        name="Tax Code Changes",
        description="Tracking Title 26 changes",
        author="Author",
        source_urls=[],
        license="MIT",
        version="1.0.0",
    )
    dataset = Dataset(metadata)

    # Add document versions
    dataset.add_uslm_xml(
        "tests/test_data/usc/2025-07-18/usc26.xml", "2025-07-18", label="Before"
    )
    dataset.add_uslm_xml(
        "tests/test_data/usc/2025-07-30/usc26.xml", "2025-07-30", label="After"
    )

    # Add bill
    bill = parse_bill_amendments(
        "119-21", PL_XML_PATH
    )
    dataset.add_bill(bill)

    # Compute diff
    diff = dataset.compute_diff("2025-07-18", "2025-07-30")

    # Navigate to specific section
    s174a = diff.find(
        "uscode/title_26/subtitle_A/chapter_1/subchapter_B/part_VI/section_174/subsection_a"
    )
    if s174a:
        for change in s174a.changes:
            # README shows: print(f"{change.field_name}: {change.old_value} → {change.new_value}")
            _output = f"{change.field_name}: {change.old_value} → {change.new_value}"
    else:
        raise AssertionError(
            "Section 174(a) not found in diff - README example path may be wrong"
        )

    # Save dataset (to temp file)
    with tempfile.NamedTemporaryFile(suffix=".json", delete=False) as f:
        temp_path = f.name

    try:
        dataset.save(temp_path)
    finally:
        os.unlink(temp_path)


def test_readme_example_dataset_workflow_results():
    """
    Verifies the Dataset example produces expected results.
    """
    metadata = DatasetMetadata(
        name="Tax Code Changes",
        description="Tracking Title 26 changes",
        author="Author",
        source_urls=[],
        license="MIT",
        version="1.0.0",
    )
    dataset = Dataset(metadata)

    dataset.add_uslm_xml(
        "tests/test_data/usc/2025-07-18/usc26.xml", "2025-07-18", label="Before"
    )
    dataset.add_uslm_xml(
        "tests/test_data/usc/2025-07-30/usc26.xml", "2025-07-30", label="After"
    )

    bill = parse_bill_amendments(
        "119-21", PL_XML_PATH
    )
    dataset.add_bill(bill)

    # Verify versions added
    assert len(dataset.versions) == 2, "README shows 2 versions"

    # Verify bill added
    assert len(dataset.bills) == 1, "README shows 1 bill"

    # Verify diff works and has changes
    diff = dataset.compute_diff("2025-07-18", "2025-07-30")
    s174a = diff.find(
        "uscode/title_26/subtitle_A/chapter_1/subchapter_B/part_VI/section_174/subsection_a"
    )
    assert s174a is not None, "Section should exist"
    assert len(s174a.changes) > 0, "Section 174(a) should have changes"

    # Verify change has expected fields (as shown in README output)
    change = s174a.changes[0]
    assert len(change.old_value) > 0
    assert len(change.new_value) > 0
