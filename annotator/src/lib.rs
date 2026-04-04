use std::str::FromStr;

use words_to_data::{
    diff::TreeDiff,
    legal_diff::{AnnotationMetadata, AnnotationStatus, BillReference, ChangeAnnotation, LegalDiff},
    uslm::{AmendingAction, bill_parser::parse_bill_amendments},
    utils::parse_uslm_xml,
};

/// Load two USC XML files, compute and return the TreeDiff as a JSON string.
///
/// Parsing large XML files (e.g. Title 26) is CPU-intensive, so this runs on
/// a blocking thread to avoid stalling the async runtime.
#[tauri::command]
async fn load_usc_pair(
    old_path: String,
    old_date: String,
    new_path: String,
    new_date: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let old = parse_uslm_xml(&old_path, &old_date).map_err(|e| e.to_string())?;
        let new_doc = parse_uslm_xml(&new_path, &new_date).map_err(|e| e.to_string())?;
        let diff = TreeDiff::from_elements(&old, &new_doc);
        serde_json::to_string(&diff).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Parse a bill XML file and return the AmendmentData as a JSON string.
#[tauri::command]
async fn load_bill(path: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let data = parse_bill_amendments(&path).map_err(|e| e.to_string())?;
        serde_json::to_string(&data).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Build a ChangeAnnotation from form fields and return it as a JSON string.
///
/// The timestamp is set server-side to UTC now. The status is always Pending
/// for human annotations created through this tool.
#[tauri::command]
fn create_annotation(
    operation: String,
    bill_id: String,
    amendment_id: String,
    causative_text: String,
    paths: Vec<String>,
    annotator: String,
    notes: Option<String>,
) -> Result<String, String> {
    let op = AmendingAction::from_str(&operation).map_err(|e| e.to_string())?;
    let annotation = ChangeAnnotation {
        operation: op,
        source_bill: BillReference {
            bill_id,
            amendment_id,
            causative_text,
        },
        paths,
        metadata: AnnotationMetadata {
            status: AnnotationStatus::Pending,
            confidence: None,
            annotator,
            timestamp: time::OffsetDateTime::now_utc(),
            notes,
            reasoning: None,
        },
    };
    serde_json::to_string(&annotation).map_err(|e| e.to_string())
}

/// Combine a TreeDiff with a list of annotations into a LegalDiff and write
/// it as pretty-printed JSON to the given output path.
#[tauri::command]
fn export_legal_diff(
    tree_diff_json: String,
    annotations_json: String,
    output_path: String,
) -> Result<(), String> {
    let tree_diff: TreeDiff =
        serde_json::from_str(&tree_diff_json).map_err(|e| e.to_string())?;
    let annotations: Vec<ChangeAnnotation> =
        serde_json::from_str(&annotations_json).map_err(|e| e.to_string())?;
    let mut legal_diff = LegalDiff::new(&tree_diff);
    for annotation in annotations {
        legal_diff.add_annotation(annotation);
    }
    let json = serde_json::to_string_pretty(&legal_diff).map_err(|e| e.to_string())?;
    std::fs::write(&output_path, json).map_err(|e| e.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_usc_pair,
            load_bill,
            create_annotation,
            export_legal_diff,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
