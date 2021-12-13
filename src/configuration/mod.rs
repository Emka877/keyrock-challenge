mod datastructs;
mod functions;

// Expose the structs and functions as if they were under crate::configuration
pub use datastructs::*;
pub use functions::read_configuration_file;
