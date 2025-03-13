use std::{num::NonZero, time::Duration};

use conways_game_of_life_dyn_lib::{game_of_life::calculate_rules_classic, set_field_chunks, Field};

use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};

fn benchmark_step_singlet(c: &mut Criterion)
{
    let mut group = c.benchmark_group("step_singlet");

    let mut f = set_field_chunks!(
        0,0,0b00000100_00001110_00000011_00000011;// Chaos
    );

    group.measurement_time(Duration::from_secs(9));

    group.bench_function(
        "step_singlet",
        |b| b.iter(|| step_singlet_runner(&mut f, NonZero::new(220).unwrap()))
    );

    group.finish();
}

fn step_singlet_runner(f: &mut Field, times: NonZero<u8>)
{
    for _i in 0..times.get() {
        f.step_singlet(calculate_rules_classic, 3);
    }
}

criterion_group!(benches, benchmark_step_singlet);
criterion_main!(benches);