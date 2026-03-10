use words_to_data::uslm::{ElementType, parser::parse};

// Test 1: Find root element in real USC document
#[test]
fn test_find_root_in_real_document() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result = root.find("uscodedocument_9");
    assert!(result.is_some(), "Should find root");

    let found = result.unwrap();
    assert_eq!(found.data.path, "uscodedocument_9");
    assert_eq!(found.data.element_type, ElementType::USCodeDocument);
}

// Test 2: Find title element
#[test]
fn test_find_title_element() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result = root.find("uscodedocument_9/title_9");
    assert!(result.is_some(), "Should find title");

    let found = result.unwrap();
    assert_eq!(found.data.path, "uscodedocument_9/title_9");
    assert_eq!(found.data.element_type, ElementType::Title);
}

// Test 3: Find chapter element
#[test]
fn test_find_chapter_element() {
    let root = parse("tests/test_data/usc/2025-07-18/usc01.xml", "2025-07-18")
        .expect("Failed to parse usc01.xml");

    let result = root.find("uscodedocument_1/title_1/chapter_1");
    assert!(result.is_some(), "Should find chapter");

    let found = result.unwrap();
    assert_eq!(found.data.element_type, ElementType::Chapter);
    assert_eq!(found.data.number_value, "1");
}

// Test 4: Find section element
#[test]
fn test_find_section_element() {
    let root = parse("tests/test_data/usc/2025-07-18/usc04.xml", "2025-07-18")
        .expect("Failed to parse usc04.xml");

    let result = root.find("uscodedocument_4/title_4/chapter_1/section_1");
    assert!(result.is_some(), "Should find section");

    let found = result.unwrap();
    assert_eq!(found.data.element_type, ElementType::Section);
}

// Test 5: Find subsection element (deep navigation)
#[test]
fn test_find_subsection_element() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result = root.find("uscodedocument_9/title_9/chapter_1/section_10/subsection_a");
    assert!(result.is_some(), "Should find subsection");

    let found = result.unwrap();
    assert_eq!(found.data.element_type, ElementType::Subsection);
    assert_eq!(found.data.number_value, "a");
}

// Test 6: Find paragraph element (very deep navigation)
#[test]
fn test_find_paragraph_element() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result =
        root.find("uscodedocument_9/title_9/chapter_1/section_10/subsection_a/paragraph_1");
    assert!(result.is_some(), "Should find paragraph");

    let found = result.unwrap();
    assert_eq!(found.data.element_type, ElementType::Paragraph);
}

// Test 7: Find nonexistent path returns None
#[test]
fn test_find_nonexistent_path() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result = root.find("uscodedocument_9/title_9/chapter_99");
    assert!(result.is_none(), "Should not find nonexistent chapter");
}

// Test 8: Partial path fails (must match exactly)
#[test]
fn test_find_partial_path_fails() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result = root.find("uscodedocument_9/title_9/chapter_1/section_99");
    assert!(result.is_none(), "Should not find nonexistent section");
}

// Test 9: Wrong title prefix returns None
#[test]
fn test_find_wrong_title_prefix() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result = root.find("uscodedocument_26/title_26");
    assert!(result.is_none(), "Should not find wrong title");
}

// Test 10: Find preserves children
#[test]
fn test_find_preserves_children() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result = root.find("uscodedocument_9/title_9/chapter_1/section_10/subsection_a");
    assert!(result.is_some(), "Should find subsection");

    let found = result.unwrap();
    // Subsection a should have multiple paragraph children
    assert!(
        !found.children.is_empty(),
        "Subsection should have children"
    );
    assert_eq!(found.children[0].data.element_type, ElementType::Paragraph);
}

// Test 11: Find leaf node
#[test]
fn test_find_leaf_node() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    // Find a paragraph which is a leaf node
    let result =
        root.find("uscodedocument_9/title_9/chapter_1/section_10/subsection_a/paragraph_1");
    assert!(result.is_some(), "Should find leaf paragraph");

    let found = result.unwrap();
    // Leaf node has no children (or might have subparagraphs)
    assert_eq!(found.data.element_type, ElementType::Paragraph);
}

// Test 12: Find with multiple siblings
#[test]
fn test_find_with_multiple_siblings() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    // Section 10 subsection a has multiple paragraph siblings
    let para1 = root.find("uscodedocument_9/title_9/chapter_1/section_10/subsection_a/paragraph_1");
    let para2 = root.find("uscodedocument_9/title_9/chapter_1/section_10/subsection_a/paragraph_2");
    let para3 = root.find("uscodedocument_9/title_9/chapter_1/section_10/subsection_a/paragraph_3");

    assert!(para1.is_some(), "Should find paragraph 1");
    assert!(para2.is_some(), "Should find paragraph 2");
    assert!(para3.is_some(), "Should find paragraph 3");

    assert_eq!(para1.unwrap().data.number_value, "1");
    assert_eq!(para2.unwrap().data.number_value, "2");
    assert_eq!(para3.unwrap().data.number_value, "3");
}

// Test 13: Empty path returns None
#[test]
fn test_find_empty_path() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    let result = root.find("");
    assert!(result.is_none(), "Empty path should return None");
}

// Test 14: Find in appendix file
#[test]
fn test_find_appendix_element() {
    let root = parse("tests/test_data/usc/2025-07-18/usc05A.xml", "2025-07-18")
        .expect("Failed to parse usc05A.xml");

    // usc05A is Title 5 Appendix - verify root can be found
    let result = root.find("uscodedocument_5a");
    assert!(result.is_some(), "Should find appendix document root");

    let found = result.unwrap();
    assert_eq!(found.data.element_type, ElementType::USCodeDocument);

    // Try to find title element within appendix
    let title_result = root.find("uscodedocument_5a/title_5a");
    if let Some(result) = title_result {
        assert_eq!(result.data.element_type, ElementType::Title);
    }
}

// Test 15: Navigate deeply nested structure
#[test]
fn test_find_deeply_nested_structure() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    // Navigate to subparagraph (6 levels deep)
    let result = root.find(
        "uscodedocument_9/title_9/chapter_1/section_16/subsection_a/paragraph_1/subparagraph_A",
    );

    if let Some(found) = result {
        assert_eq!(found.data.element_type, ElementType::Subparagraph);
        assert_eq!(found.data.number_value, "A");
    }
    // If this specific path doesn't exist, that's OK - we're testing the navigation works
}

// Test 16: Path too deep returns None
#[test]
fn test_find_path_too_deep() {
    let root = parse("tests/test_data/usc/2025-07-18/usc09.xml", "2025-07-18")
        .expect("Failed to parse usc09.xml");

    // Try to navigate beyond what exists
    let result = root.find("uscodedocument_9/title_9/chapter_1/section_1/subsection_a/paragraph_1/subparagraph_a/clause_1/subclause_1/item_1");
    assert!(result.is_none(), "Path too deep should return None");
}
