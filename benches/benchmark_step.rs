use conways_game_of_life_rust::Field;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};

const ROWS: u16 = 512;
const COLUMNS: u16 = 512;
const SEED: u64 = 5343542;

fn benchmark_step_singlet(c: &mut Criterion)
{
    let mut field = Field::new(ROWS, COLUMNS);
    /*
    *field.get_at(0,0) = 2;
    *field.get_at(1,0) = 4;
    *field.get_at(2,0) = 7;
    */
    set_random_field(&mut field, SEED);

    c.bench_function(
        "step_singlet",
        |b| b.iter(|| field.step_singlet())
    );
}

fn benchmarkstep_multit(c: &mut Criterion)
{
    let mut field = Field::new(ROWS, COLUMNS);

    /*
    *field.get_at(0,0) = 2;
    *field.get_at(1,0) = 4;
    *field.get_at(2,0) = 7;
    */

    set_random_field(&mut field, SEED);

    c.bench_function(
        "step_multit",
        |b| b.iter(|| field.step_multit())
    );
}

criterion_group!(benches, benchmark_step_singlet, benchmarkstep_multit);
criterion_main!(benches);


fn set_random_field(f: &mut Field, seed: u64)
{
    let mut rng = Pcg32::seed_from_u64(seed);
    for r in 0..f.rows {
        for b in 0..f.blocks {
            *f.get_at(r, b) = rng.gen_range(0..=255);
        }
    }
}