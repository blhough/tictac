use criterion::*;
use tictac::*;
use tictac::game::entry::Entry;
use tictac::game::board::{TicTac};
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
	group.bench_function(
		"2",
		|b| b.iter_batched_ref(
			|| Ult::new(),
			|g| {
				g.apply_move2(Entry::X, 4);
				drop(g.generate_moves2(Entry::X))
			},
			BatchSize::SmallInput));
	group.finish();
}

fn bench_tictac(c: &mut Criterion) {
	let mut group = c.benchmark_group("Tictac_gen_apply");
	group.significance_level(0.05).sample_size(1000);
	group.bench_function(
		"1",
		 |b| b.iter_batched_ref(
			 || TicTac::new(),
			 |g| {
				 g.apply_move(Entry::X, 4);
				 g.generate_moves(Entry::X)
			 },
			 BatchSize::SmallInput));
	group.bench_function(
		"2",
		|b| b.iter_batched_ref(
			|| TicTac::new(),
			|g| {
				g.apply_move2(Entry::X, 4);
				drop(g.generate_moves2(Entry::X))
			},
			BatchSize::SmallInput));
	group.finish();
}

criterion_group!(benches, bench_ult, bench_tictac);
criterion_main!(benches);