use std::sync::Arc;

use tokio::sync::Mutex;
use axum::{extract::Path, routing::get, Router, response::IntoResponse, Json, Extension};
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

struct Lotto<'a> {
    pot: Vec<u32>,
    rng: &'a mut SmallRng,
}

impl<'a> Lotto<'a> {
    fn new(pot_size: u32, rng: &'a mut SmallRng) -> Self {
        Self { 
            pot: (1..=pot_size).collect(),
            rng,
        }
    }

    fn take(&mut self, amount: usize) -> Vec<u32> {
        self.pot.shuffle(&mut self.rng);
        self.pot.iter()
            .take(amount)
            .map(|e| e.to_owned()).collect()
    }
}

type SharedState = Arc<Mutex<SmallRng>>;

async fn handler_lotto(
    Path((pot_size, amount)): Path<(u32, usize)>,
    Extension(state): Extension<SharedState>,
) -> impl IntoResponse {
    let mut rng = SmallRng::from_entropy();
    let mut lotto = Lotto::new(pot_size, &mut rng);
    let results = lotto.take(amount);
    Json(results)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let state = Arc::new(Mutex::new(SmallRng::from_entropy()));
    let router = Router::new()
        .route("/lotto/:pot/:amount", get(handler_lotto))
        .layer(Extension(state));

    Ok(router.into())
}
