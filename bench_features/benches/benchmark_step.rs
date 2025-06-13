use conways_game_of_life_dyn_lib::{ChunkCellData8x8, Field, rule_fns};
use std::time::Duration;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};


fn benchmark_step_singlet(c: &mut Criterion)
{
    let mut group = c.benchmark_group("Step single thread");

    group.warm_up_time(Duration::from_secs(16));
    group.measurement_time(Duration::from_secs(300));
    group.sample_size(220);
    //group.sampling_mode(SamplingMode::Auto);

    {
        let pattern = ChunkCellData8x8::from(0x40E0303);// Chaos pattern

        group.bench_function(
            "step 'chaos' pattern",
            |b| b.iter(|| {
                let mut f = Field::new();
                f.add_new_chunk(0, 0, pattern);
                for _ in 0..220 {
                    f.step(rule_fns::rule_conways_game_of_life, 3);
                }
            })
        );

        group.bench_function(
            "step 'chaos' pattern branchless",
            |b| b.iter(|| {
                let mut f = Field::new();
                f.add_new_chunk(0, 0, pattern);
                for _ in 0..220 {
                    f.step(rule_fns::rule_conways_game_of_life_branchless, 3);
                }
            })
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_step_singlet);
criterion_main!(benches);