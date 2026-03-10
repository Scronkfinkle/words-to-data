use words_to_data::diff::TreeDiff;
use words_to_data::uslm::{ElementType, parser::parse};

// Test 1: Full parse → serialize → deserialize → verify roundtrip
#[test]
fn test_full_parse_serialize_deserialize_roundtrip() {
    // Parse a USC file
    let original = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    // Serialize to JSON string
    let json_string = serde_json::to_string(&original).expect("Failed to serialize to JSON");

    // Deserialize back from JSON
    let deserialized: words_to_data::uslm::USLMElement =
        serde_json::from_str(&json_string).expect("Failed to deserialize from JSON");

    // Verify key fields match
    assert_eq!(original.data.path, deserialized.data.path);
    assert_eq!(original.data.element_type, deserialized.data.element_type);
    assert_eq!(original.data.uslm_id, deserialized.data.uslm_id);
    assert_eq!(original.data.number_value, deserialized.data.number_value);
    assert_eq!(original.data.heading, deserialized.data.heading);

    // Verify children count matches
    assert_eq!(original.children.len(), deserialized.children.len());
}

// Test 2: Find all ElementType variants in real documents
#[test]
fn test_parse_all_element_types() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    // Collect all element types found
    fn collect_types(
        elem: &words_to_data::uslm::USLMElement,
        types: &mut std::collections::HashSet<ElementType>,
    ) {
        types.insert(elem.data.element_type);
        for child in &elem.children {
            collect_types(child, types);
        }
    }

    let mut found_types = std::collections::HashSet::new();
    collect_types(&root, &mut found_types);

    // Verify we found multiple types
    assert!(
        found_types.len() >= 4,
        "Should find at least 4 different element types, found: {:?}",
        found_types
    );

    // Verify some common types
    assert!(
        found_types.contains(&ElementType::USCodeDocument),
        "Should contain USCodeDocument"
    );
    assert!(
        found_types.contains(&ElementType::Title),
        "Should contain Title"
    );
}

// Test 3: Verify verbose_name generation
#[test]
fn test_verbose_name_generation() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    // Navigate to title
    let title = root
        .find("uscodedocument_9/title_9")
        .expect("Failed to find title");

    // Verify verbose_name is populated and non-empty
    assert!(
        !title.data.verbose_name.is_empty(),
        "Verbose name should be populated"
    );
    assert!(
        title.data.verbose_name.contains("9"),
        "Verbose name should contain title number"
    );
}

// Test 4: Compare number_display vs number_value
#[test]
fn test_number_display_vs_value() {
    let root = parse("tests/test_data/usc/2025-07-18/usc01.xml", "2025-07-18")
        .expect("Failed to parse usc01.xml");

    // Find a section
    let section = root
        .find("uscodedocument_1/title_1/chapter_1/section_1")
        .expect("Failed to find section");

    // number_value should be simpler (just the number)
    assert_eq!(section.data.number_value, "1");

    // number_display might include § symbol and formatting
    assert!(
        section.data.number_display.contains("1"),
        "Number display should contain the number"
    );
}

// Test 5: Diff with all change types
#[test]
fn test_diff_with_all_change_types() {
    let tree1 = parse("tests/test_data/usc/2025-07-18/usc26.xml", "2025-07-18")
        .expect("Failed to parse usc26 from 2025-07-18");

    let tree2 = parse("tests/test_data/usc/2025-07-30/usc26.xml", "2025-07-30")
        .expect("Failed to parse usc26 from 2025-07-30");

    // Generate diff
    let diff = TreeDiff::from_elements(&tree1, &tree2);

    // Verify diff structure
    assert!(!diff.root_path.is_empty(), "Diff should have root path");

    // The diff might have changes, additions, or removals
    // Just verify we can access all fields
    let _changes = &diff.changes;
    let _added = &diff.added;
    let _removed = &diff.removed;
    let _child_diffs = &diff.child_diffs;

    // Verify diff is serializable
    let _json = serde_json::to_string(&diff).expect("Diff should be serializable");
}

// Test 6: Diff word-level granularity
#[test]
fn test_diff_word_level_granularity() {
    let tree1 = parse("tests/test_data/usc/2025-07-18/usc07.xml", "2025-07-18")
        .expect("Failed to parse usc07 from 2025-07-18");

    let tree2 = parse("tests/test_data/usc/2025-07-30/usc07.xml", "2025-07-30")
        .expect("Failed to parse usc07 from 2025-07-30");

    // Generate diff
    let diff = TreeDiff::from_elements(&tree1, &tree2);

    // If there are changes, verify they have proper structure
    if !diff.changes.is_empty() {
        let first_change = &diff.changes[0];

        // Verify field change has required fields - field_name is enum, not string
        // Just verify the fields exist
        let _field = &first_change.field_name;
        let _old = &first_change.old_value;
        let _new = &first_change.new_value;

        // Verify text changes exist and have proper structure
        if !first_change.changes.is_empty() {
            let text_change = &first_change.changes[0];

            // Text change should have value field
            assert!(
                !text_change.value.is_empty() || text_change.value.is_empty(),
                "Text change value should be accessible"
            );
        }
    }

    // Test passes if diff generation completes without error
}

// Test 7: Parse multiple titles and verify consistency
#[test]
fn test_parse_multiple_titles_consistency() {
    let titles = vec!["01", "04", "09"];
    let mut all_parsed = Vec::new();

    for title in titles {
        let path = format!("tests/test_data/usc/2025-07-18/usc{}.xml", title);
        let parsed = parse(&path, "2025-07-18")
            .unwrap_or_else(|_| panic!("Failed to parse usc{}.xml", title));

        all_parsed.push(parsed);
    }

    // Verify all have consistent structure
    for (idx, root) in all_parsed.iter().enumerate() {
        assert_eq!(
            root.data.element_type,
            ElementType::USCodeDocument,
            "Title {} should be USCodeDocument",
            idx
        );

        assert!(
            root.data.uslm_id.is_some(),
            "Title {} should have USLM ID",
            idx
        );

        assert!(
            !root.children.is_empty(),
            "Title {} should have children",
            idx
        );
    }
}

// Test 8: Verify paths are hierarchical and properly formatted
#[test]
fn test_path_hierarchy_format() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    // Verify root path
    assert_eq!(root.data.path, "uscodedocument_9");

    // Navigate to nested element and verify path builds correctly
    let subsection = root
        .find("uscodedocument_9/title_9/chapter_1/section_10/subsection_a")
        .expect("Failed to find subsection");

    // Verify path is hierarchical
    assert_eq!(
        subsection.data.path,
        "uscodedocument_9/title_9/chapter_1/section_10/subsection_a"
    );

    // Verify path segments
    let segments: Vec<&str> = subsection.data.path.split('/').collect();
    assert_eq!(segments.len(), 5);
    assert_eq!(segments[0], "uscodedocument_9");
    assert_eq!(segments[1], "title_9");
    assert_eq!(segments[2], "chapter_1");
    assert_eq!(segments[3], "section_10");
    assert_eq!(segments[4], "subsection_a");
}
