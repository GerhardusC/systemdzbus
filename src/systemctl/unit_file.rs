#[derive(Debug)]
pub struct UnitFile {
    pub path: String,
    pub enablement_status: EnablementStatus,
}

#[derive(Debug)]
pub enum EnablementStatus {
    Alias,
    Disabled,
    Enabled,
    EnabledRuntime,
    Generated,
    Other,
    Static,
    Transient,
}

impl From<String> for EnablementStatus {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "alias" => EnablementStatus::Alias,
            "disabled" => EnablementStatus::Disabled,
            "enabled" => EnablementStatus::Enabled,
            "enabled-runtime" => EnablementStatus::EnabledRuntime,
            "generated" => EnablementStatus::Generated,
            "static" => EnablementStatus::Static,
            "transient" => EnablementStatus::Transient,
            _ => EnablementStatus::Other,
        }
    }
}

impl From<(String, String)> for UnitFile {
    fn from(value: (String, String)) -> Self {
        Self {
            path: value.0,
            enablement_status: value.1.into(),
        }
    }
}
