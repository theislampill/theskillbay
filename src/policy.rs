use crate::models::{LocalPolicy, CentralPolicy, InstallDecision, ExecutionDecision, SignedAnnouncement};
use crate::crypto;

/// Evaluate local policy for install
pub fn evaluate_local_install(
    announcement: &SignedAnnouncement,
    local_policy: &LocalPolicy,
) -> InstallDecision {
    // Check if publisher is blocked
    if local_policy.blocked_skills.iter().any(|blocked| blocked == &announcement.skill_id) {
        return InstallDecision {
            skill_id: announcement.skill_id.clone(),
            allowed: false,
            reason: "Skill blocked by local policy".to_string(),
        };
    }

    // Check publisher whitelist if any
    if !local_policy.allowed_publishers.is_empty() &&
       !local_policy.allowed_publishers.contains(&announcement.public_key) {
        return InstallDecision {
            skill_id: announcement.skill_id.clone(),
            allowed: false,
            reason: "Publisher not in allowed list".to_string(),
        };
    }

    InstallDecision {
        skill_id: announcement.skill_id.clone(),
        allowed: true,
        reason: "Approved by local policy".to_string(),
    }
}

/// Evaluate central policy for install
pub fn evaluate_central_install(
    announcement: &SignedAnnouncement,
    central_policy: Option<&CentralPolicy>,
) -> Option<InstallDecision> {
    let central = central_policy?;
    if central.banned_publishers.contains(&announcement.public_key) {
        return Some(InstallDecision {
            skill_id: announcement.skill_id.clone(),
            allowed: false,
            reason: "Publisher banned by central policy".to_string(),
        });
    }
    if !central.approved_skills.contains(&announcement.skill_id) {
        return Some(InstallDecision {
            skill_id: announcement.skill_id.clone(),
            allowed: false,
            reason: "Skill not approved by central policy".to_string(),
        });
    }
    Some(InstallDecision {
        skill_id: announcement.skill_id.clone(),
        allowed: true,
        reason: "Approved by central policy".to_string(),
    })
}

/// Combine local and central for install decision
pub fn combined_install_decision(
    announcement: &SignedAnnouncement,
    local_policy: &LocalPolicy,
    central_policy: Option<&CentralPolicy>,
) -> InstallDecision {
    let local_decision = evaluate_local_install(announcement, local_policy);
    if !local_decision.allowed {
        return local_decision;
    }
    if local_policy.require_central_approval {
        if let Some(central) = central_policy {
            let central_decision = evaluate_central_install(announcement, Some(central));
            if let Some(dec) = central_decision {
                return dec;
            } else {
                return InstallDecision {
                    skill_id: announcement.skill_id.clone(),
                    allowed: false,
                    reason: "Central approval required but no central policy available".to_string(),
                };
            }
        } else {
            return InstallDecision {
                skill_id: announcement.skill_id.clone(),
                allowed: false,
                reason: "Central approval required but no central policy available".to_string(),
            };
        }
    }
    if let Some(central) = central_policy {
        let central_decision = evaluate_central_install(announcement, Some(central));
        if let Some(dec) = central_decision {
            if !dec.allowed {
                return dec;
            }
        }
    }
    local_decision
}

/// Evaluate for execution (similar logic)
pub fn evaluate_execution(
    skill_id: &str,
    local_policy: &LocalPolicy,
    central_policy: Option<&CentralPolicy>,
) -> ExecutionDecision {
    if local_policy.blocked_skills.iter().any(|blocked| blocked == skill_id) {
        return ExecutionDecision {
            skill_id: skill_id.to_string(),
            allowed: false,
            reason: "Skill blocked by local policy".to_string(),
        };
    }
    // Note: Central policy check for execution is deferred in V0 due to identity resolution complexity
    ExecutionDecision {
        skill_id: skill_id.to_string(),
        allowed: true,
        reason: "Approved for execution".to_string(),
    }
}