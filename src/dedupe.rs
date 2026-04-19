use crate::models::SignedAnnouncement;

/// Stub for semantic deduplication
/// Future: Use embeddings or ML for similarity
pub fn similarity_score(ann1: &SignedAnnouncement, ann2: &SignedAnnouncement) -> f64 {
    let name1 = ann1.metadata.get("name").unwrap_or(&ann1.skill_id);
    let name2 = ann2.metadata.get("name").unwrap_or(&ann2.skill_id);
    let desc1 = ann1.metadata.get("description").map(|s| s.as_str()).unwrap_or("");
    let desc2 = ann2.metadata.get("description").map(|s| s.as_str()).unwrap_or("");

    // Simple Jaccard similarity on words
    let words1: std::collections::HashSet<&str> = name1.split_whitespace().chain(desc1.split_whitespace()).collect();
    let words2: std::collections::HashSet<&str> = name2.split_whitespace().chain(desc2.split_whitespace()).collect();

    let intersection = words1.intersection(&words2).count();
    let union = words1.len() + words2.len() - intersection;

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Check if two announcements are duplicates (similarity > threshold)
pub fn is_duplicate(ann1: &SignedAnnouncement, ann2: &SignedAnnouncement, threshold: f64) -> bool {
    similarity_score(ann1, ann2) > threshold
}