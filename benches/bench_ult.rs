use criterion::*;
use tictac::game::entry::Entry;
use tictac::game::ult::{Ult};
use tictac::game::Game;

fn bench_ult(c: &mut Criterion) {
	let mut group = c.benchmark_group("Ult_gen_apply");
	group.significance_level(0.05).sample_size(1000);
	group.bench_function(
		"1",
		 |b| b.iter_batched_ref(
			 || Ult::new(),
			 |g| {
				 g.apply_move(Entry::X, 4);
				 g.generate_moves(Entry::X)
			 },
			 BatchSize::SmallInput));
	group.finish();
}

fn bench_ult2(c: &mut Criterion) {
	let mut group = c.benchmark_group("Ult_rand_gen_apply");
	group.significance_level(0.05).sample_size(1000);
	group.bench_function(
		"1",
		 |b| b.iter_batched_ref(
			 || Ult::new(),
			 |g| {
				 g.apply_move(Entry::X, 4);
				 g.generate_random_move(Entry::X)
			 },
			 BatchSize::SmallInput));
	group.finish();
}

criterion_group!(benches, bench_ult, bench_ult2);
criterion_main!(benches);