from words_to_data import TreeDiff, compute_diff, parse_uslm_xml, USLMElement

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
