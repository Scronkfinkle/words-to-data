from words_to_data import TreeDiff, compute_diff, parse_uslm_xml, USLMElement, parse_bill_amendments, AmendmentData, BillAmendment, UscReference

def test_uslm_elements():
    element = parse_uslm_xml("tests/test_data/usc/2025-07-30/usc26.xml", "2025-07-30")
    s174a = element.find("uscodedocument_26/title_26/subtitle_A/chapter_1/subchapter_B/part_VI/section_174/subsection_a")
    assert isinstance(element,USLMElement)
    assert isinstance(s174a,USLMElement)

def test_diffs():
    old = parse_uslm_xml('tests/test_data/usc/2025-07-18/usc26.xml', '2025-07-18')
    new = parse_uslm_xml('tests/test_data/usc/2025-07-30/usc26.xml', '2025-07-30')
    # Compute diff
    diff = compute_diff(old, new)
    s174a = diff.find("uscodedocument_26/title_26/subtitle_A/chapter_1/subchapter_B/part_VI/section_174/subsection_a")
    assert isinstance(s174a, TreeDiff)
    field_change = s174a.changes[0]
    assert len(field_change.changes) == 2
    assert field_change.field_name == "chapeau"
    assert field_change.old_value == "In the case of a taxpayer’s specified research or experimental expenditures for any taxable year—"
    assert field_change.new_value == "In the case of a taxpayer’s foreign research or experimental expenditures for any taxable year—"

def test_bill_parsing():
    # Parse bill amendments
    data = parse_bill_amendments("tests/test_data/bills/hr-119-21.xml")

    # Validate AmendmentData
    assert isinstance(data, AmendmentData)
    assert data.bill_id == "119-21"
    assert len(data.amendments) > 0

    # Validate BillAmendment
    amendment = data.amendments[0]
    assert isinstance(amendment, BillAmendment)
    assert amendment.source_path.startswith("/us/pl/")
    assert len(amendment.target_paths) > 0
    assert len(amendment.action_types) > 0

    # Validate UscReference
    ref = amendment.target_paths[0]
    assert isinstance(ref, UscReference)
    assert ref.path.startswith("/us/usc/")
    assert len(ref.display_text) > 0

    # Validate action types are valid strings
    valid_actions = ["amend", "add", "delete", "insert", "redesignate", "repeal"]
    assert all(action in valid_actions for action in amendment.action_types)
