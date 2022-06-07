use actix_web::web::Json;
use intmap::IntMap;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
use types::api::radical::find_kanji::{Request, Response};

/// Get kanji by its radicals
pub async fn kanji_by_radicals(payload: Json<Request>) -> Result<Json<Response>, actix_web::Error> {
    let start = Instant::now();
    let kanji_retr = resources::get().kanji();

    // TODO: see if IntMap makes it really faster
    let mut possible_radicals: HashSet<char> = HashSet::with_capacity(100);
    let mut kanji_res: IntMap<Vec<char>> = IntMap::with_capacity(28);
    //let mut kanji_res: HashMap<u32, Vec<char>> = HashMap::new();

    for kanji in kanji_retr.by_radicals(&payload.radicals) {
        /*
        kanji_res
            .entry(kanji.stroke_count as u32)
            .or_default()
            .push(kanji.literal);
        */

        if let Some(s) = kanji_res.get_mut(kanji.stroke_count as u32) {
            s.push(kanji.literal);
        } else {
            kanji_res.insert(kanji.stroke_count as u32, vec![kanji.literal]);
        }

        if !kanji.parts.is_empty() {
            possible_radicals.extend(kanji.parts.iter());
        }
    }

    let possible_radicals = possible_radicals.into_iter().collect();
    let kanji_res: HashMap<u32, Vec<char>> = kanji_res.into_iter().collect();

    log::debug!("Suggestions took: {:?}", start.elapsed());

    Ok(Json(Response {
        possible_radicals,
        kanji: kanji_res,
    }))
}
