use types::{UserId, OrganizationId, ProjectId};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Owner,
    Administrator,
    Operator,
    Viewer,
}

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Insufficient permissions")]
    Forbidden,
}

pub struct AuthorizationContext {
    pub user_id: UserId,
    pub role: Role,
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn can_create_project(ctx: &AuthorizationContext, _target_org: OrganizationId) -> Result<(), AuthorizationError> {
        match ctx.role {
            Role::Owner | Role::Administrator => Ok(()),
            _ => Err(AuthorizationError::Forbidden),
        }
    }

    pub fn can_deploy_release(ctx: &AuthorizationContext, _target_project: ProjectId) -> Result<(), AuthorizationError> {
        match ctx.role {
            Role::Owner | Role::Administrator | Role::Operator => Ok(()),
            _ => Err(AuthorizationError::Forbidden),
        }
    }
    
    pub fn can_view_mission(ctx: &AuthorizationContext) -> Result<(), AuthorizationError> {
        Ok(()) // Viewers and above
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_checks() {
        let owner_ctx = AuthorizationContext { user_id: UserId::new(), role: Role::Owner };
        let viewer_ctx = AuthorizationContext { user_id: UserId::new(), role: Role::Viewer };

        assert!(PolicyEngine::can_create_project(&owner_ctx, OrganizationId::new()).is_ok());
        assert!(PolicyEngine::can_create_project(&viewer_ctx, OrganizationId::new()).is_err());
    }
}
