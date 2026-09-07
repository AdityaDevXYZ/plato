use crate::error::ApplicationError;
use risk::{
    model::*,
    pipeline::{RiskAssessmentPipeline, AssessmentContext},
    repository::RiskRepository,
    events::*,
};
use std::sync::Arc;
use uuid::Uuid;
use eventbus::{EventPublisher, EventMessage};
use time::OffsetDateTime;

pub struct RiskAssessmentService {
    repo: Arc<dyn RiskRepository>,
    pipeline: Arc<RiskAssessmentPipeline>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl RiskAssessmentService {
    pub fn new(repo: Arc<dyn RiskRepository>, pipeline: Arc<RiskAssessmentPipeline>, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self { repo, pipeline, event_publisher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn evaluate_plan(&self, plan_id: Uuid, correlation_id: Uuid) -> Result<RiskAssessmentReport, ApplicationError> {
        let ctx = AssessmentContext { plan_id };
        
        let factors = self.pipeline.run(&ctx).await.map_err(|e| ApplicationError::Domain(format!("Risk assessment failed: {}", e)))?;
        let (overall_score, overall_severity) = self.pipeline.aggregate(&factors);

        let report = RiskAssessmentReport {
            report_id: Uuid::new_v4(),
            plan_id,
            correlation_id,
            timestamp: OffsetDateTime::now_utc(),
            factors,
            overall_score,
            overall_severity: overall_severity.clone(),
            confidence: 0.9, // Mock
        };

        self.repo.save_report(&report).await.map_err(|e| ApplicationError::Repository(e.to_string()))?;

        let mut events = vec![EventMessage::new(Box::new(RiskAssessmentCompleted { report_id: report.report_id, plan_id, overall_score }), correlation_id, "plato.risk")];
        
        if overall_severity == RiskSeverity::Critical {
            events.push(EventMessage::new(Box::new(RiskThresholdExceeded { report_id: report.report_id, plan_id, risk_level: "Critical".into() }), correlation_id, "plato.risk"));
        }

        let _ = self.event_publisher.publish(events);

        Ok(report)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_report(&self, id: Uuid) -> Result<RiskAssessmentReport, ApplicationError> {
        self.repo.get_report(id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Report not found".into()))
    }
}
