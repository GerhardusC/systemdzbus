use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnitFile {
    /// The location of the unit file on disk, I think
    pub path: String,
    pub enablement_status: EnablementStatus,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EnablementStatus {
    Alias,
    Disabled,
    Enabled,
    EnabledRuntime,
    Generated,
    Static,
    Transient,
    Other(String),
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
            _ => EnablementStatus::Other(value),
        }
    }
}

impl Display for EnablementStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EnablementStatus::Alias => "alias",
            EnablementStatus::Disabled => "disabled",
            EnablementStatus::Enabled => "enabled",
            EnablementStatus::EnabledRuntime => "enabled-runtime",
            EnablementStatus::Generated => "generated",
            EnablementStatus::Static => "static",
            EnablementStatus::Transient => "transient",
            EnablementStatus::Other(val) => val,
        };
        f.write_str(value)
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
