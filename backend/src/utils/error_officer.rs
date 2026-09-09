pub fn error_env(env_name: &str) {
        tracing::error!(
                "There's an error when trying to get env variable! env name: {}",
                env_name
        );
}

pub fn error_parse(variable_name: &str) {
        tracing::error!(
                "There's an error when trying to parse variable! variable name: {}",
                variable_name
        );
}

pub fn error_db(error_message: &str) {
        tracing::error!(
                "There's an error when trying to do database operation! error: {}",
                error_message
        );
}
