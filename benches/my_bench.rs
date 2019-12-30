use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};
use Advent_Of_Code_2019::day9::intcode;

//let mut data=vec![109, -1, 104, 1, 99];
//let mut data=vec![109, -1, 204, 1, 99];
//let mut data=vec![109, 1, 9, 2, 204, -6, 99];
//let mut data=vec![109, 1, 109, 9, 204, -6, 99];

//let mut data=vec![109, 1, 209, -1, 204, -106, 99] ;
//let mut data=vec![109, 1, 3, 3, 204, 2, 99];

//let mut data=vec!    [109, 1, 203, 2, 204, 2, 99];

fn criterion_benchmark(c: &mut Criterion) {
    //let comp = sum_of_primes();
    //day9::intcode::sum_of_primes;
    c.bench(
        "routines",
        Benchmark::new("Sum primes", |b| b.iter(|| intcode::sum_of_primes())).sample_size(10),
    );
    c.bench(
        "routines",
        Benchmark::new("Prime small", |b| b.iter(|| intcode::prime_factor_small())).sample_size(10),
    );
    c.bench(
        "routines",
        Benchmark::new("Prime big}", |b| b.iter(|| intcode::prime_factor_big())).sample_size(10),
    );
    /*
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("sum_of_primes", |b| b.iter(|| sum_of_primes()));
    //.sample_size(20);
    c.bench_function("prime_factor_small", |b| b.iter(|| prime_factor_small()));
    //.sample_size(20);
    c.bench_function("prime_factor_big", |b| b.iter(|| prime_factor_big()));
    //.sample_size(20);*/
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
