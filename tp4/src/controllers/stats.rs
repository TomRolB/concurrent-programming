use crate::server::server::Server;
use crate::{services, utils};

pub fn get_stats(server: &Server) -> String {
    let count_map_arc = server.get_map_arc();
    let count_map = count_map_arc.read().unwrap().clone();
    let stats: String = services::stats::get_stats(count_map);
    utils::response::create_response(200, stats)
}
