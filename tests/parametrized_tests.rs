use rstest::rstest;
use words_to_data::diff::TreeDiff;
use words_to_data::uslm::{DocumentType, USCType, parser::parse};

// Test 1: Validate USLM ID format across multiple titles
#[rstest]
#[case("01", "/us/usc/t1")]
#[case("04", "/us/usc/t4")]
#[case("09", "/us/usc/t9")]
fn test_parse_uslm_id_format(#[case] title: &str, #[case] expected_id: &str) {
    let path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", title);
    let root =
        parse(&path, "2025-07-18").unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", title));

    assert_eq!(
        root.data.uslm_id.as_ref().unwrap(),
        expected_id,
        "Title {} should have USLM ID {}",
        title,
        expected_id
    );
}

// Test 2: Verify USC document type across titles
#[rstest]
#[case("01")]
#[case("04")]
#[case("09")]
fn test_parse_document_type(#[case] title: &str) {
    let path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", title);
    let root =
        parse(&path, "2025-07-18").unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", title));

    match &root.data.document_type {
        DocumentType::USCode { usc_type } => {
            assert_eq!(*usc_type, USCType::Title, "Should be USC Title type");
        }
        _ => panic!("Expected USCode document type"),
    }
}

// Test 3: Test hierarchy depth variation across titles
#[rstest]
#[case("01", 3)] // Has chapters
#[case("04", 3)] // Has chapters
#[case("09", 3)] // Has chapters
#[case("17", 3)] // Has chapters
fn test_hierarchy_depth_variation(#[case] title: &str, #[case] min_depth: usize) {
    let path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", title);
    let root =
        parse(&path, "2025-07-18").unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", title));

    // Calculate max depth of tree
    fn max_depth(elem: &words_to_data::uslm::USLMElement) -> usize {
        if elem.children.is_empty() {
            1
        } else {
            1 + elem.children.iter().map(max_depth).max().unwrap_or(0)
        }
    }

    let depth = max_depth(&root);
    assert!(
        depth >= min_depth,
        "Title {} should have depth >= {} but got {}",
        title,
        min_depth,
        depth
    );
}

// Test 4: Element count consistency (ensure parsing is complete)
#[rstest]
#[case("01", 10)] // At least 10 elements
#[case("04", 10)]
#[case("09", 10)]
fn test_element_count_consistency(#[case] title: &str, #[case] min_count: usize) {
    let path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", title);
    let root =
        parse(&path, "2025-07-18").unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", title));

    fn count_elements(elem: &words_to_data::uslm::USLMElement) -> usize {
        1 + elem.children.iter().map(count_elements).sum::<usize>()
    }

    let count = count_elements(&root);
    assert!(
        count >= min_count,
        "Title {} should have at least {} elements but got {}",
        title,
        min_count,
        count
    );
}

// Test 5: Validate date field across titles
#[rstest]
#[case("01", "2025-07-18")]
#[case("04", "2025-07-18")]
#[case("09", "2025-07-18")]
fn test_parse_date_in_metadata(#[case] title: &str, #[case] expected_date: &str) {
    let path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", title);
    let root =
        parse(&path, expected_date).unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", title));

    // Date should be set correctly
    let date_str = format!(
        "{}-{:02}-{:02}",
        root.data.date.year(),
        root.data.date.month() as u8,
        root.data.date.day()
    );
    assert_eq!(date_str, expected_date);
}

// Test 6: JSON serialization roundtrip across titles
#[rstest]
#[case("01")]
#[case("04")]
#[case("09")]
fn test_cross_title_serialization(#[case] title: &str) {
    let path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", title);
    let root =
        parse(&path, "2025-07-18").unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", title));

    // Serialize to JSON
    let json = serde_json::to_string(&root).expect("Failed to serialize to JSON");

    // Deserialize back
    let deserialized: words_to_data::uslm::USLMElement =
        serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify paths match
    assert_eq!(root.data.path, deserialized.data.path);
    assert_eq!(root.data.uslm_id, deserialized.data.uslm_id);
}

// Test 7: Compare appendix vs regular titles
#[rstest]
#[case("05", "05A")] // Title 5 vs Title 5 Appendix
fn test_appendix_vs_regular_titles(#[case] regular: &str, #[case] appendix: &str) {
    let regular_path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", regular);
    let appendix_path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", appendix);

    let regular_root = parse(&regular_path, "2025-07-18")
        .unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", regular));

    let appendix_root = parse(&appendix_path, "2025-07-18")
        .unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", appendix));

    // Both should be USC documents
    assert!(matches!(
        regular_root.data.document_type,
        DocumentType::USCode { .. }
    ));
    assert!(matches!(
        appendix_root.data.document_type,
        DocumentType::USCode { .. }
    ));

    // Paths should be different
    assert_ne!(regular_root.data.path, appendix_root.data.path);
}

// Test 8: Generate diffs across title pairs
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
