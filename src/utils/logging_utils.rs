use log::{ info, error };

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

// pub fn log_debug(message: &str) {
//     debug!("{}", message);
// }
