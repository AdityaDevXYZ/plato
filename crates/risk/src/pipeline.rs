use crate::model::*;
use async_trait::async_trait;

pub struct AssessmentContext {
    pub plan_id: uuid::Uuid,
    // Typically contains references to Twin, Orbit, Resource summaries, etc.
}

#[async_trait]
pub trait RiskAssessment: Send + Sync {
    fn name(&self) -> &'static str;
    async fn evaluate(&self, ctx: &AssessmentContext) -> Result<Vec<RiskFactor>, String>;
}

pub struct RiskAssessmentPipeline {
    assessments: Vec<Box<dyn RiskAssessment>>,
}

impl RiskAssessmentPipeline {
    pub fn new(assessments: Vec<Box<dyn RiskAssessment>>) -> Self {
        Self { assessments }
    }

    pub async fn run(&self, ctx: &AssessmentContext) -> Result<Vec<RiskFactor>, String> {
        let mut all_factors = Vec::new();
        for assessment in &self.assessments {
            let mut factors = assessment.evaluate(ctx).await?;
            all_factors.append(&mut factors);
        }
        Ok(all_factors)
    }

    pub fn aggregate(&self, factors: &[RiskFactor]) -> (u8, RiskSeverity) {
        if factors.is_empty() { return (0, RiskSeverity::Low); }
        let total_score: u32 = factors.iter().map(|f| f.score as u32).sum();
        let avg_score = (total_score / factors.len() as u32) as u8;
        
        let severity = if factors.iter().any(|f| f.severity == RiskSeverity::Critical) || avg_score >= 80 {
            RiskSeverity::Critical
        } else if factors.iter().any(|f| f.severity == RiskSeverity::High) || avg_score >= 60 {
            RiskSeverity::High
        } else if factors.iter().any(|f| f.severity == RiskSeverity::Medium) || avg_score >= 30 {
            RiskSeverity::Medium
        } else {
            RiskSeverity::Low
        };

        (avg_score, severity)
    }
}
