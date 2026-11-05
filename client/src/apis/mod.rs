use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod artifacts_api;
pub mod artifacts_export_api;
pub mod cac_api;
pub mod cmmc_assessment_dashboards_api;
pub mod cmmc_assessments_api;
pub mod cloud_resource_results_api;
pub mod coast_guard_system_fisma_metrics_dashboard_api;
pub mod container_scan_results_api;
pub mod controls_api;
pub mod device_scan_results_api;
pub mod hardware_baseline_api;
pub mod milestones_api;
pub mod organization_migration_status_dashboard_api;
pub mod pac_api;
pub mod poam_api;
pub mod registration_api;
pub mod software_baseline_api;
pub mod static_code_scans_api;
pub mod system_atciatc_dashboard_api;
pub mod system_application_findings_dashboards_api;
pub mod system_artifacts_dashboards_api;
pub mod system_associations_dashboard_api;
pub mod system_conmon_integration_status_dashboard_api;
pub mod system_connectivity_ccsd_dashboards_api;
pub mod system_critical_assets_dashboard_api;
pub mod system_device_findings_dashboards_api;
pub mod system_fisma_metrics_dashboard_api;
pub mod system_hardware_dashboards_api;
pub mod system_migration_status_dashboard_api;
pub mod system_poam_dashboards_api;
pub mod system_ports_protocols_dashboards_api;
pub mod system_privacy_dashboard_api;
pub mod system_questionnaire_dashboards_api;
pub mod system_roles_api;
pub mod system_security_controls_dashboards_api;
pub mod system_sensor_hardware_dashboards_api;
pub mod system_sensor_software_dashboards_api;
pub mod system_software_dashboards_api;
pub mod system_status_dashboard_api;
pub mod system_terms_conditions_dashboards_api;
pub mod system_vulnerability_dashboard_api;
pub mod system_workflows_dashboards_api;
pub mod systems_api;
pub mod test_api;
pub mod test_results_api;
pub mod user_system_assignments_dashboard_api;
pub mod vaombfisma_dashboard_api;
pub mod va_system_dashboards_api;
pub mod workflow_definitions_api;
pub mod workflow_instances_api;

pub mod configuration;
