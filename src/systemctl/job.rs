use zbus::zvariant::OwnedObjectPath;

#[derive(Debug)]
pub struct Job {
    /// The numeric job id
    pub id: u32,
    /// The primary unit name for this job
    pub unit_name: String,
    /// The job type as string
    pub job_type: String,
    /// The job state as string
    pub job_state: String,
    /// The job object path
    pub job_path: OwnedObjectPath,
    /// The unit object path
    pub unit_path: OwnedObjectPath,
}

impl
    From<(
        u32,
        String,
        String,
        String,
        OwnedObjectPath,
        OwnedObjectPath,
    )> for Job
{
    fn from(
        value: (
            u32,
            String,
            String,
            String,
            OwnedObjectPath,
            OwnedObjectPath,
        ),
    ) -> Self {
        Job {
            id: value.0,
            unit_name: value.1,
            job_type: value.2,
            job_state: value.3,
            job_path: value.4,
            unit_path: value.5,
        }
    }
}
