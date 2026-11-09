//! emasser - Rust POC for the eMASS (EaaS) REST CLI.
//!
//! Port of `mitre/emasser`, a thin reference CLI that wraps the eMASS OpenAPI client.
//! This POC reproduces the dispatch + JSON formatting and exercises the `get`
//! command surface (`test connection`, `systems all`, `systems byId`,
//! `roles all`, `controls forSystem`, `poams forSystem` / `poams byPoamId`,
//! `testResults forSystem`, `milestones byPoamId` / `milestones byMilestoneId`,
//! `artifacts forSystem`, `cac controls` / `pac package` / `hardware assets` / `software assets`, `workflow_definitions forSite` / `workflow_instances all|byInstanceId`, `cmmc assessments`)
//! through the OpenAPI-generated Rust client. Authentication uses the same env-driven mTLS + API
//! key scheme as the reference tool (see `emasser configure`).

mod converters;

use anyhow::{anyhow, bail, Context, Result};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use emass_client::apis;
use emass_client::apis::configuration::ApiKey;

use emass_client::models::{
    ArtifactsRequestDeleteBodyInner,
    ArtifactsRequiredFields,
    CloudResourcesDeleteBodyInner,
    ContainerResourcesDeleteBodyInner,
    // `put` group (stripped typed models accepted by the client PUT endpoints)
    ControlsRequiredFields,
    HwBaselineReadOnlyFields,
    HwBaselineRequestDeleteBodyInner,
    HwBaselineRequiredFields,
    MilestonesRequestDeleteBodyInner,
    PoamIds,
    // `delete` group (typed body structs the client DELETE endpoints accept)
    PoamRequestDeleteBodyInner,
    StaticCodeApplicationPost,
    StaticCodeRequestPostBody,
    StaticCodeRequestPostBodyApplication,
    SwBaselineReadOnlyFields,
    SwBaselineRequestDeleteBodyInner,
    SwBaselineRequiredFields,
};

const HELP: &str = "\
emasser - Rust POC for the eMASS (EaaS) REST API

USAGE:
  emasser get <resource> [subcommand] [options]

RESOURCES:
  test connection                 Test the eMASS API connection
  systems all [--coams-id ID]     List systems
  systems byId --systemId <id>    Get a system by ID (policy: --policy <id>)
  roles all                       List system roles
  controls forSystem --systemId <id>     Get controls in a system (--acronyms <names>)
  poams forSystem --systemId <id>         Get POA&M items in a system
  poams byPoamId --systemId <id> --poamId <id>  Get one POA&M item
  testResults forSystem --systemId <id>   Get test results in a system
  milestones byPoamId --systemId <id> --poamId <id>  Get milestones for a POA&M item
  milestones byMilestoneId --poamId <id> --milestoneId <id>  Get one milestone
  artifacts forSystem --systemId <id>     Get artifacts for a system
  cac controls --systemId <id>            Get CAC location of controls (--controlAcronyms)
  pac package --systemId <id>             Get PAC location of the system package
  hardware assets --systemId <id>         Get hardware assets (--pageIndex --pageSize)
  software assets --systemId <id>         Get software assets (--pageIndex --pageSize)
  workflow_definitions forSite            Get all workflow definitions
  workflow_instances all                  Get all workflow instances (--status --includeComments)
  workflow_instances byInstanceId --workflowInstanceId <id>  Get one workflow instance
  cmmc assessments --sinceDate <date>     ccmc assessments --sinceDate <date>     Get CMC assessment information
  dashboards <name> --orgId <id>          Get a system dashboard (--excludeinherited --pageIndex --pageSize)
  system id [--systemName <name>] [--systemOwner <owner>]    Find system(s) by name and/or owner
  system byId --systemId <id>             Get a single system by id (optional --policy)
  artifacts export --systemId <id> --filename <f>   Download an artifact (-C --compress, -o --printToStdout)

ENVIRONMENT (see `emasser configure`):
  EMASSER_HOST_URL          eMASS host, e.g. https://emas.example.gov
  EMASSER_API_KEY           api-key / user-uid value
  EMASSER_KEY_FILE_PATH     mTLS private key PEM
  EMASSER_CERT_FILE_PATH    mTLS client cert PEM
  EMASSER_KEY_FILE_PASSWORD password for the private key (optional)
  EMASSER_USER_UID          user-uid value
  EMASSER_PREFER            Prefer header for the mock, e.g. `code=200, dynamic=true`
  EMASSER_VERIFY_SSL        'false' to accept self-signed servers
  EMASSER_CLI_DISPLAY_NULL  'false' to strip null fields from output
  EMASSER_EPOCH_TO_DATETIME 'true' to convert epoch timestamps to datetime
  EMASSER_DOWNLOAD_DIR      directory for `get` file downloads
";

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}

async fn run() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.to_lowercase()) {
        None => print!("{HELP}"),
        Some(ref s) if matches!(s.as_str(), "-h" | "--help" | "help" | "h") => print!("{HELP}"),
        Some(s) if s == "get" => dispatch_get(&args[1..]).await?,
        Some(s) if s == "post" => dispatch_post(&args[1..]).await?,
        Some(s) if s == "put" => dispatch_put(&args[1..]).await?,
        Some(s) if s == "delete" => dispatch_delete(&args[1..]).await?,
        Some(s) if s == "configure" => dispatch_configure(&args[1..]).await?,
        Some(s) => bail!("command `{s}` is not yet ported (POC supports `get`, `post`, `put`, `delete`, `configure`)"),
    }
    Ok(())
}

