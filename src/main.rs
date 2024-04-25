// Things to think about:
// Auth?
// Sync vs async?
// How to format errors?
// Allow all HTTP methods?
// Allow any form of HTTP result?

#[derive(serde::Serialize)]
pub struct CalculationResult {
    z: f64,
}

#[derive(serde::Deerialize)]
pub struct CalculationInput {
    x: f64,
    y: f64
}

pub enum Error {
    InternalError,
}

// More hands-on way of exposing a lambda-like endpoint
#[shuttle_runtime::faas_post("/mutliply")]
pub async fn multiply(input: CalculationInput) -> Result<CalculationResult, Error> {
    Ok(CalculationResult {
        z: input.x * input.y
    })
}

// Could it be simplified such that someone would only need to do the below? Shuttle
// would then generate the JSON types
#[shuttle_runtime::fast_post("/multiply_simple")]
pub async fn multiply_simple(x: f64, y: f64) -> f64 {
    x * y
}

fn main() {}