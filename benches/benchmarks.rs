use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use gh_actions::{User, UserManager, calculate_fibonacci, calculate_fibonacci_recursive, is_prime, factorial, validate_email};

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    
    for i in [10, 20, 30, 40].iter() {
        group.bench_with_input(BenchmarkId::new("iterative", i), i, |b, i| {
            b.iter(|| calculate_fibonacci(black_box(*i)))
        });
        
        group.bench_with_input(BenchmarkId::new("recursive", i), i, |b, i| {
            b.iter(|| calculate_fibonacci_recursive(black_box(*i)))
        });
    }
    
    group.finish();
}

fn bench_prime_checking(c: &mut Criterion) {
    let mut group = c.benchmark_group("prime_checking");
    
    let test_numbers = vec![97, 1009, 10007, 100003];
    
    for num in test_numbers.iter() {
        group.bench_with_input(BenchmarkId::new("is_prime", num), num, |b, num| {
            b.iter(|| is_prime(black_box(*num)))
        });
    }
    
    group.finish();
}

fn bench_factorial(c: &mut Criterion) {
    let mut group = c.benchmark_group("factorial");
    
    for i in [5, 10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::new("factorial", i), i, |b, i| {
            b.iter(|| factorial(black_box(*i)))
        });
    }
    
    group.finish();
}

fn bench_email_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("email_validation");
    
    let emails = vec![
        "test@example.com",
        "very.long.email.address@very-long-domain-name.co.uk",
        "invalid.email",
        "user+tag@domain.org",
    ];
    
    for (i, email) in emails.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("validate", i), email, |b, email| {
            b.iter(|| validate_email(black_box(email)))
        });
    }
    
    group.finish();
}

fn bench_user_manager_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("user_manager");
    
    // Benchmark adding users
    group.bench_function("add_user", |b| {
        b.iter_batched(
            || UserManager::new(),
            |mut manager| {
                let user = User {
                    id: 1,
                    name: "Test User".to_string(),
                    email: "test@example.com".to_string(),
                    active: true,
                };
                manager.add_user(black_box(user)).unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    // Benchmark getting users from populated manager
    group.bench_function("get_user", |b| {
        b.iter_batched(
            || {
                let mut manager = UserManager::new();
                for i in 1..=100 {
                    let user = User {
                        id: i,
                        name: format!("User {}", i),
                        email: format!("user{}@example.com", i),
                        active: i % 2 == 0,
                    };
                    manager.add_user(user).unwrap();
                }
                manager
            },
            |manager| {
                black_box(manager.get_user(50));
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    // Benchmark filtering active users
    group.bench_function("get_active_users", |b| {
        b.iter_batched(
            || {
                let mut manager = UserManager::new();
                for i in 1..=1000 {
                    let user = User {
                        id: i,
                        name: format!("User {}", i),
                        email: format!("user{}@example.com", i),
                        active: i % 2 == 0,
                    };
                    manager.add_user(user).unwrap();
                }
                manager
            },
            |manager| {
                black_box(manager.get_active_users());
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    // Benchmark deleting users
    group.bench_function("delete_user", |b| {
        b.iter_batched(
            || {
                let mut manager = UserManager::new();
                for i in 1..=100 {
                    let user = User {
                        id: i,
                        name: format!("User {}", i),
                        email: format!("user{}@example.com", i),
                        active: true,
                    };
                    manager.add_user(user).unwrap();
                }
                manager
            },
            |mut manager| {
                manager.delete_user(black_box(50)).unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

fn bench_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_operations");
    
    // Benchmark adding many users
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("add_many_users", size), size, |b, size| {
            b.iter_batched(
                || UserManager::new(),
                |mut manager| {
                    for i in 1..=*size {
                        let user = User {
                            id: i,
                            name: format!("User {}", i),
                            email: format!("user{}@example.com", i),
                            active: i % 2 == 0,
                        };
                        manager.add_user(user).unwrap();
                    }
                    black_box(manager);
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_fibonacci,
    bench_prime_checking,
    bench_factorial,
    bench_email_validation,
    bench_user_manager_operations,
    bench_bulk_operations
);

criterion_main!(benches);