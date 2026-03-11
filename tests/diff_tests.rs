use rstest::rstest;
use words_to_data::{
    diff::TreeDiff,
    uslm::{TextContentField, parser::parse},
};

#[test]
fn test_diff_generation_26() {
    let doc_old = parse("tests/test_data/usc/2025-07-18/usc26.xml", "2025-07-18")
        .expect("Error running parser");
    let doc_new = parse("tests/test_data/usc/2025-07-30/usc26.xml", "2025-07-30")
        .expect("Error running parser");

    let diff = TreeDiff::from_elements(&doc_old, &doc_new);

    let s174a_diff = diff.find("uscodedocument_26/title_26/subtitle_A/chapter_1/subchapter_B/part_VI/section_174/subsection_a").expect("Section 174A has no changes, nor does its children!");
    let change = s174a_diff
        .changes
        .first()
        .expect("Change should be detected on Section 174(A)");
    assert_eq!(change.field_name, TextContentField::Chapeau);
    assert_eq!(change.changes.len(), 2);
    assert_eq!(
        change.old_value,
        "In the case of a taxpayer’s specified research or experimental expenditures for any taxable year—"
    );
    assert_eq!(
        change.new_value,
        "In the case of a taxpayer’s foreign research or experimental expenditures for any taxable year—"
    );
}

// Generate diffs across title pairs
#[rstest]
#[case("01")]
#[case("09")]
#[case("26")]
fn test_diff_generation_across_titles(#[case] title: &str) {
    let path1 = format!("tests/test_data/usc/2025-07-18/usc{}.xml", title);
    let path2 = format!("tests/test_data/usc/2025-07-30/usc{}.xml", title);

    let tree1 = parse(&path1, "2025-07-18")
        .unwrap_or_else(|_| panic!("Failed to parse {} from 2025-07-18", title));

    let tree2 = parse(&path2, "2025-07-30")
        .unwrap_or_else(|_| panic!("Failed to parse {} from 2025-07-30", title));

    // Generate diff
    let diff = TreeDiff::from_elements(&tree1, &tree2);

    // Verify diff was generated
    assert!(!diff.root_path.is_empty(), "Diff should have a root path");

    // The diff may or may not have changes depending on the title
    // Just verify the diff structure is valid
    assert_eq!(diff.root_path, tree1.data.path);
}
