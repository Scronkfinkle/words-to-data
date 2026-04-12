use words_to_data::{diff::TreeDiff, uslm::parser::parse};

#[test]
fn test_correct_matching_regex() {
    let result_a = parse("tests/test_data/usc/2025-07-18/usc26.xml", "2025-07-18")
        .expect("unable to parse doc");
    let result_b = parse("tests/test_data/usc/2025-07-30/usc26.xml", "2025-07-30")
        .expect("unable to parse doc");

    let diff: TreeDiff = TreeDiff::from_elements(&result_a, &result_b);

    let s174a = diff.find("uscodedocument_26/title_26/subtitle_A/chapter_1/subchapter_B/part_VI/section_174/subsection_a/paragraph_2").unwrap();
    let mention_regex = s174a.mention_regex().unwrap();
    let target = "According to Section 174 (a)(2) blah blah";
    let mat = mention_regex.find(target).unwrap();
    assert_eq!(mat.as_str(), "Section 174 (a)(2) ");
}

#[test]
fn test_get_all_regexes() {
    let result_a = parse("tests/test_data/usc/2025-07-18/usc26.xml", "2025-07-18")
        .expect("unable to parse doc");
    let result_b = parse("tests/test_data/usc/2025-07-30/usc26.xml", "2025-07-30")
        .expect("unable to parse doc");
    let diff: TreeDiff = TreeDiff::from_elements(&result_a, &result_b);
    let regs = diff.mention_regexes();
    assert_eq!(regs.len(), 819);
}
