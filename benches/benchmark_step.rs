use std::time::Duration;

use conways_game_of_life_lib_rust::Field;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};

const ROWS: u16 = 64;
const COLUMNS: u16 = 64;
const SEED: u64 = 5343542;

fn benchmark_step_singlet(c: &mut Criterion)
{
    let mut group = c.benchmark_group("step_singlet");
    
    let mut field = Field::new(ROWS, COLUMNS);
    set_random_field(&mut field, SEED);

    group.measurement_time(Duration::from_secs(9));

    group.bench_function(
        "step_singlet",
        |b| b.iter(|| field.step_singlet())
    );

    group.finish();
}

fn benchmark_step_multit(c: &mut Criterion)
{
    let mut group = c.benchmark_group("step_multit");
    
    let mut field = Field::new(ROWS, COLUMNS);
    set_random_field(&mut field, SEED);

    group.measurement_time(Duration::from_secs(9));

    group.bench_function(
        "step_multit",
        |b| b.iter(|| field.step_multit())
    );

    group.finish();
}

criterion_group!(benches, benchmark_step_singlet, benchmark_step_multit);
criterion_main!(benches);


fn set_random_field(f: &mut Field, seed: u64)
{
    let mut rng = Pcg32::seed_from_u64(seed);
    for r in 0..f.get_rows() {
        for b in 0..f.get_blocks() {
            *f.get_at(r, b) = rng.gen_range(0..=255);
        }
    }
}