async fn dispatch_get(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("usage: emasser get <resource> [subcommand] [options] (e.g. `emasser get test connection`)");
    }
    match args[0].to_lowercase().as_str() {
        "test" => cmd_test_connection().await,
        "systems" => cmd_systems(&args[1..]).await,
        "roles" => cmd_roles(&args[1..]).await,
        "controls" => cmd_controls(&args[1..]).await,
        "poams" => cmd_poams(&args[1..]).await,
        "testresults" => cmd_test_results(&args[1..]).await,
        "test_results" => cmd_test_results(&args[1..]).await,
        "milestones" => cmd_milestones(&args[1..]).await,
        "artifacts" => cmd_artifacts(&args[1..]).await,
        "cac" => cmd_cac(&args[1..]).await,
        "pac" => cmd_pac(&args[1..]).await,
        "hardware" => cmd_hardware(&args[1..]).await,
        "software" => cmd_software(&args[1..]).await,
        "workflow_definitions" => cmd_workflow_definitions(&args[1..]).await,
        "workflow_instances" => cmd_workflow_instances(&args[1..]).await,
        "cmmc" => cmd_cmmc(&args[1..]).await,
        "dashboards" => cmd_dashboards(&args[1..]).await,
        "system" => cmd_system(&args[1..]).await,
        other => bail!("unknown resource `{other}` (try: test, systems, roles, controls, poams, testResults, milestones, artifacts, cac, pac, hardware, software, workflow_definitions, workflow_instances, cmmc, dashboards, system)"),
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// `get test connection`
async fn cmd_test_connection() -> Result<()> {
    let config = build_config()?;
    let resp = apis::test_api::test_connection(&config)
        .await
        .context("test connection request failed")?;
    print_output(&resp);
    Ok(())
}

/// `get systems all` | `get systems byId`
async fn cmd_systems(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "all".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "all" => {
            let opts = parse_options(opts_args);
            match apis::systems_api::get_systems(
                &config,
                opts.get("coams-id").map(String::as_str),
                opts.get("ditpr-id").map(String::as_str),
                opts.get("include-decommissioned").map(|v| v == "true"),
                opts.get("include-ditpr-metrics").map(|v| v == "true"),
                opts.get("policy").map(String::as_str),
                opts.get("registration-type").map(String::as_str),
                opts.get("reports-for-scorecard").map(|v| v == "true"),
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        "byid" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get systems byId` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            match apis::systems_api::get_system(
                &config,
                sys_id,
                opts.get("policy").map(String::as_str),
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get systems all|byId (e.g. `emasser get systems all`)"),
    }
}

/// `get roles all` | `get roles byCategory`
async fn cmd_roles(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "all".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "all" => {
            match apis::system_roles_api::get_system_roles(&config).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        "bycategory" => {
            let opts = parse_options(opts_args);
            let role_category: &str = opts
                .get("roleCategory")
                .context("`get roles byCategory` requires --roleCategory <id> (PAC|CAC|Other)")?
                .as_str();
            let role: &str = opts
                .get("role")
                .context("`get roles byCategory` requires --role <id>")?
                .as_str();
            let policy = opts.get("policy").map(String::as_str);
            match apis::system_roles_api::get_system_roles_by_category_id(
                &config, role_category, role, policy,
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        _ => bail!("usage: emasser get roles all|byCategory (e.g. `emasser get roles byCategory --roleCategory PAC --role SCA`)"),
    }
    Ok(())
}

/// `get controls forSystem`
async fn cmd_controls(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "forsystem".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "forsystem" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get controls forSystem` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            match apis::controls_api::get_system_controls(
                &config,
                sys_id,
                opts.get("acronyms").map(String::as_str),
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get controls forSystem (e.g. `emasser get controls forSystem --systemId 5`)"),
    }
}

/// `get poams forSystem` | `get poams byPoamId`
async fn cmd_poams(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "forsystem".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "forsystem" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get poams forSystem` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            let scds = opts.get("scheduledCompletionDateStart").map(String::as_str);
            let scde = opts.get("scheduledCompletionDateEnd").map(String::as_str);
            let acronyms = opts.get("controlAcronyms").map(String::as_str);
            let procedures = opts.get("assessmentProcedures").map(String::as_str);
            let ccis = opts.get("ccis").map(String::as_str);
            // emass CLI defaults `systemOnly` to false (not absent); the client
            // always injects `?systemOnly=false`. Mirror that so the request bytes match.
            let system_only = match opts.get("systemOnly") {
                Some(v) => Some(v == "true"),
                None => Some(false),
            };
            match apis::poam_api::get_system_poams(
                &config,
                sys_id,
                scds,
                scde,
                acronyms,
                procedures,
                ccis,
                system_only,
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        "bypoamid" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get poams byPoamId` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            let poam_id: i32 = opts
                .get("poamId")
                .context("`get poams byPoamId` requires --poamId <id>")?
                .parse()
                .context("--poamId must be an integer")?;
            match apis::poam_api::get_system_poams_by_poam_id(&config, sys_id, poam_id)
                .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get poams forSystem|byPoamId (e.g. `emasser get poams forSystem --systemId 5`)"),
    }
}

/// `get testResults forSystem`
async fn cmd_test_results(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "forsystem".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "forsystem" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get testResults forSystem` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            match apis::test_results_api::get_system_test_results(
                &config,
                sys_id,
                opts.get("controlAcronyms").map(String::as_str),
                opts.get("assessmentProcedures").map(String::as_str),
                opts.get("ccis").map(String::as_str),
                opts.get("latestOnly").map(|v| v == "true"),
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get testResults forSystem (e.g. `emasser get testResults forSystem --systemId 5`)"),
    }
}

/// `get milestones byPoamId` | `get milestones byMilestoneId`
async fn cmd_milestones(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "bypoamid".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "bypoamid" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get milestones byPoamId` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            let poam_id: i32 = opts
                .get("poamId")
                .context("`get milestones byPoamId` requires --poamId <id>")?
                .parse()
                .context("--poamId must be an integer")?;
            let scds = opts.get("scheduledCompletionDateStart").map(String::as_str);
            let scde = opts.get("scheduledCompletionDateEnd").map(String::as_str);
            match apis::milestones_api::get_system_milestones_by_poam_id(
                &config, sys_id, poam_id, scds, scde,
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        "bymilestoneid" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get milestones byMilestoneId` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            let poam_id: i32 = opts
                .get("poamId")
                .context("`get milestones byMilestoneId` requires --poamId <id>")?
                .parse()
                .context("--poamId must be an integer")?;
            let milestone_id: i32 = opts
                .get("milestoneId")
                .context("`get milestones byMilestoneId` requires --milestoneId <id>")?
                .parse()
                .context("--milestoneId must be an integer")?;
            match apis::milestones_api::get_system_milestones_by_poam_id_and_milestone_id(
                &config, sys_id, poam_id, milestone_id,
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get milestones byPoamId|byMilestoneId (e.g. `emasser get milestones byPoamId --systemId 5 --poamId 1`)"),
    }
}

/// `get artifacts forSystem`
async fn cmd_artifacts(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "forsystem".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "forsystem" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get artifacts forSystem` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            match apis::artifacts_api::get_system_artifacts(
                &config,
                sys_id,
                opts.get("filename").map(String::as_str),
                opts.get("controlAcronyms").map(String::as_str),
                opts.get("assessmentProcedures").map(String::as_str),
                opts.get("ccis").map(String::as_str),
                opts.get("systemOnly").map(|v| v == "true"),
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        "export" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = parse_opt_i32(&opts, "systemId", "artifacts export systemId")?
                .context("artifacts export: systemId must be provided")?;
            let filename: &str = opts
                .get("filename")
                .context("artifacts export: filename must be provided")?
                .as_str();
            let compress = opts.get("compress").map(|v| v == "true");
            let print_to_stdout = opts.get("printToStdout").map(|v| v == "true").unwrap_or(false);
            let resp = apis::artifacts_export_api::get_system_artifacts_export(
                &config,
                sys_id,
                filename,
                compress,
            )
            .await
            .context("artifacts export: request failed")?;
            if print_to_stdout {
                std::io::stdout()
                    .write_all(&resp.bytes().await.context("artifacts export: reading body")?)?;
                println!();
            } else {
                let fname = format!("{sys_id}_{filename}");
                let dl = format!("./downloads/{fname}");
                std::fs::write(&dl, resp.bytes().await.context("artifacts export: reading body")?)
                    .with_context(|| format!("artifacts export: writing {dl}"))?;
                println!("Saved artifact -> {dl}");
            }
            Ok(())
        }
        _ => bail!("usage: emasser get artifacts forSystem|export --systemId <id> (--filename <f> [-C --compress, -o --printToStdout])"),
    }
}

// ---------------------------------------------------------------------------
// get dashboards  (System Dashboard subcommands)
// ---------------------------------------------------------------------------

// Every dashboard shares the client signature
// `(config, org_id, excludeinherited, page_index, page_size)` and returns
// `GetSystemStatusDetails200Response`, so we dispatch by macro instead of one
// match arm per dashboard.
macro_rules! dashboards_call {
    ($config:ident, $org_id:ident, $exclude_inherited:ident, $page_index:ident, $page_size:ident, $mod:ident::$fn:ident) => {{
        let resp = apis::$mod::$fn(
            &$config,
            $org_id,
            $exclude_inherited,
            $page_index,
            $page_size,
        )
        .await
        .context("dashboards: request failed")?;
        print_output(&resp);
    }};
}

async fn cmd_dashboards(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let opts_args = args.get(1..).unwrap_or(&[]);
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "status_details".into());
    let opts = parse_options(opts_args);
    let org_id: i32 = parse_opt_i32(&opts, "orgId", "dashboards orgId")?
        .context("`get dashboards <name>` requires --orgId <id>")?;
    let exclude_inherited = opts.get("excludeinherited").map(|v| v == "true");
    let page_index = parse_opt_i32(&opts, "pageIndex", "dashboards pageIndex")?;
    let page_size = parse_opt_i32(&opts, "pageSize", "dashboards pageSize")?;

    match sub.as_str() {
        "status_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_status_dashboard_api::get_system_status_details),
        "terms_conditions_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_terms_conditions_dashboards_api::get_system_terms_conditions_summary),
        "terms_conditions_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_terms_conditions_dashboards_api::get_system_terms_conditions_details),
        "connectivity_ccsd_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_connectivity_ccsd_dashboards_api::get_system_connectivity_ccsd_summary),
        "connectivity_ccsd_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_connectivity_ccsd_dashboards_api::get_system_connectivity_ccsd_details),
        "atc_iatc_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_atciatc_dashboard_api::get_system_atc_iatc_details),
        "questionnaire_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_questionnaire_dashboards_api::get_system_questionnaire_summary),
        "questionnaire_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_questionnaire_dashboards_api::get_system_questionnaire_details),
        "workflows_history_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_workflows_dashboards_api::get_system_workflows_history_summary),
        "workflows_history_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_workflows_dashboards_api::get_system_workflows_history_details),
        "workflows_history_stage_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_workflows_dashboards_api::get_system_workflows_history_stage_details),
        "control_compliance_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_security_controls_dashboards_api::get_system_control_compliance_summary),
        "security_control_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_security_controls_dashboards_api::get_system_security_control_details),
        "assessment_procedures_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_security_controls_dashboards_api::get_system_assessment_procedures_details),
        "poam_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_poam_dashboards_api::get_system_poam_summary),
        "poam_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_poam_dashboards_api::get_system_poam_details),
        "artifacts_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_artifacts_dashboards_api::get_system_artifacts_summary),
        "artifacts_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_artifacts_dashboards_api::get_system_artifacts_details),
        "hardware_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_hardware_dashboards_api::get_system_hardware_summary),
        "hardware_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_hardware_dashboards_api::get_system_hardware_details),
        "sensor_hardware_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_sensor_hardware_dashboards_api::get_system_sensor_hardware_summary),
        "sensor_hardware_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_sensor_hardware_dashboards_api::get_system_sensor_hardware_details),
        "software_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_software_dashboards_api::get_system_software_summary),
        "software_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_software_dashboards_api::get_system_software_details),
        "sensor_software_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_sensor_software_dashboards_api::get_system_sensor_software_summary),
        "sensor_software_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_sensor_software_dashboards_api::get_system_sensor_software_details),
        "sensor_software_counts" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_sensor_software_dashboards_api::get_system_sensor_software_counts),
        "critical_assets_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_critical_assets_dashboard_api::get_system_critical_assets_summary),
        "vulnerability_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_vulnerability_dashboard_api::get_system_vulnerability_summary),
        "device_findings_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_device_findings_dashboards_api::get_system_device_findings_summary),
        "device_findings_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_device_findings_dashboards_api::get_system_device_findings_details),
        "application_findings_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_application_findings_dashboards_api::get_system_application_findings_summary),
        "application_findings_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_application_findings_dashboards_api::get_system_application_findings_details),
        "ports_protocols_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_ports_protocols_dashboards_api::get_system_ports_protocols_summary),
        "ports_protocols_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_ports_protocols_dashboards_api::get_system_ports_protocols_details),
        "integration_status_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_conmon_integration_status_dashboard_api::get_system_common_integration_status_summary),
        "associations_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_associations_dashboard_api::get_system_associations_details),
        "system_migration_status_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_migration_status_dashboard_api::get_system_migration_status_summary),
        "fisma_metrics" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_fisma_metrics_dashboard_api::get_system_fisma_metrics),
        "privacy_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, system_privacy_dashboard_api::get_system_privacy_summary),
        "assignments_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, user_system_assignments_dashboard_api::get_user_system_assignments_details),
        "organization_migration_status_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, organization_migration_status_dashboard_api::get_organization_migration_status_summary),
        "coastguard_fisma_metrics" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, coast_guard_system_fisma_metrics_dashboard_api::get_coast_guard_system_fisma_metrics),
        "fisma_saop_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, vaombfisma_dashboard_api::get_va_omb_fsma_saop_summary),
        "va_icamp_tableau_poam_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_icamp_tableau_poam_details),
        "va_aa_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_aa_summary),
        "va_a2_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_a2_summary),
        "va_pl_109_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_pl109_reporting_summary),
        "fisma_inventory_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_fisma_invetory_summary),
        "fisma_inventory_crypto_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_fisma_invetory_crypto_summary),
        "threat_risk_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_threat_risk_summary),
        "threat_risk_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_threat_source_details),
        "threat_architecture_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, va_system_dashboards_api::get_va_system_threat_architecture_details),
        "cmmc_status_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, cmmc_assessment_dashboards_api::get_cmmc_assessment_status_summary),
        "cmmc_compliance_summary" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, cmmc_assessment_dashboards_api::get_cmmc_assessment_requirements_compliance_summary),
        "cmmc_security_requirements_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, cmmc_assessment_dashboards_api::get_cmmc_assessment_security_requirements_details),
        "cmmc_requirement_objectives_details" => dashboards_call!(config, org_id, exclude_inherited, page_index, page_size, cmmc_assessment_dashboards_api::get_cmmc_assessment_requirement_objectives_details),
        _ => bail!("usage: emasser get dashboards <dashboard> --orgId <id>"),
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// get system  (id | byId)
// ---------------------------------------------------------------------------
async fn cmd_system(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let opts_args = args.get(1..).unwrap_or(&[]);
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "byid".into());
    match sub.as_str() {
        "id" => {
            let opts = parse_options(opts_args);
            let system_name: Option<String> = opts.get("system_name").map(String::to_string);
            let system_owner: Option<String> = opts.get("system_owner").map(String::to_string);
            if system_name.is_none() && system_owner.is_none() {
                bail!("usage: emasser get system id [--system_name <name>] [--system_owner <owner>] (one required)");
            }
            let resp =
                apis::systems_api::get_systems(&config, None, None, None, None, None, None, None)
                    .await
                    .context("system id: request failed")?;
            let matched: Vec<&emass_client::models::Systems> = resp
                .data
                .as_ref()
                .map(|systems| {
                    systems
                        .iter()
                        .filter(|s| {
                            let nm = s
                                .name
                                .as_ref()
                                .and_then(|inner| inner.as_ref().map(|v| v.as_str()));
                            let ow = s
                                .owning_organization
                                .as_ref()
                                .and_then(|inner| inner.as_ref().map(|v| v.as_str()));
                            match (system_name.as_deref(), system_owner.as_deref()) {
                                (Some(n), None) => nm == Some(n),
                                (None, Some(o)) => ow == Some(o),
                                (Some(n), Some(o)) => nm == Some(n) && ow == Some(o),
                                (None, None) => false,
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();
            if matched.is_empty() {
                bail!("no system matched the given system_name/system_owner");
            }
            for s in &matched {
                let nm = s
                    .name
                    .as_ref()
                    .and_then(|inner| inner.as_ref().map(|v| v.as_str()));
                let ow = s
                    .owning_organization
                    .as_ref()
                    .and_then(|inner| inner.as_ref().map(|v| v.as_str()));
                println!(
                    "{} - {} - {}",
                    s.system_id.unwrap_or_default(),
                    ow.unwrap_or(""),
                    nm.unwrap_or("")
                );
            }
            Ok(())
        }
        "byid" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = parse_opt_i32(&opts, "systemId", "system byId systemId")?
                .context("`get system byId` requires --systemId <id>")?;
            let policy = opts.get("policy").map(String::as_str);
            let resp = apis::systems_api::get_system(&config, sys_id, policy)
                .await
                .context("system byId: request failed")?;
            print_output(&resp);
            Ok(())
        }
        _ => bail!("usage: emasser get system id|byId (see `--help`)"),
    }
}

// ---------------------------------------------------------------------------
// Option helper
// ---------------------------------------------------------------------------

/// Parse an optional i32 flag (e.g. `--pageIndex`); `None` when absent.
fn parse_opt_i32(
    opts: &HashMap<String, String>,
    key: &str,
    ctx: &'static str,
) -> Result<Option<i32>> {
    match opts.get(key) {
        Some(v) => Ok(v.parse::<i32>().context(ctx).map(Some)?),
        None => Ok(None),
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// `get cac controls`
async fn cmd_cac(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "controls".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "controls" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get cac controls` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            match apis::cac_api::get_system_cac(
                &config,
                sys_id,
                opts.get("controlAcronyms").map(String::as_str),
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => {
            bail!("usage: emasser get cac controls (e.g. `emasser get cac controls --systemId 5`)")
        }
    }
}

/// `get pac package`
async fn cmd_pac(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "package".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "package" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get pac package` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            match apis::pac_api::get_system_pac(&config, sys_id).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get pac package (e.g. `emasser get pac package --systemId 5`)"),
    }
}

/// `get hardware assets`
async fn cmd_hardware(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "assets".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "assets" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get hardware assets` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            let page_index = parse_opt_i32(&opts, "pageIndex", "--pageIndex must be an integer")?;
            let page_size = parse_opt_i32(&opts, "pageSize", "--pageSize must be an integer")?;
            match apis::hardware_baseline_api::get_system_hw_baseline(
                &config, sys_id, page_index, page_size,
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!(
            "usage: emasser get hardware assets (e.g. `emasser get hardware assets --systemId 5`)"
        ),
    }
}

/// `get software assets`
async fn cmd_software(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "assets".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "assets" => {
            let opts = parse_options(opts_args);
            let sys_id: i32 = opts
                .get("systemId")
                .context("`get software assets` requires --systemId <id>")?
                .parse()
                .context("--systemId must be an integer")?;
            let page_index = parse_opt_i32(&opts, "pageIndex", "--pageIndex must be an integer")?;
            let page_size = parse_opt_i32(&opts, "pageSize", "--pageSize must be an integer")?;
            match apis::software_baseline_api::get_system_sw_baseline(
                &config, sys_id, page_index, page_size,
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!(
            "usage: emasser get software assets (e.g. `emasser get software assets --systemId 5`)"
        ),
    }
}

/// `get workflow_definitions forSite`
async fn cmd_workflow_definitions(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "forsite".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "forsite" => {
            let opts = parse_options(opts_args);
            match apis::workflow_definitions_api::get_workflow_definitions(
                &config,
                opts.get("includeInactive").map(|v| v == "true"),
                opts.get("registrationType").map(String::as_str),
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get workflow_definitions forSite (e.g. `emasser get workflow_definitions forSite`)"),
    }
}

/// `get workflow_instances all` | `get workflow_instances byInstanceId`
async fn cmd_workflow_instances(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "all".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "all" => {
            let opts = parse_options(opts_args);
            match apis::workflow_instances_api::get_system_workflow_instances(
                &config,
                opts.get("includeComments").map(|v| v == "true"),
                opts.get("includeDecommissionSystems").map(|v| v == "true"),
                parse_opt_i32(&opts, "pageIndex", "--pageIndex must be an integer")?,
                opts.get("sinceDate").map(String::as_str),
                opts.get("status").map(String::as_str),
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        "byinstanceid" => {
            let opts = parse_options(opts_args);
            let wid: i32 = opts
                .get("workflowInstanceId")
                .context("`get workflow_instances byInstanceId` requires --workflowInstanceId <id>")?
                .parse()
                .context("--workflowInstanceId must be an integer")?;
            match apis::workflow_instances_api::get_system_workflow_instances_by_workflow_instance_id(
                &config, wid,
            )
            .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get workflow_instances all|byInstanceId (e.g. `emasser get workflow_instances all`)"),
    }
}

/// `get cmmc assessments`
async fn cmd_cmmc(args: &[String]) -> Result<()> {
    let config = build_config()?;
    let sub = args
        .first()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "assessments".into());
    let opts_args = args.get(1..).unwrap_or(&[]);
    match sub.as_str() {
        "assessments" => {
            let opts = parse_options(opts_args);
            let since_date = opts
                .get("sinceDate")
                .context("`get cmmc assessments` requires --sinceDate <date>")?
                .clone();
            match apis::cmmc_assessments_api::get_cmmc_assessments(&config, &since_date)
                .await
            {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
            Ok(())
        }
        _ => bail!("usage: emasser get cmmc assessments (e.g. `emasser get cmmc assessments --sinceDate 2026-01-01`)"),
    }
}

// ---------------------------------------------------------------------------
// Config + HTTP client (mTLS + API key, matching the reference tool)
// ---------------------------------------------------------------------------

fn build_config() -> Result<emass_client::apis::configuration::Configuration> {
    let host = env::var("EMASSER_HOST_URL").with_context(|| "EMASSER_HOST_URL is not set")?;
    let api_key = env::var("EMASSER_API_KEY").with_context(|| "EMASSER_API_KEY is not set")?;

    let cert = env::var("EMASSER_CERT_FILE_PATH")
        .ok()
        .filter(|s| !s.is_empty());
    let key = env::var("EMASSER_KEY_FILE_PATH")
        .ok()
        .filter(|s| !s.is_empty());
    let password = env::var("EMASSER_KEY_FILE_PASSWORD")
        .ok()
        .filter(|s| !s.is_empty());
    let verify_ssl = match env::var("EMASSER_VERIFY_SSL") {
        Ok(v) => v != "false",
        Err(_) => true,
    };

    let client = build_client(
        verify_ssl,
        cert.as_deref(),
        key.as_deref(),
        password.as_deref(),
    )?;

    let mut config = emass_client::apis::configuration::Configuration::default();
    // eMASS forces the https scheme. The generated client builds the absolute
    // request URL from `base_path`; without a scheme reqwest errors with
    // "relative URL without a base", so normalize `host` to an https base.
    let scheme_host = if host.starts_with("http://") || host.starts_with("https://") {
        host.clone()
    } else {
        format!("https://{host}")
    };
    config.base_path = scheme_host.trim_end_matches('/').to_string();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: api_key,
    });
    // The `userId` and `mockType` security schemes in the eMASS spec are distinct
    // from `apiKey`, so they need their own config fields (openapi-generator
    // originally dumped all three into api_key). These are env-driven; the
    // defaults target the local mock's dynamic mode.
    let user_uid = env::var("EMASSER_USER_UID").unwrap_or_else(|_| "123".to_string());
    config.user_uid = Some(user_uid);
    // emass never sends a `Prefer` header (no `prefer` in its config), so the
    // client must fall back to its default. Leave it unset unless the caller
    // explicitly sets `EMASSER_PREFER`; the client's None-fallback is the
    // api-key, which the mock treats as the stable `data:[null]` response.
    if let Ok(p) = env::var("EMASSER_PREFER") {
        config.prefer = Some(p);
    }
    // Wire the built client (danger_accept_invalid_certs + mTLS identity) into
    // the generated configuration. Without this the request falls back to the
    // default reqwest client, which skips the verify flag and rejects the mock
    // server's issuer (rustls_platform_verifier => UnknownIssuer).
    config.client = client;
    Ok(config)
}

fn build_client(
    verify_ssl: bool,
    cert: Option<&str>,
    key: Option<&str>,
    password: Option<&str>,
) -> Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder();
    if !verify_ssl {
        builder = builder.danger_accept_invalid_certs(true);
    }
    match (cert, key) {
        (Some(crt), Some(k)) => {
            let crt_pem = fs::read(crt).with_context(|| format!("reading cert file {crt}"))?;
            let k_pem = fs::read(k).with_context(|| format!("reading key file {k}"))?;
            let ident = mk_identity(&crt_pem, &k_pem, password)
                .context("building client certificate identity")?;
            builder = builder.identity(ident);
        }
        (Some(_), None) => bail!("EMASSER_CERT_FILE_PATH is set but EMASSER_KEY_FILE_PATH is not"),
        (None, Some(_)) => bail!("EMASSER_KEY_FILE_PATH is set but EMASSER_CERT_FILE_PATH is not"),
        (None, None) => eprintln!(
            "warning: no mTLS client certificate configured (set EMASSER_CERT_FILE_PATH / EMASSER_KEY_FILE_PATH)"
        ),
    }
    // Security headers (`api-key`, `user-uid`, `Prefer`) are injected by the
    // generated client from the Configuration fields populated in build_config,
    // which also handle env-driven defaults and the api_key fallback.
    builder.build().context("building reqwest client")
}

fn mk_identity(cert: &[u8], key: &[u8], password: Option<&str>) -> Result<reqwest::Identity> {
    // With `rustls` only `Identity::from_pem` is available (it parses a PEM
    // buffer containing a private key and at least one certificate). The reference
    // tool supports encrypted PKCS#12/PKCS#8 keys via an optional password;
    // this POC supports PEM keys only (plaintext), which is what most
    // certificate bundles use.
    if password.map(|p| !p.is_empty()).unwrap_or(false) {
        eprintln!(
            "warning: POC `from_pem` does not decrypt password-protected keys; set key files without a password"
        );
    }
    // rustls_pki_types requires: key + at least one cert in the same buffer.
    let mut buf = Vec::new();
    buf.extend_from_slice(key);
    buf.extend_from_slice(cert);
    reqwest::Identity::from_pem(&buf)
        .context("loading client certificate/key identity via from_pem")
}

// ---------------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------------

fn print_output<T: serde::Serialize>(resp: &T) {
    let display_nulls = env::var("EMASSER_CLI_DISPLAY_NULL")
        .ok()
        .map(|v| v == "true")
        .unwrap_or(true);
    let display_dt = env::var("EMASSER_EPOCH_TO_DATETIME")
        .ok()
        .map(|v| v == "true")
        .unwrap_or(false);
    let value: serde_json::Value = serde_json::to_value(resp).unwrap_or(serde_json::Value::Null);
    let formatted = converters::to_output_hash(&value, display_nulls, display_dt);
    println!("{}", converters::pretty_generate(&formatted));
}

fn print_err<T>(err: &emass_client::apis::Error<T>) {
    match err {
        emass_client::apis::Error::ResponseError(rc) => {
            // eMASS error bodies look like: { "Error message": "...", "Response body": "{...}" }
            let parsed: serde_json::Value = serde_json::from_str(&rc.content)
                .unwrap_or(serde_json::Value::String(rc.content.clone()));
            println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
        }
        emass_client::apis::Error::Reqwest(e) => {
            eprintln!("connection error: {e}");
        }
        emass_client::apis::Error::Serde(e) => {
            eprintln!("unexpected response (expected a valid JSON body): {e}");
        }
        emass_client::apis::Error::Io(e) => {
            eprintln!("io error: {e}");
        }
    }
}

// ---------------------------------------------------------------------------
// tiny option parser (--flag value | --flag)
// ---------------------------------------------------------------------------

fn parse_options(args: &[String]) -> HashMap<String, String> {
    let mut opts = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if let Some(flag) = a.strip_prefix("--") {
            let key = flag.to_string();
            if i + 1 < args.len() {
                opts.insert(key, args[i + 1].clone());
                i += 2;
            } else {
                opts.insert(key, "true".into());
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    opts
}

// ========================================================================
//  `post` group  (mirrors lib/emasser/lib/emasser/post.rb)
//  The client exposes direct call signatures; raw-JSON arms feed the client
//  exactly the JSON arrays emasser sends, while the typed-model arms set only
//  the fields the generated structs expose.
// ========================================================================

async fn dispatch_post(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("usage: emasser post <poams|testResults|milestones|cac|pac|artifacts|hardware|software|cloud|scan> [add|update] [options]");
    }
    let lower: Vec<String> = args
        .iter()
        .take_while(|s| !s.starts_with("--"))
        .map(|s| s.to_lowercase())
        .collect();
    let sub: Vec<&str> = lower.iter().map(String::as_str).collect();
    let flags_start = args
        .iter()
        .position(|s| s.starts_with("--"))
        .unwrap_or(args.len());
    let opts = parse_options(&args[flags_start..]);
    match sub.first().copied() {
        Some("poams") => match sub.as_slice() {
            ["poams", "add"] => cmd_post_poams_add(&opts).await,
            ["poams", "add", "with", "milestones"] => {
                cmd_post_poams_add_with_milestones(&opts).await
            }
            ["poams", "add", "with", "risk", "analysis"] => {
                cmd_post_poams_add_with_risk_analysis(&opts).await
            }
            _ => bail!("usage: emasser post poams add[| with milestones| with risk analysis]"),
        },
        Some("test_results") => match sub.as_slice() {
            ["test_results", "add"] => cmd_post_test_results_add(&opts).await,
            ["test_results", "update"] => cmd_post_test_results_update(&opts).await,
            _ => bail!("usage: emasser post test_results add|update"),
        },
        Some("milestones") => match sub.as_slice() {
            ["milestones", "add"] => cmd_post_milestones_add(&opts).await,
            _ => bail!("usage: emasser post milestones add"),
        },
        Some("cac") => match sub.as_slice() {
            ["cac", "add"] => cmd_post_cac_add(&opts).await,
            _ => bail!("usage: emasser post cac add"),
        },
        Some("pac") => match sub.as_slice() {
            ["pac", "add"] => cmd_post_pac_add(&opts).await,
            _ => bail!("usage: emasser post pac add"),
        },
        Some("artifacts") => match sub.as_slice() {
            ["artifacts", "add"] => cmd_post_artifacts_add(&opts).await,
            ["artifacts", "update"] => cmd_post_artifacts_update(&opts).await,
            _ => bail!("usage: emasser post artifacts add|update"),
        },
        Some("hardware") => match sub.as_slice() {
            ["hardware", "add"] => cmd_post_hardware_add(&opts).await,
            _ => bail!("usage: emasser post hardware add"),
        },
        Some("software") => match sub.as_slice() {
            ["software", "add"] => cmd_post_software_add(&opts).await,
            _ => bail!("usage: emasser post software add"),
        },
        Some("cloud") => match sub.as_slice() {
            ["cloud", "add"] => cmd_post_cloud_add(&opts).await,
            _ => bail!("usage: emasser post cloud add"),
        },
        Some("container") => match sub.as_slice() {
            ["container", "add"] => cmd_post_container_add(&opts).await,
            _ => bail!("usage: emasser post container add"),
        },
        Some("scan") => match sub.as_slice() {
            ["scan", "add", "static", "code"] => cmd_post_scan_static_code(&opts).await,
            ["scan", "add"] => cmd_post_scan_add(&opts).await,
            ["scan", "clear"] => cmd_post_scan_clear(&opts).await,
            _ => bail!("usage: emasser post scan add|add static code|clear"),
        },
        Some("register") => match sub.as_slice() {
            ["register", "cert"] => cmd_post_register_cert(&opts).await,
            _ => bail!("usage: emasser post register cert"),
        },
        Some("device_scans") => match sub.as_slice() {
            ["device_scans", "upload"] => cmd_post_device_scans_upload(&opts).await,
            _ => bail!("usage: emasser post device_scans upload"),
        },
        _ => bail!("usage: emasser post <poams|testResults|...|scan_findings> ..."),
    }
}

fn require_int(opts: &HashMap<String, String>, key: &str) -> Result<i32> {
    match opts.get(key) {
        Some(v) => v
            .parse::<i32>()
            .with_context(|| format!("`--{key}` must be an integer")),
        None => bail!("`--{key}` is required"),
    }
}

fn parse_status(s: &str) -> emass_client::models::poam_required_fields::Status {
    match s.to_lowercase().as_str() {
        "completed" => emass_client::models::poam_required_fields::Status::Completed,
        "archived" => emass_client::models::poam_required_fields::Status::Archived,
        "notapplicable" | "not_applicable" => {
            emass_client::models::poam_required_fields::Status::NotApplicable
        }
        "riskaccepted" | "risk accepted" => {
            emass_client::models::poam_required_fields::Status::RiskAccepted
        }
        _ => emass_client::models::poam_required_fields::Status::Ongoing,
    }
}

fn parse_bool(s: &str) -> bool {
    matches!(
        s.to_lowercase().as_str(),
        "true" | "t" | "yes" | "y" | "1" | "on"
    )
}

// Parse --milestones / static-code --findings JSON: accept an array or a
// single object; return a Vec of milestone/finding JSON objects.
fn parse_json_list(s: &str) -> Result<Vec<serde_json::Value>> {
    let v: serde_json::Value = serde_json::from_str(s).context("failed to parse JSON argument")?;
    if let Some(arr) = v.as_array() {
        Ok(arr.clone())
    } else {
        Ok(vec![v])
    }
}

fn parse_severity(s: &str) -> emass_client::models::static_code_application_post::RawSeverity {
    match s.to_lowercase().as_str() {
        "critical" => emass_client::models::static_code_application_post::RawSeverity::Critical,
        "high" => emass_client::models::static_code_application_post::RawSeverity::High,
        "moderate" | "medium" => {
            emass_client::models::static_code_application_post::RawSeverity::Medium
        }
        "low" => emass_client::models::static_code_application_post::RawSeverity::Low,
        _ => emass_client::models::static_code_application_post::RawSeverity::Low,
    }
}

// ---- poams ---------------------------------------------------------------
/// Build the full POA&M body for `post poams add`, mirroring the reference
/// `lib/emass/post.rb` `poams` method byte-for-byte:
///
///   required_fields → optional_fields → conditional_fields → milestones:[{...}]
///
/// Field insertion order, value types (Integer for `:numeric`, Boolean for
/// `:boolean`, everything else a String) and the always-emitted single-element
/// `milestones` array match the reference exactly. A milestone's
/// `scheduledCompletionDate` is coerced to a Float via `.to_f` (reference), e.g.
/// `1700000001` → `1700000001.0`.
fn poams_body(
    opts: &HashMap<String, String>,
    include_milestones: bool,
) -> Result<serde_json::Value> {
    use serde_json::{Map, Value};

    // :numeric → Integer if integral, else Float. Matches the reference, which emits
    // Integer for `1700000000` and Float for `100.00` (the client serializes
    // the reference value's actual numeric type).
    fn parse_numeric(s: &str) -> Result<Value, anyhow::Error> {
        if let Ok(i) = s.parse::<i64>() {
            Ok(Value::from(i))
        } else {
            let f: f64 = s
                .parse()
                .map_err(|_| anyhow::anyhow!("invalid number: {s}"))?;
            Ok(Value::from(f))
        }
    }

    let mut body: Map<String, Value> = Map::new();

    // ---- required_fields (reference attr order) ----
    if let Some(v) = opts.get("status") {
        body.insert("status".to_string(), serde_json::json!(parse_status(v)));
    }
    if let Some(v) = opts.get("vulnerabilityDescription") {
        body.insert(
            "vulnerabilityDescription".to_string(),
            Value::from(v.clone()),
        );
    }
    if let Some(v) = opts.get("sourceIdentifyingVulnerability") {
        body.insert(
            "sourceIdentifyingVulnerability".to_string(),
            Value::from(v.clone()),
        );
    }
    if let Some(v) = opts.get("pocOrganization") {
        body.insert("pocOrganization".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("resources") {
        body.insert("resources".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("identifiedInCfoAuditOrOtherReview") {
        body.insert(
            "identifiedInCFOAuditOrOtherReview".to_string(),
            Value::from(parse_bool(v)),
        );
    }

    // ---- optional_fields (reference attr order; all String except noted) ----
    if let Some(v) = opts.get("externalUid") {
        body.insert("externalUid".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("controlAcronym") {
        body.insert("controlAcronym".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("assessmentProcedure") {
        body.insert("assessmentProcedure".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("securityChecks") {
        body.insert("securityChecks".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("rawSeverity") {
        body.insert("rawSeverity".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("relevanceOfThreat") {
        body.insert("relevanceOfThreat".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("likelihood") {
        body.insert("likelihood".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("impact") {
        body.insert("impact".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("impactDescription") {
        body.insert("impactDescription".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("residualRiskLevel") {
        body.insert("residualRiskLevel".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("recommendations") {
        body.insert("recommendations".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("mitigations") {
        body.insert("mitigations".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("resultingResidualRiskLevelAfterProposedMitigations") {
        body.insert(
            "resultingResidualRiskLevelAfterProposedMitigations".to_string(),
            Value::from(v.clone()),
        );
    }
    if let Some(v) = opts.get("predisposingConditions") {
        body.insert("predisposingConditions".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("threatDescription") {
        body.insert("threatDescription".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("devicesAffected") {
        body.insert("devicesAffected".to_string(), Value::from(v.clone()));
    }

    // ---- conditional_fields (reference attr order) ----
    if let Some(v) = opts.get("pocFirstName") {
        body.insert("pocFirstName".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("pocLastName") {
        body.insert("pocLastName".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("pocEmail") {
        body.insert("pocEmail".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("pocPhoneNumber") {
        body.insert("pocPhoneNumber".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("severity") {
        body.insert("severity".to_string(), Value::from(v.clone()));
    }
    // :numeric → Integer
    if let Some(v) = opts.get("scheduledCompletionDate") {
        body.insert("scheduledCompletionDate".to_string(), parse_numeric(v)?);
    }
    if let Some(v) = opts.get("completionDate") {
        body.insert("completionDate".to_string(), parse_numeric(v)?);
    }
    if let Some(v) = opts.get("comments") {
        body.insert("comments".to_string(), Value::from(v.clone()));
    }
    if let Some(v) = opts.get("personnelResourcesFundedBaseHours") {
        body.insert(
            "personnelResourcesFundedBaseHours".to_string(),
            parse_numeric(v)?,
        );
    }
    if let Some(v) = opts.get("personnelResourcesCostCode") {
        body.insert(
            "personnelResourcesCostCode".to_string(),
            Value::from(v.clone()),
        );
    }
    if let Some(v) = opts.get("personnelResourcesUnfundedBaseHours") {
        body.insert(
            "personnelResourcesUnfundedBaseHours".to_string(),
            parse_numeric(v)?,
        );
    }
    if let Some(v) = opts.get("personnelResourcesNonfundingObstacle") {
        body.insert(
            "personnelResourcesNonfundingObstacle".to_string(),
            Value::from(v.clone()),
        );
    }
    if let Some(v) = opts.get("personnelResourcesNonfundingObstacleOtherReason") {
        body.insert(
            "personnelResourcesNonfundingObstacleOtherReason".to_string(),
            Value::from(v.clone()),
        );
    }
    if let Some(v) = opts.get("nonPersonnelResourcesFundedAmount") {
        body.insert(
            "nonPersonnelResourcesFundedAmount".to_string(),
            parse_numeric(v)?,
        );
    }
    if let Some(v) = opts.get("nonPersonnelResourcesCostCode") {
        body.insert(
            "nonPersonnelResourcesCostCode".to_string(),
            Value::from(v.clone()),
        );
    }
    if let Some(v) = opts.get("nonPersonnelResourcesUnfundedAmount") {
        body.insert(
            "nonPersonnelResourcesUnfundedAmount".to_string(),
            parse_numeric(v)?,
        );
    }
    if let Some(v) = opts.get("nonPersonnelResourcesNonfundingObstacle") {
        body.insert(
            "nonPersonnelResourcesNonfundingObstacle".to_string(),
            Value::from(v.clone()),
        );
    }
    if let Some(v) = opts.get("nonPersonnelResourcesNonfundingObstacleOtherReason") {
        body.insert(
            "nonPersonnelResourcesNonfundingObstacleOtherReason".to_string(),
            Value::from(v.clone()),
        );
    }

    // ---- milestones (always emitted as a single-element array) ----
    if include_milestones {
        let mut milestone: Map<String, Value> = Map::new();
        // --milestone JSON object (single token); reference emits the first entry.
        if let Some(ml) = opts.get("milestone") {
            let list: Vec<Value> = parse_json_list(ml)?;
            if let Some(base) = list.first() {
                if let Some(d) = base.get("description") {
                    milestone.insert("description".to_string(), d.clone());
                }
                if let Some(sc) = base.get("scheduledCompletionDate") {
                    let f = sc
                        .as_f64()
                        .or_else(|| sc.as_i64().map(|i| i as f64))
                        .ok_or_else(|| {
                            anyhow::anyhow!("milestone scheduledCompletionDate must be a number")
                        })?;
                    milestone.insert("scheduledCompletionDate".to_string(), Value::from(f));
                }
            }
        }
        body.insert(
            "milestones".to_string(),
            Value::Array(vec![Value::Object(milestone)]),
        );
    }

    Ok(Value::Object(body))
}

async fn cmd_post_poams_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    // Build the full ordered body (required→optional→conditional→milestones)
    // exactly as the reference `poams` does, then send it as a raw JSON array.
    let poam = vec![poams_body(opts, true)?];
    match apis::poam_api::add_poam_by_system_id(&config, sys_id, poam).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

async fn cmd_post_poams_add_with_milestones(opts: &HashMap<String, String>) -> Result<()> {
    // Milestones are now part of the raw create body (see poams_body), so this
    // variant is identical to `cmd_post_poams_add`: a single POA&M record is
    // created with its milestone already embedded.
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let poam = vec![poams_body(opts, true)?];
    let resp = apis::poam_api::add_poam_by_system_id(&config, sys_id, poam).await?;
    print_output(&resp);
    Ok(())
}

async fn cmd_post_poams_add_with_risk_analysis(opts: &HashMap<String, String>) -> Result<()> {
    // With the raw ordered body (poams_body) the analysis fields (rawSeverity /
    // severity, likelihood, impact, ...) are now emitted directly, so this
    // variant no longer needs the client-side limitation workaround.
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let poam = vec![poams_body(opts, true)?];
    match apis::poam_api::add_poam_by_system_id(&config, sys_id, poam).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// ---- test_results --------------------------------------------------------
fn test_results_body(opts: &HashMap<String, String>) -> serde_json::Value {
    let mut b = serde_json::json!({});
    if let Some(v) = opts.get("testId") {
        b["testId"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("testType") {
        b["testType"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("testDate") {
        b["testDate"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("source") {
        b["source"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("test") {
        b["test"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("result") {
        b["result"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("comment") {
        b["comment"] = serde_json::Value::from(v.clone());
    }
    b
}

async fn cmd_post_test_results_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let mut body = test_results_body(opts);
    if body.get("testType").is_none() {
        body["testType"] = serde_json::Value::from("CSP");
    }
    if body.get("testDate").is_none() {
        body["testDate"] = serde_json::Value::from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
    }
    match apis::test_results_api::add_test_results_by_system_id(&config, sys_id, vec![body]).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

async fn cmd_post_test_results_update(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let body = test_results_body(opts);
    match apis::test_results_api::add_test_results_by_system_id(&config, sys_id, vec![body]).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// ---- milestones ----------------------------------------------------------
async fn cmd_post_milestones_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let poam_id = require_int(opts, "poamId")?;
    let v = opts
        .get("description")
        .or_else(|| opts.get("milestones"))
        .ok_or_else(|| anyhow!("`milestones add` requires --description <...>"))?;
    let mls = parse_json_list(v)?;
    for m in mls {
        match apis::milestones_api::add_milestone_by_system_id_and_poam_id(
            &config,
            sys_id,
            poam_id,
            vec![m.clone()],
        )
        .await
        {
            Ok(r) => print_output(&r),
            Err(e) => print_err(&e),
        }
    }
    Ok(())
}

// ---- cac / pac -----------------------------------------------------------
fn cac_body(opts: &HashMap<String, String>) -> serde_json::Value {
    let mut b = serde_json::json!({});
    if let Some(v) = opts.get("controlAcronym") {
        b["controlAcronym"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("complianceStatus") {
        b["complianceStatus"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("currentStageName") {
        b["currentStageName"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("currentStage") {
        b["currentStage"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("totalStages") {
        b["totalStages"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("comment") {
        b["comment"] = serde_json::Value::from(v.clone());
    }
    b
}

async fn cmd_post_cac_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    match apis::cac_api::add_system_cac(&config, sys_id, vec![cac_body(opts)]).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

fn pac_body(opts: &HashMap<String, String>) -> serde_json::Value {
    let mut b = serde_json::json!({});
    if let Some(v) = opts.get("workflow") {
        b["workflow"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("name") {
        b["name"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("currentStageName") {
        b["currentStageName"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("currentStage") {
        b["currentStage"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("totalStages") {
        b["totalStages"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("daysAtCurrentStage") {
        b["daysAtCurrentStage"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("comment") {
        b["comment"] = serde_json::Value::from(v.clone());
    }
    b
}

async fn cmd_post_pac_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    match apis::pac_api::add_system_pac(&config, sys_id, vec![pac_body(opts)]).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// ---- artifacts (direct-param upload) -------------------------------------
async fn cmd_post_artifacts_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let filename: PathBuf = opts
        .get("filename")
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("`artifacts add` requires --filename <path>"))?;
    let is_bulk = opts.get("isBulk").map(|v| parse_bool(v));
    let is_template = opts.get("isTemplate").map(|v| parse_bool(v));
    let r#type = opts.get("type").map(String::as_str);
    let category = opts.get("category").map(String::as_str);
    match apis::artifacts_api::add_artifacts_by_system_id(
        &config,
        sys_id,
        filename,
        is_bulk,
        is_template,
        r#type,
        category,
    )
    .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

async fn cmd_post_artifacts_update(opts: &HashMap<String, String>) -> Result<()> {
    // artifacts update reuses the same add-artifacts endpoint (idempotent upsert)
    cmd_post_artifacts_add(opts).await
}

// ---- hardware / software baselines ---------------------------------------
async fn cmd_post_hardware_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let mut hw = HwBaselineRequiredFields::new();
    if let Some(v) = opts.get("name") {
        hw.asset_name = Some(v.clone());
    }
    match apis::hardware_baseline_api::add_hw_baseline_assets(&config, sys_id, vec![hw]).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

async fn cmd_post_software_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let mut sw = SwBaselineRequiredFields::new();
    if let Some(v) = opts.get("name") {
        sw.software_name = Some(v.clone());
    }
    if let Some(v) = opts.get("vendor") {
        sw.software_vendor = Some(v.clone());
    }
    if let Some(v) = opts.get("version") {
        sw.version = Some(v.clone());
    }
    match apis::software_baseline_api::add_sw_baseline_assets(&config, sys_id, vec![sw]).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// ---- cloud ---------------------------------------------------------------
// Build the single-cloud-object body from CLI options, mirroring emass
// post.rb (`cloud add`) EXACTLY (lib/emass_cli.rb, the `cloud add` block):
//   * provider/resourceId/resourceName/resourceType are ALWAYS set (reference sets
//     them unconditionally, yielding `null` only if a required flag is omitted);
//   * initiatedBy / cspAccountId / cspRegion are set only when provided;
//   * isBaseline is set only when truthy (reference: `body[:isBaseline] = ... if ...`);
//   * `tags` is ALWAYS present as an object (with `test` only when --test given);
//   * complianceResults is ALWAYS a 1-element array whose single object carries
//     cspPolicyDefinitionId / isCompliant / policyDefinitionTitle unconditionally,
//     plus assessmentProcedure / complianceCheckTimestamp / complianceReason /
//     control / policyDeploymentName / policyDeploymentVersion / severity only when
//     provided.
fn cloud_body(opts: &HashMap<String, String>) -> serde_json::Value {
    let opt = |k: &str| -> serde_json::Value {
        match opts.get(k) {
            Some(v) => serde_json::Value::from(v.clone()),
            None => serde_json::Value::Null,
        }
    };
    let mut b = serde_json::json!({
        "provider": opt("provider"),
        "resourceId": opt("resourceId"),
        "resourceName": opt("resourceName"),
        "resourceType": opt("resourceType"),
    });
    if let Some(v) = opts.get("initiatedBy") {
        b["initiatedBy"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("cspAccountId") {
        b["cspAccountId"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("cspRegion") {
        b["cspRegion"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("isBaseline") {
        if parse_bool(v) {
            b["isBaseline"] = serde_json::Value::Bool(true);
        }
    }
    // tags: always an object; { test } only when --test is present.
    let mut tags = serde_json::Map::new();
    if let Some(v) = opts.get("test") {
        tags.insert("test".into(), serde_json::Value::from(v.clone()));
    }
    b["tags"] = serde_json::Value::Object(tags);

    // complianceResults: 1-element array; 3 keys always set, the rest optional.
    let mut cr = serde_json::json!({
        "cspPolicyDefinitionId": opt("cspPolicyDefinitionId"),
        "isCompliant": serde_json::Value::Bool(
            opts.get("isCompliant").map(|x| parse_bool(x)).unwrap_or(false),
        ),
        "policyDefinitionTitle": opt("policyDefinitionTitle"),
    });
    if let Some(v) = opts.get("assessmentProcedure") {
        cr["assessmentProcedure"] = serde_json::Value::from(v.clone());
    }
    if let Some(n) = opt_i64(opts, "complianceCheckTimestamp") {
        cr["complianceCheckTimestamp"] = serde_json::Value::from(n);
    }
    if let Some(v) = opts.get("complianceReason") {
        cr["complianceReason"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("control") {
        cr["control"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("policyDeploymentName") {
        cr["policyDeploymentName"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("policyDeploymentVersion") {
        cr["policyDeploymentVersion"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("severity") {
        cr["severity"] = serde_json::Value::from(v.clone());
    }
    b["complianceResults"] = serde_json::Value::Array(vec![serde_json::Value::Object(
        cr.as_object().unwrap().clone(),
    )]);
    b
}

// ---- container (upload containers and their scan results) ----------------
// Mirrors emasser's `Container#add`: assemble the nested benchmarks/results
// body and POST it via add_container_sans_by_system_id (sibling of `cloud`).
fn container_body(opts: &HashMap<String, String>) -> Result<serde_json::Value> {
    // Required scalar fields (emasser marks these `required: true`).
    let container_id = opts
        .get("containerId")
        .context("`container add` requires --containerId")?;
    let container_name = opts
        .get("containerName")
        .context("`container add` requires --containerName")?;
    let time = opt_i64(opts, "time").context("`container add` requires --time <unix> (integer)")?;
    let benchmark = opts
        .get("benchmark")
        .context("`container add` requires --benchmark")?;
    let last_seen = opt_i64(opts, "lastSeen")
        .context("`container add` requires --lastSeen <unix> (integer)")?;
    let rule_id = opts
        .get("ruleId")
        .context("`container add` requires --ruleId")?;
    let status = opts
        .get("status")
        .context("`container add` requires --status <enum>")?;

    let mut body = serde_json::json!({});
    body["containerId"] = serde_json::Value::from(container_id.clone());
    body["containerName"] = serde_json::Value::from(container_name.clone());
    body["time"] = serde_json::Value::from(time);
    if let Some(v) = opts.get("namespace") {
        body["namespace"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("podIp") {
        body["podIp"] = serde_json::Value::from(v.clone());
    }
    if let Some(v) = opts.get("podName") {
        body["podName"] = serde_json::Value::from(v.clone());
    }

    // Tags - optional object.
    if let Some(test) = opts.get("test") {
        body["tags"] = serde_json::json!({"test": test.clone()});
    }

    // Benchmarks - required object.
    let mut benchmarks =
        serde_json::json!({"benchmark": serde_json::Value::from(benchmark.clone())});
    // isBaseline defaults to true in emasser (option default), so always emit it.
    benchmarks["isBaseline"] = serde_json::Value::from(parse_bool(
        opts.get("isBaseline").map_or("true", |v| v.as_str()),
    ));
    if let Some(v) = opt_i64(opts, "version") {
        benchmarks["version"] = serde_json::Value::from(v);
    }
    if let Some(v) = opt_i64(opts, "release") {
        benchmarks["release"] = serde_json::Value::from(v);
    }

    // Benchmarks.Results - required.
    let mut br = serde_json::json!({});
    br["lastSeen"] = serde_json::Value::from(last_seen);
    br["ruleId"] = serde_json::Value::from(rule_id.clone());
    br["status"] = serde_json::Value::from(status.clone());
    if let Some(v) = opts.get("message") {
        br["message"] = serde_json::Value::from(v.clone());
    }
    benchmarks["results"] = serde_json::json!([br]);

    body["benchmarks"] = serde_json::json!([benchmarks]);
    Ok(body)
}

async fn cmd_post_container_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let body = container_body(opts)?;
    match apis::container_scan_results_api::add_container_sans_by_system_id(
        &config,
        sys_id,
        vec![body],
    )
    .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

async fn cmd_post_register_cert(_opts: &HashMap<String, String>) -> Result<()> {
    // `post register cert` registers a certificate & returns an API key.
    // emasser calls RegistrationApi#register_user with no request params, so
    // the POC client call also takes only `&config`.
    let config = build_config()?;
    match apis::registration_api::register_user(&config).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// `post device_scans upload` mirrors emasser's DeviceScans#upload: required
// systemId/filename/scanType, optional isBaseline (default false), plus
// emasser's file-extension guard (disaStigViewerCklCklb -> .ckl/.cklb,
// acasAsrArf/policyAuditor -> .zip).
async fn cmd_post_device_scans_upload(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let scan_type = opts
        .get("scanType")
        .with_context(|| "`device_scans upload` requires --scanType")?;
    let filename = opts
        .get("filename")
        .with_context(|| "`device_scans upload` requires --filename")?;
    if scan_type == "disaStigViewerCklCklb" && !filename.contains(".ckl") {
        bail!("If the scan type is \"disaStigViewerCklCklb\" a .ckl or .cklb file is expected");
    }
    if (scan_type == "acasAsrArf" || scan_type == "policyAuditor") && !filename.contains(".zip") {
        bail!("If the scan type is \"acasAsrArf\" or \"policyAuditor\" a .zip file is expected");
    }
    let is_baseline = opts.get("isBaseline").map(|v| parse_bool(v));
    match apis::device_scan_results_api::add_scan_results_by_system_id(
        &config,
        sys_id,
        scan_type.as_str(),
        std::path::PathBuf::from(filename.clone()),
        is_baseline,
    )
    .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

async fn cmd_post_cloud_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    match apis::cloud_resource_results_api::add_cloud_resources_by_system_id(
        &config,
        sys_id,
        vec![cloud_body(opts)],
    )
    .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// ---- scan (device scans + static code) -----------------------------------
async fn cmd_post_scan_add(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let scan_type = opts
        .get("type")
        .map(String::as_str)
        .unwrap_or("NIST_SP_800_53");
    let filename: PathBuf = opts
        .get("filename")
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("`scan add` requires --filename <path>"))?;
    let is_baseline = opts.get("isBaseline").map(|v| parse_bool(v));
    match apis::device_scan_results_api::add_scan_results_by_system_id(
        &config,
        sys_id,
        scan_type,
        filename,
        is_baseline,
    )
    .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

/// `post scan clear` — clear an application's static-code findings.
/// Mirrors `lib/emasse/lib/emasse/post.rb` `Scans#clear`: the
/// `--clearFindings` flag is required and truthy, and the body carries
/// only `application` (name + version) plus one finding with
/// `clearFindings=true`.
async fn cmd_post_scan_clear(opts: &HashMap<String, String>) -> Result<()> {
    // Validate required fields *before* loading config / connecting, so a
    // clear validation error is returned rather than a config/network error
    // (mirrors emasser `Scans#clear`, which guards before the API call).
    let clear_findings = opts
        .get("clearFindings")
        .map(|v| parse_bool(v))
        .ok_or_else(|| anyhow!("`post scan clear` requires --clearFindings <true>"))?;
    if !clear_findings {
        bail!("To clear an application's findings set --clearFindings true");
    }
    let name = opts
        .get("name")
        .cloned()
        .ok_or_else(|| anyhow!("`post scan clear` requires --name <application name>"))?;
    let sys_id = require_int(opts, "systemId")?;
    // `version` is required by the eMASS spec + reference tool: a clear's
    // application needs both `applicationName` and `version`.
    let version = opts
        .get("version")
        .cloned()
        .ok_or_else(|| anyhow!("`post scan clear` requires --version <app version>"))?;
    let config = build_config()?;
    let mut app = StaticCodeRequestPostBodyApplication::new();
    app.application_name = Some(name);
    app.version = Some(version);
    let mut finding = StaticCodeApplicationPost::new();
    finding.clear_findings = Some(clear_findings);
    let mut body = StaticCodeRequestPostBody::new();
    body.application = Some(Box::new(app));
    body.application_findings = Some(vec![finding]);
    // eMASS requires the body as a single-element JSON array `[ {...} ]`
    // (matches emasser `post.rb` `Array.new(1, body)`). The client serializes a
    // Vec directly, so pass `vec![body]` for byte parity with the reference.
    match apis::static_code_scans_api::add_static_code_scans_by_system_id(
        &config,
        sys_id,
        vec![body],
    )
    .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

async fn cmd_post_scan_static_code(opts: &HashMap<String, String>) -> Result<()> {
    let config = build_config()?;
    let sys_id = require_int(opts, "systemId")?;
    let mut app = StaticCodeRequestPostBodyApplication::new();
    app.application_name = Some(
        opts.get("name")
            .cloned()
            .unwrap_or_else(|| "static_code".to_string()),
    );
    app.version = opts.get("version").cloned();
    let mut body = StaticCodeRequestPostBody::new();
    body.application = Some(Box::new(app));
    if let Some(f) = opts.get("findings") {
        let mut findings = Vec::new();
        for j in parse_json_list(f)? {
            let mut fd = StaticCodeApplicationPost::new();
            if let Some(s) = j.get("rawSeverity") {
                fd.raw_severity = Some(parse_severity(s.as_str().unwrap_or("")));
            }
            if let Some(s) = j.get("codeCheckName") {
                fd.code_check_name = s.as_str().map(String::from);
            }
            fd.count = j.get("count").and_then(|v| v.as_i64());
            fd.scan_date = j.get("scanDate").and_then(|v| v.as_i64());
            fd.cwe_id = j.get("cweId").and_then(|v| v.as_str().map(String::from));
            fd.clear_findings = j.get("clearFindings").and_then(|v| v.as_bool());
            findings.push(fd);
        }
        body.application_findings = Some(findings);
    }
    match apis::static_code_scans_api::add_static_code_scans_by_system_id(
        &config,
        sys_id,
        vec![body],
    )
    .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// ========================================================================
//  `put` / `delete` groups — ported below (stubs kept to unblock `post`)
// ========================================================================
// ========================================================================
//  `put` / `delete` groups — `put` ported below; `delete` stub kept
// ========================================================================
// ========================================================================
//  `put` group  (mirrors lib/emasser/lib/emasser/put.rb)
//
//  Each PUT arm mirrors emasser's "typed struct -> merge -> client call".
//  The generated client's PUT endpoints accept *stripped* typed models
//  (update_poam_by_system_id -> Vec<PoamIds>; update_hw_baseline_assets ->
//  Vec<HwBaselineReadOnlyFields>; etc.), so each arm sets only the fields
//  the client struct exposes. The milestones endpoint accepts raw JSON, so
//  it is fully populated.
// ========================================================================

/// Optional 64-bit int from opts (used by the i64-typed PoamIds model).
fn opt_i64(opts: &HashMap<String, String>, key: &str) -> Option<i64> {
    opts.get(key).and_then(|s| s.parse::<i64>().ok())
}

async fn dispatch_put(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("usage: emasser put <controls|poams|milestones|artifacts|hardware|software> update --systemId <id> ...");
    }
    // Lower-case the positional sub-commands; leave flags verbatim.
    let lower: Vec<String> = args
        .iter()
        .take_while(|s| !s.starts_with("--"))
        .map(|s| s.to_lowercase())
        .collect();
    let sub: Vec<&str> = lower.iter().map(String::as_str).collect();
    let flags_start = args
        .iter()
        .position(|s| s.starts_with("--"))
        .unwrap_or(args.len());
    let opts = parse_options(&args[flags_start..]);
    let cmd = sub.first().copied().unwrap_or("");
    let sys_id = require_int(&opts, "systemId")?;
    match cmd {
        "controls" => cmd_put_controls(sys_id, &opts).await,
        "poams" => cmd_put_poams(sys_id, &opts).await,
        "milestones" => cmd_put_milestones(sys_id, &opts).await,
        "artifacts" => cmd_put_artifacts(sys_id, &opts).await,
        "hardware" => cmd_put_hardware(sys_id, &opts).await,
        "software" => cmd_put_software(sys_id, &opts).await,
        _ => bail!("usage: emasser put <controls|poams|milestones|artifacts|hardware|software> update --systemId <id> ..."),
    }
}

// -- controls ---------------------------------------------------------------

// Mirror emasser's `process_business_logic`: enforce conditional security
// control update rules before the call (emasser `exit` == our `bail`).
fn process_business_logic_controls(opts: &HashMap<String, String>) -> Result<()> {
    let status = opts
        .get("implementationStatus")
        .map(String::as_str)
        .unwrap_or("");
    if status.is_empty() {
        bail!("The \"--implementationStatus\" parameter is required when updating a Security Control.");
    }
    let req = |k: &str| opts.get(k).map(String::as_str).is_some();
    match status {
        "Planned" | "Implemented" => {
            if !(req("responsibleEntities") && req("slcmCriticality") && req("slcmFrequency") && req("slcmMethod") && req("slcmReporting") && req("slcmTracking") && req("slcmComments")) {
                bail!("Missing one of: responsibleEntities, slcmCriticality, slcmFrequency, slcmMethod, slcmReporting, slcmTracking, slcmComments");
            }
        }
        "Not Applicable" => {
            if !(req("naJustification") && req("responsibleEntities")) {
                bail!("Missing one of: naJustification, responsibleEntities");
            }
        }
        "Manually Inherited" => {
            if !(req("commonControlProvider") && req("responsibleEntities") && req("slcmCriticality") && req("slcmFrequency") && req("slcmMethod") && req("slcmReporting") && req("slcmTracking") && req("slcmComments")) {
                bail!("Missing one of: commonControlProvider, responsibleEntities, slcmCriticality, slcmFrequency, slcmMethod, slcmReporting, slcmTracking, slcmComments");
            }
        }
        "Inherited" => {
            if !req("commonControlProvider") {
                bail!("When implementationStatus value is \"Inherited\" only the following fields are updated: controlDesignation and commonControlProvider. Missing the commonControlProvider field");
            }
        }
        _ => bail!("implementationStatus must be one of: Planned, Implemented, Inherited, Not Applicable, Manually Inherited"),
    }
    Ok(())
}

async fn cmd_put_controls(sys_id: i32, opts: &HashMap<String, String>) -> Result<()> {
    // emasser runs process_business_logic before building the body.
    process_business_logic_controls(opts)?;
    let mut c = ControlsRequiredFields::new();
    if let Some(v) = opts.get("acronym") {
        c.acronym = Some(v.clone());
    }
    if let Some(v) = opts.get("responsibleEntities") {
        c.responsible_entities = Some(v.clone());
    }
    if let Some(v) = opts.get("controlDesignation") {
        c.control_designation = Some(match v.to_lowercase().as_str() {
            "common" => emass_client::models::controls_required_fields::ControlDesignation::Common,
            "system-specific" => {
                emass_client::models::controls_required_fields::ControlDesignation::SystemSpecific
            }
            "hybrid" => emass_client::models::controls_required_fields::ControlDesignation::Hybrid,
            _ => emass_client::models::controls_required_fields::ControlDesignation::Common,
        });
    }
    if let Some(v) = opts.get("estimatedCompletionDate") {
        c.estimated_completion_date = v.parse::<i64>().ok();
    }
    if let Some(v) = opts.get("implementationNarrative") {
        c.implementation_narrative = Some(v.clone());
    }
    match apis::controls_api::update_control_by_system_id(&build_config()?, sys_id, vec![c]).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// -- poams ------------------------------------------------------------------

async fn cmd_put_poams(sys_id: i32, opts: &HashMap<String, String>) -> Result<()> {
    // client PoamIds carries only: system_id, poam_id, display_poam_id
    let mut p = PoamIds::new();
    p.system_id = Some(sys_id as i64);
    if let Some(v) = opt_i64(opts, "poamId") {
        p.poam_id = Some(v);
    }
    if let Some(v) = opt_i64(opts, "displayPoamId") {
        p.display_poam_id = Some(v);
    }
    match apis::poam_api::update_poam_by_system_id(&build_config()?, sys_id, vec![p]).await {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// -- milestones (raw JSON body) --------------------------------------------

async fn cmd_put_milestones(sys_id: i32, opts: &HashMap<String, String>) -> Result<()> {
    // emasser's Milestones#update requires BOTH --poamId (path param) and
    // --milestoneId (body field):
    //   /api/systems/{systemId}/poams/{poamId}/milestones  -> poamId in path,
    //   milestoneId/description/scheduledCompletionDate in the body.
    let poam_id: i32 = match opts.get("poamId").and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => bail!("--poamId is required to update a milestone"),
    };
    // emasser milestones endpoint accepts raw JSON: build the full body.
    let mut body: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    if let Some(v) = opts.get("milestoneId") {
        body.insert("milestoneId".into(), serde_json::json!(v));
    }
    if let Some(v) = opts.get("description") {
        body.insert("description".into(), serde_json::json!(v));
    }
    if let Some(v) = opts.get("scheduledCompletionDate") {
        body.insert("scheduledCompletionDate".into(), serde_json::json!(v));
    }
    if let Some(v) = opts.get("reviewStatus") {
        body.insert("reviewStatus".into(), serde_json::json!(v));
    }
    if let Some(v) = opts.get("createdDate") {
        body.insert("createdDate".into(), serde_json::json!(v));
    }
    if let Some(v) = opts.get("createdBy") {
        body.insert("createdBy".into(), serde_json::json!(v));
    }
    let body_array = vec![serde_json::Value::Object(body)];
    match apis::milestones_api::update_milestone_by_system_id_and_poam_id(
        &build_config()?,
        sys_id,
        poam_id,
        body_array,
    )
    .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// -- artifacts --------------------------------------------------------------

async fn cmd_put_artifacts(sys_id: i32, opts: &HashMap<String, String>) -> Result<()> {
    // client ArtifactsRequiredFields carries: filename, is_template, type, category
    let mut a = ArtifactsRequiredFields::new();
    if let Some(v) = opts.get("filename") {
        a.filename = Some(v.clone());
    }
    if let Some(v) = opts.get("isTemplate") {
        a.is_template = Some(Some(v == "true"));
    }
    if let Some(v) = opts.get("type") {
        a.r#type = Some(v.clone());
    }
    if let Some(v) = opts.get("category") {
        a.category = Some(v.clone());
    }
    match apis::artifacts_api::update_artifact_by_system_id(&build_config()?, sys_id, vec![a]).await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// -- hardware ---------------------------------------------------------------

async fn cmd_put_hardware(sys_id: i32, opts: &HashMap<String, String>) -> Result<()> {
    // client HwBaselineReadOnlyFields carries only: hardware_id
    let mut h = HwBaselineReadOnlyFields::new();
    if let Some(v) = opts.get("hardwareId") {
        h.hardware_id = Some(Some(v.clone()));
    }
    match apis::hardware_baseline_api::update_hw_baseline_assets(&build_config()?, sys_id, vec![h])
        .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}

// -- software ---------------------------------------------------------------

async fn cmd_put_software(sys_id: i32, opts: &HashMap<String, String>) -> Result<()> {
    // client SwBaselineReadOnlyFields carries only: software_id
    let mut s = SwBaselineReadOnlyFields::new();
    if let Some(v) = opts.get("softwareId") {
        s.software_id = Some(Some(v.clone()));
    }
    match apis::software_baseline_api::update_sw_baseline_assets(&build_config()?, sys_id, vec![s])
        .await
    {
        Ok(resp) => print_output(&resp),
        Err(err) => print_err(&err),
    }
    Ok(())
}
// ========================================================================
//  `delete` group  (mirrors lib/emasser/lib/emasser/delete.rb)
//  Each arm builds the client's typed DELETE body struct(s) from the CLI
//  options and forwards them to the matching client delete_* function.
// ========================================================================

// Collect the space-separated values following `--<key>` from raw args
// (mirrors emasser `type: :array` options like `--files` / `--hardwareIds`).
fn array_args(args: &[String], key: &str) -> Vec<String> {
    let flag = format!("--{}", key);
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            i += 1;
            while i < args.len() && !args[i].starts_with("--") {
                out.push(args[i].clone());
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    out
}

async fn dispatch_delete(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("usage: emasser delete <poams|milestones|artifacts|hardware|software|cloud_resource|container> remove [options]");
    }
    let lower: Vec<String> = args
        .iter()
        .take_while(|s| !s.starts_with("--"))
        .map(|s| s.to_lowercase())
        .collect();
    let sub: Vec<&str> = lower.iter().map(String::as_str).collect();
    let flags_start = args
        .iter()
        .position(|s| s.starts_with("--"))
        .unwrap_or(args.len());
    let opts = parse_options(&args[flags_start..]);
    match sub.as_slice() {
        ["poams", "remove"] => {
            let sys_id = require_int(&opts, "systemId")?;
            let poam_id: i64 = match opts.get("poamId") {
                Some(v) => v.parse::<i64>().with_context(|| "`--poamId` must be an integer")?,
                None => bail!("--poamId is required"),
            };
            let mut b = PoamRequestDeleteBodyInner::new();
            b.poam_id = Some(poam_id);
            match apis::poam_api::delete_poam(&build_config()?, sys_id, vec![b]).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        ["milestones", "remove"] => {
            let sys_id = require_int(&opts, "systemId")?;
            let poam_id: i32 = match opts.get("poamId") {
                Some(v) => v.parse::<i32>().with_context(|| "`--poamId` must be an integer")?,
                None => bail!("--poamId is required"),
            };
            let milestone_id: i64 = match opts.get("milestoneId") {
                Some(v) => v.parse::<i64>().with_context(|| "`--milestoneId` must be an integer")?,
                None => bail!("--milestoneId is required"),
            };
            let mut b = MilestonesRequestDeleteBodyInner::new();
            b.milestone_id = Some(milestone_id);
            match apis::milestones_api::delete_milestone(&build_config()?, sys_id, poam_id, vec![b]).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        ["artifacts", "remove"] => {
            let sys_id = require_int(&opts, "systemId")?;
            let files = array_args(args, "files");
            if files.is_empty() {
                bail!("`--files` is required (one or more artifact filenames)");
            }
            let v: Vec<ArtifactsRequestDeleteBodyInner> = files
                .iter()
                .map(|f| {
                    let mut b = ArtifactsRequestDeleteBodyInner::new();
                    b.filename = Some(f.clone());
                    b
                })
                .collect();
            match apis::artifacts_api::delete_artifact(&build_config()?, sys_id, v).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        ["hardware", "remove"] => {
            let sys_id = require_int(&opts, "systemId")?;
            let ids = array_args(args, "hardwareIds");
            if ids.is_empty() {
                bail!("`--hardwareIds` is required (one or more hardware asset GUIDs)");
            }
            let v: Vec<HwBaselineRequestDeleteBodyInner> = ids
                .iter()
                .map(|id| {
                    let mut b = HwBaselineRequestDeleteBodyInner::new();
                    b.hardware_id = Some(id.clone());
                    b
                })
                .collect();
            match apis::hardware_baseline_api::delete_hw_baseline_assets(&build_config()?, sys_id, v).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        ["software", "remove"] => {
            let sys_id = require_int(&opts, "systemId")?;
            let ids = array_args(args, "softwareIds");
            if ids.is_empty() {
                bail!("`--softwareIds` is required (one or more software asset GUIDs)");
            }
            let v: Vec<SwBaselineRequestDeleteBodyInner> = ids
                .iter()
                .map(|id| {
                    let mut b = SwBaselineRequestDeleteBodyInner::new();
                    b.software_id = Some(id.clone());
                    b
                })
                .collect();
            match apis::software_baseline_api::delete_sw_baseline_assets(&build_config()?, sys_id, v).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        ["cloud_resource", "remove"] => {
            let sys_id = require_int(&opts, "systemId")?;
            let mut b = CloudResourcesDeleteBodyInner::new();
            b.resource_id = opts.get("resourceId").cloned();
            match apis::cloud_resource_results_api::delete_cloud_resources(&build_config()?, sys_id, vec![b]).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        ["container", "remove"] => {
            let sys_id = require_int(&opts, "systemId")?;
            let mut b = ContainerResourcesDeleteBodyInner::new();
            b.container_id = opts.get("containerId").cloned();
            match apis::container_scan_results_api::delete_container_sans(&build_config()?, sys_id, vec![b]).await {
                Ok(resp) => print_output(&resp),
                Err(err) => print_err(&err),
            }
        }
        _ => bail!("usage: emasser delete <poams|milestones|artifacts|hardware|software|cloud_resource|container> remove [options]"),
    }
    Ok(())
}

/// emasser's `.env` template, copied byte-for-byte from
/// lib/emasser/configure.rb `generate_env_file`.
const ENV_TEMPLATE: &str = "\
# -----------------------------------------------------------------------------
# Required environment variables for eMASSer
EMASSER_API_KEY=''
EMASSER_HOST_URL=''
EMASSER_CERT_FILE_PATH=''
EMASSER_KEY_FILE_PATH=''
EMASSER_KEY_FILE_PASSWORD=''
# Required by most eMASS instances for actionable requests (post,put,delete)
EMASSER_USER_UID=''
# -----------------------------------------------------------------------------
# Optional environment variables for eMASSer
EMASSER_CLIENT_SIDE_VALIDATION='true'
EMASSER_VERIFY_SSL='false'
EMASSER_VERIFY_SSL_HOST='true'
EMASSER_DEBUGGING='true'
EMASSER_CLI_DISPLAY_NULL='false'
EMASSER_EPOCH_TO_DATETIME='true'
EMASSER_DOWNLOAD_DIR='eMASSerDownloads'
";

/// emasser `is_numeric?` — a value is numeric iff it parses as a float
/// (Integer/Decimal/Infinity/NaN). Non-portable wrappers omitted.
fn is_numeric(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

/// emasser `update_key_value_pairs('.env', answers)`:
/// read the file, and for each line whose key is in `updates`, rewrite the
/// value — numeric values unquoted, other strings wrapped in single quotes,
/// already-quoted values left as-is — while preserving every other line
/// (comments, order, trailing newline) exactly.
fn update_key_value_pairs(file_path: &Path, updates: &HashMap<String, String>) -> Result<()> {
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Error reading or writing file: {}", file_path.display()))?;
    let mut updated_lines: Vec<String> = Vec::with_capacity(file_content.len() / 16 + 1);
    for line in file_content.split('\n') {
        let trimmed = line.trim();
        let mut parts = trimmed.splitn(2, '=');
        let key = parts.next().unwrap_or("");
        let _value_ = parts.next();
        if let Some(updates_value) = updates.get(key) {
            if is_numeric(updates_value) {
                updated_lines.push(format!("{key}={updates_value}"));
            } else if !updates_value.starts_with('\'') {
                updated_lines.push(format!("{key}='{updates_value}'"));
            } else {
                updated_lines.push(format!("{key}={updates_value}"));
            }
        } else {
            updated_lines.push(line.to_string());
        }
    }
    fs::write(file_path, updated_lines.join("\n"))?;
    println!("File ({}) updated successfully.", file_path.display());
    Ok(())
}

/// emasser `configure` portable essence: the interactive tty-prompt wizard is
/// non-portable, so this drives it non-interactively — read each of the 13
/// emasser env vars from the process environment (falling back to emasser's
/// documented defaults, mirroring each `get_*_variable` default), create the
/// `.env` template when absent, then merge the values in exactly like
/// `update_key_value_pairs`.
async fn dispatch_configure(_args: &[String]) -> Result<()> {
    // (key, emasser default when the env var is unset)
    let defaults: [(&str, &str); 13] = [
        ("EMASSER_API_KEY", ""),
        ("EMASSER_HOST_URL", ""),
        ("EMASSER_CERT_FILE_PATH", ""),
        ("EMASSER_KEY_FILE_PATH", ""),
        ("EMASSER_KEY_FILE_PASSWORD", ""),
        ("EMASSER_USER_UID", ""),
        ("EMASSER_CLIENT_SIDE_VALIDATION", "true"),
        ("EMASSER_VERIFY_SSL", "false"),
        ("EMASSER_VERIFY_SSL_HOST", "true"),
        ("EMASSER_DEBUGGING", "false"),
        ("EMASSER_CLI_DISPLAY_NULL", "false"),
        ("EMASSER_EPOCH_TO_DATETIME", "true"),
        ("EMASSER_DOWNLOAD_DIR", "eMASSerDownloads"),
    ];
    let mut answers: HashMap<String, String> = HashMap::new();
    for (key, default) in defaults {
        answers.insert(
            key.to_string(),
            env::var(key).unwrap_or_else(|_| default.to_string()),
        );
    }

    let env_path = Path::new(".env");
    if !env_path.exists() {
        fs::write(env_path, ENV_TEMPLATE)?;
        println!("(.env file created and loaded).");
    }
    update_key_value_pairs(env_path, &answers)?;
    Ok(())
}
