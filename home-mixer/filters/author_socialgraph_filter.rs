use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use std::collections::HashSet;
use tonic::async_trait;
use xai_candidate_pipeline::filter::{Filter, FilterResult};

// Remove candidates that are blocked or muted by the viewer
// This filter converts the blocked/muted user ID lists to HashSets for O(1)
pub struct AuthorSocialgraphFilter;

#[async_trait]
impl Filter<ScoredPostsQuery, PostCandidate> for AuthorSocialgraphFilter {
    async fn filter(
        &self,
        query: &ScoredPostsQuery,
        candidates: Vec<PostCandidate>,
    ) -> Result<FilterResult<PostCandidate>, String> {
        // Early return if no blocked/muted users - avoid HashSet allocation
        if query.user_features.blocked_user_ids.is_empty()
            && query.user_features.muted_user_ids.is_empty()
        {
            return Ok(FilterResult {
                kept: candidates,
                removed: Vec::new(),
            });
        }

        let blocked_set: HashSet<i64> = query
            .user_features
            .blocked_user_ids
            .iter()
            .copied()
            .collect();
        let muted_set: HashSet<i64> = query
            .user_features
            .muted_user_ids
            .iter()
            .copied()
            .collect();

        let mut kept: Vec<PostCandidate> = Vec::with_capacity(candidates.len());
        let mut removed: Vec<PostCandidate> = Vec::new();

        for candidate in candidates {
            let author_id = candidate.author_id as i64;
            if blocked_set.contains(&author_id) || muted_set.contains(&author_id) {
                removed.push(candidate);
            } else {
                kept.push(candidate);
            }
        }

        Ok(FilterResult { kept, removed })
    }
}
