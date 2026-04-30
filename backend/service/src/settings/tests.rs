use std::{path::PathBuf, sync::Mutex};

use test_case::test_case;

use super::*;

static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvVar {
    key: String,
    value: String,
}

#[test_case(
        &[]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::default(),
            log_level: LogLevel::default(),
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: default_osv_db_path(),
            core_db_url: default_core_db_url(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "default values"
    )]
#[test_case(
        &[
            EnvVar {key: "OSV_SERVICE_BIND_ADDRESS".to_string(), value: "127.0.0.1:3030".to_string() }
        ]
        => Settings {
            bind_address: "127.0.0.1:3030".parse().unwrap(),
            log_format: LogFormat::default(),
            log_level: LogLevel::default(),
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: default_osv_db_path(),
            core_db_url: default_core_db_url(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "set bind address"
    )]
#[test_case(
        &[
            EnvVar {key: "OSV_SERVICE_LOG_FORMAT".to_string(), value: "json".to_string() }
        ]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::Json,
            log_level: LogLevel::default(),
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: default_osv_db_path(),
            core_db_url: default_core_db_url(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "set tracing format to json"
    )]
#[test_case(
        &[
            EnvVar {key: "OSV_SERVICE_LOG_LEVEL".to_string(), value: "DEBUG".to_string() }
        ]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::default(),
            log_level: LogLevel::Debug,
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: default_osv_db_path(),
            core_db_url: default_core_db_url(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "set log level to debug"
    )]
#[test_case(
        &[
            EnvVar {key: "OSV_SERVICE_API_URL_PREFIX".to_string(), value: "/v1".to_string() }
        ]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::default(),
            log_level: LogLevel::default(),
            api_url_prefix: "/v1".to_string(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: default_osv_db_path(),
            core_db_url: default_core_db_url(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "set api url prefix"
    )]
#[test_case(
        &[
            EnvVar {key: "OSV_SERVICE_MANIFEST_DB_PATH".to_string(), value: "/var/data/manifests".to_string() }
        ]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::default(),
            log_level: LogLevel::default(),
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: PathBuf::from("/var/data/manifests"),
            osv_db_path: default_osv_db_path(),
            core_db_url: default_core_db_url(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "set manifest db path"
    )]
#[test_case(
        &[
            EnvVar {key: "OSV_SERVICE_OSV_DB_PATH".to_string(), value: "/var/data/osv".to_string() },
        ]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::default(),
            log_level: LogLevel::default(),
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: PathBuf::from("/var/data/osv"),
            core_db_url: default_core_db_url(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "set osv db path"
    )]
#[test_case(
        &[
            EnvVar {
                key: "OSV_SERVICE_CORE_DB_URL".to_string(),
                value: "sqlite:///var/data/core.db".to_string(),
            },
        ]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::default(),
            log_level: LogLevel::default(),
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: default_osv_db_path(),
            core_db_url: "sqlite:///var/data/core.db".to_string(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "set core db url"
    )]
#[test_case(
        &[
            EnvVar {
                key: "OSV_SERVICE_OSV_SYNC_INTERVAL".to_string(),
                value: "10".to_string(),
            },
        ]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::default(),
            log_level: LogLevel::default(),
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: default_osv_db_path(),
            core_db_url: default_core_db_url(),
            osv_sync_interval: Duration::from_mins(10),
            frontend_path: default_frontend_path(),
            smtp_url: None,
        };
        "set osv sync interval"
    )]
#[test_case(
        &[
            EnvVar {
                key: "OSV_SERVICE_FRONTEND_PATH".to_string(),
                value: "/var/www/frontend".to_string(),
            },
        ]
        => Settings {
            bind_address: default_bind_address(),
            log_format: LogFormat::default(),
            log_level: LogLevel::default(),
            api_url_prefix: default_api_url_prefix(),
            manifest_db_path: default_manifest_db_path(),
            osv_db_path: default_osv_db_path(),
            core_db_url: default_core_db_url(),
            osv_sync_interval: default_osv_sync_interval(),
            frontend_path: PathBuf::from("/var/www/frontend"),
            smtp_url: None,
        };
        "set frontend path"
    )]
fn settings_init_test(env_vars: &[EnvVar]) -> Settings {
    let guard = ENV_LOCK.lock().unwrap();

    let required = [];
    for e in required.iter().chain(env_vars.iter()) {
        unsafe {
            std::env::set_var(&e.key, &e.value);
        }
    }
    let res = Settings::load().unwrap();
    for e in required.iter().chain(env_vars.iter()) {
        unsafe {
            std::env::remove_var(&e.key);
        }
    }
    drop(guard);
    res
}
