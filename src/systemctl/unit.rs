use std::fmt::Display;

use zbus::zvariant::OwnedObjectPath;

// NOTE: These docs are all from the man page of org.freedesktop.systemd1

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnitEnablementChange {
    pub unit_change_kind: UnitChangeKind,
    pub filename: String,
    pub destination: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitEnablementResponse {
    NoContext,
    AdditionalContext(Vec<UnitEnablementChange>),
}

impl From<Vec<(String, String, String)>> for UnitEnablementResponse {
    fn from(value: Vec<(String, String, String)>) -> Self {
        UnitEnablementResponse::AdditionalContext(
            value
                .into_iter()
                .map(|x| UnitEnablementChange {
                    unit_change_kind: x.0.into(),
                    filename: x.1,
                    destination: x.2,
                })
                .collect(),
        )
    }
}

impl From<(bool, Vec<(String, String, String)>)> for UnitEnablementResponse {
    fn from(value: (bool, Vec<(String, String, String)>)) -> Self {
        if value.0 {
            UnitEnablementResponse::NoContext
        } else {
            UnitEnablementResponse::AdditionalContext(
                value
                    .1
                    .into_iter()
                    .map(|x| UnitEnablementChange {
                        unit_change_kind: x.0.into(),
                        filename: x.1,
                        destination: x.2,
                    })
                    .collect(),
            )
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitChangeKind {
    Symlink,
    Unlink,
    Other(String),
}

impl Display for UnitChangeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            UnitChangeKind::Symlink => "symlink",
            UnitChangeKind::Unlink => "unlink",
            UnitChangeKind::Other(val) => val,
        };
        f.write_str(value)
    }
}

impl From<String> for UnitChangeKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "symlink" => UnitChangeKind::Symlink,
            "unlink" => UnitChangeKind::Unlink,
            _ => UnitChangeKind::Other(value),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitMode {
    Replace,
    Fail,
    Isolate,
    IgnoreDependencies,
    IgnoreRequirements,
    Other(String),
}

impl Display for UnitMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            UnitMode::Replace => "replace",
            UnitMode::Fail => "fail",
            UnitMode::Isolate => "isolate",
            UnitMode::IgnoreDependencies => "ignore-dependencies",
            UnitMode::IgnoreRequirements => "ignore-requirements",
            UnitMode::Other(other) => other,
        };
        f.write_str(mode)
    }
}

impl From<String> for UnitMode {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "replace" => UnitMode::Replace,
            "fail" => UnitMode::Fail,
            "isolate" => UnitMode::Isolate,
            "ignore-dependencies" => UnitMode::IgnoreDependencies,
            "ignore-requirements" => UnitMode::IgnoreRequirements,
            _ => UnitMode::Other(value),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitLoadState {
    Stub,
    Loaded,
    NotFound,
    BadSetting,
    Error,
    Merged,
    Masked,
    Other(String),
}

impl From<String> for UnitLoadState {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "stub" => UnitLoadState::Stub,
            "loaded" => UnitLoadState::Loaded,
            "not-found" => UnitLoadState::NotFound,
            "bad-setting" => UnitLoadState::BadSetting,
            "error" => UnitLoadState::Error,
            "merged" => UnitLoadState::Merged,
            "masked" => UnitLoadState::Masked,
            _ => UnitLoadState::Other(value),
        }
    }
}

impl Display for UnitLoadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            UnitLoadState::Stub => "stub",
            UnitLoadState::Loaded => "loaded",
            UnitLoadState::NotFound => "not-found",
            UnitLoadState::BadSetting => "bad-setting",
            UnitLoadState::Error => "error",
            UnitLoadState::Merged => "merged",
            UnitLoadState::Masked => "masked",
            UnitLoadState::Other(val) => val,
        };

        f.write_str(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitActiveState {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Maintenance,
    Other(String),
}

impl From<String> for UnitActiveState {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "active" => UnitActiveState::Active,
            "reloading" => UnitActiveState::Reloading,
            "inactive" => UnitActiveState::Inactive,
            "failed" => UnitActiveState::Failed,
            "activating" => UnitActiveState::Activating,
            "deactivating" => UnitActiveState::Deactivating,
            "maintenance" => UnitActiveState::Maintenance,
            _ => UnitActiveState::Other(value),
        }
    }
}

impl Display for UnitActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            UnitActiveState::Active => "active",
            UnitActiveState::Reloading => "reloading",
            UnitActiveState::Inactive => "inactive",
            UnitActiveState::Failed => "failed",
            UnitActiveState::Activating => "activating",
            UnitActiveState::Deactivating => "deactivating",
            UnitActiveState::Maintenance => "maintenance",
            UnitActiveState::Other(val) => val,
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Unit {
    /// The primary unit name as string
    pub name: String,
    /// The human readable description
    pub description: String,
    /// The load state (i.e. whether the unit file has been loaded successfully)
    pub load_state: UnitLoadState,
    /// The active state (i.e. whether the unit is currently started or not)
    pub active_state: UnitActiveState,
    /// The sub state (a more fine-grained version of the active state that is specific to the unit type, which the active state is not)
    pub sub_state: String,
    /// A unit that is being followed in its state by this unit, if there is any.
    pub followed_unit: Option<String>,
    /// The unit object path
    pub object_path: OwnedObjectPath,
    /// If there is a job queued for the job unit, the numeric job id
    pub queued_job_id: Option<u32>,
    /// The job type as string
    pub job_type: String,
    /// The job object path
    pub job_object_path: OwnedObjectPath,
}

impl
    From<(
        String,
        String,
        String,
        String,
        String,
        String,
        OwnedObjectPath,
        u32,
        String,
        OwnedObjectPath,
    )> for Unit
{
    fn from(
        value: (
            String,
            String,
            String,
            String,
            String,
            String,
            OwnedObjectPath,
            u32,
            String,
            OwnedObjectPath,
        ),
    ) -> Self {
        Self {
            name: value.0,
            description: value.1,
            load_state: value.2.into(),
            active_state: value.3.into(),
            sub_state: value.4,
            followed_unit: if value.5.is_empty() {
                None
            } else {
                Some(value.5)
            },
            object_path: value.6,
            queued_job_id: if value.7 == 0 { None } else { Some(value.7) },
            job_type: value.8,
            job_object_path: value.9,
        }
    }
}
