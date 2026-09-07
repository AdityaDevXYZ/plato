use crate::error::ApplicationError;
use readiness::{
    model::{DeploymentReadinessReport, ReadinessDecision, EvidenceStatus},
    pipeline::{DeploymentReadinessPipeline, ReadinessContext},
    repository::ReadinessReportRepository,
    events::*,
};
use types::ReleaseId;
use std::sync::Arc;
use eventbus::{EventPublisher, EventMessage};
use uuid::Uuid;
use time::OffsetDateTime;

pub struct DeploymentReadinessService {
    pipeline: Arc<DeploymentReadinessPipeline>,
    report_repo: Arc<dyn ReadinessReportRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl DeploymentReadinessService {
    pub fn new(pipeline: Arc<DeploymentReadinessPipeline>, report_repo: Arc<dyn ReadinessReportRepository>, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self { pipeline, report_repo, event_publisher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn run_readiness(&self, release_id: ReleaseId, correlation_id: Uuid) -> Result<DeploymentReadinessReport, ApplicationError> {
        let report_id = Uuid::new_v4();
        
        let start_ev = EventMessage::new(Box::new(DeploymentReadinessStarted { release_id, report_id }), correlation_id, "plato.dra");
        let _ = self.event_publisher.publish(vec![start_ev]);

        let ctx = ReadinessContext { release_id };
        let evidence = self.pipeline.run(&ctx).await;

        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut failed = false;
        let mut warned = false;

        for e in &evidence {
            match e.status {
                EvidenceStatus::Fail => {
                    failed = true;
                    errors.push(format!("{}: {}", e.name, e.details));
                }
                EvidenceStatus::Warning => {
                    warned = true;
                    warnings.push(format!("{}: {}", e.name, e.details));
                }
                _ => {}
            }
        }

        let decision = if failed {
            ReadinessDecision::NotReady
        } else if warned {
            ReadinessDecision::ReadyWithWarnings
        } else {
            ReadinessDecision::Ready
        };

        let summary = format!("Pipeline completed with {} errors and {} warnings.", errors.len(), warnings.len());

        let report = DeploymentReadinessReport {
            report_id,
            release_id,
            correlation_id,
            timestamp: OffsetDateTime::now_utc(),
            decision: decision.clone(),
            summary,
            evidence,
            warnings,
            errors,
        };

        self.report_repo.save(&report).await.map_err(|e| ApplicationError::Repository(e.to_string()))?;

        let mut events = vec![
            EventMessage::new(Box::new(DeploymentReadinessCompleted { 
                release_id, 
                report_id, 
                decision: format!("{:?}", decision) 
            }), correlation_id, "plato.dra")
        ];

        if decision == ReadinessDecision::NotReady {
            events.push(EventMessage::new(Box::new(DeploymentNotReady { 
                release_id, report_id, reason: "Checks failed".into() 
            }), correlation_id, "plato.dra"));
        } else {
            events.push(EventMessage::new(Box::new(ReleaseReady { 
                release_id, report_id 
            }), correlation_id, "plato.dra"));
        }

        let _ = self.event_publisher.publish(events);

        Ok(report)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_report(&self, release_id: ReleaseId) -> Result<DeploymentReadinessReport, ApplicationError> {
        self.report_repo.find_by_release_id(release_id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Report not found".into()))
    }
}
