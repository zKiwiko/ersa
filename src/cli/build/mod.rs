mod gpc;
mod gpx;

pub fn gpc_build(proj_path: &str) {
    let _ = gpc::build::run_build(proj_path);
}
