use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn load(path: &Utf8Path) -> Result<Config> {
    Ok(from_str(&read_to_string(path)?)?)
}

#[derive(Deserialize)]
pub struct Config {
    pub working_directory: Utf8PathBuf,
    pub results_directory: Utf8PathBuf,
    pub suites: HashMap<String, SuiteConfig>,
}

#[derive(Deserialize)]
pub struct SuiteConfig {
    pub robot_framework_config: RobotFrameworkConfig,
    pub execution_config: ExecutionConfig,
    pub environment_config: EnvironmentConfig,
    pub session_config: SessionConfig,
}

#[derive(Clone, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct RobotFrameworkConfig {
    pub robot_target: Utf8PathBuf,
    pub variable_file: Option<Utf8PathBuf>,
    pub argument_file: Option<Utf8PathBuf>,
    pub retry_strategy: RetryStrategy,
}

#[derive(Clone, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum RetryStrategy {
    Incremental,
    Complete,
}

#[derive(Clone, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ExecutionConfig {
    pub n_retries_max: usize,
    pub execution_interval_seconds: u32,
    pub timeout: u64,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum EnvironmentConfig {
    System,
    Rcc(RCCEnvironmentConfig),
}

#[derive(Deserialize)]
pub struct RCCEnvironmentConfig {
    pub binary_path: Utf8PathBuf,
    pub robot_yaml_path: Utf8PathBuf,
    pub build_timeout: u64,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SessionConfig {
    Current,
    SpecificUser(UserSessionConfig),
}

#[derive(Deserialize)]
pub struct UserSessionConfig {
    pub user_name: String,
}
