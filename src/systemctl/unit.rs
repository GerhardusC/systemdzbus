use zbus::zvariant::OwnedObjectPath;

// NOTE: These docs are all from the man page of org.freedesktop.systemd1

#[derive(Debug)]
pub enum UnitLoadState {
    Stub,
    Loaded,
    NotLoaded,
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
            "not-found" => UnitLoadState::NotLoaded,
            "bad-setting" => UnitLoadState::BadSetting,
            "error" => UnitLoadState::Error,
            "merged" => UnitLoadState::Merged,
            "masked" => UnitLoadState::Masked,
            _ => UnitLoadState::Other(value),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Unit {
    // The primary unit name as string
    pub name: String,
    // The human readable description
    pub description: String,
    // The load state (i.e. whether the unit file has been loaded successfully)
    pub load_state: UnitLoadState,
    // The active state (i.e. whether the unit is currently started or not)
    pub active_state: UnitActiveState,
    // The sub state (a more fine-grained version of the active state that is specific to the unit type, which the active state is not)
    pub sub_state: String,
    // A unit that is being followed in its state by this unit, if there is any.
    pub followed_unit: Option<String>,
    // The unit object path
    pub object_path: OwnedObjectPath,
    // If there is a job queued for the job unit, the numeric job id
    pub queued_job_id: Option<u32>,
    // The job type as string
    pub job_type: String,
    // The job object path
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
