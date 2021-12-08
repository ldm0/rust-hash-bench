use digest::Digest;
use once_cell::sync::Lazy;
use std::{collections::BTreeMap, time::Instant};

static BYTES: Lazy<Vec<u8>> = Lazy::new(|| (0..20 * 1024 * 1024).map(|_| rand::random()).collect());

#[derive(Debug)]
struct HashInfo {
    speed: f32,
}

fn test<const ITER_COUNT: usize, const CHUNK_SIZE: usize>() -> BTreeMap<&'static str, HashInfo> {
    let mut hash_info = BTreeMap::new();
    let chunky_data = &BYTES[..CHUNK_SIZE];

    // blake2
    {
        {
            let mut x = blake2::Blake2b::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "blake2::Blake2b",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = blake2::Blake2s::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "blake2::Blake2s",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }
    }

    // blake3
    {
        {
            let mut x = blake3::Hasher::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "blake3::Blake3",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }
    }

    // sha2
    {
        {
            let mut x = sha2::Sha224::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha2::sha224",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha2::Sha256::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha2::sha256",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha2::Sha384::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha2::sha384",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha2::Sha512::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha2::sha512",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha2::Sha512Trunc224::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha2::sha512trunc224",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha2::Sha512Trunc256::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha2::sha512trunc256",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }
    }

    // sha3
    {
        {
            let mut x = sha3::Sha3_224::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::SHA3-224",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Sha3_256::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::SHA3-256",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Sha3_384::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::SHA3-384",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Sha3_512::new();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::SHA3-512",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Shake128::default();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                digest::Update::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::Shake128",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Shake256::default();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                digest::Update::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::Shake256",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Keccak224::default();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                digest::Update::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::Keccak224",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Keccak256::default();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                digest::Update::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::Keccak256",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Keccak256Full::default();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                digest::Update::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::Keccak256Full",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Keccak384::default();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                digest::Update::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::Keccak384",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = sha3::Keccak512::default();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                digest::Update::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "sha3::Keccak512",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }
    }

    // ring
    {
        {
            let mut x = ring::digest::Context::new(&ring::digest::SHA256);
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "ring::SHA-256",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = ring::digest::Context::new(&ring::digest::SHA384);
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "ring::SHA-384",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = ring::digest::Context::new(&ring::digest::SHA512);
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                x.update(chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "ring::SHA-512",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }
    }

    // tiny-keccak
    {
        {
            let mut x = tiny_keccak::KangarooTwelve::new(&[] as &[u8]);
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::KangarooTwelve",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = tiny_keccak::Sha3::v224();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::SHA3-224",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = tiny_keccak::Sha3::v256();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::SHA3-256",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = tiny_keccak::Sha3::v384();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::SHA3-384",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = tiny_keccak::Sha3::v512();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::SHA3-512",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = tiny_keccak::Keccak::v224();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::Keccak224",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }

        {
            let mut x = tiny_keccak::Keccak::v256();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::Keccak256",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }
        {
            let mut x = tiny_keccak::Keccak::v384();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::Keccak384",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }
        {
            let mut x = tiny_keccak::Keccak::v512();
            let time = Instant::now();
            for _ in 0..ITER_COUNT {
                tiny_keccak::Hasher::update(&mut x, chunky_data);
            }
            let elapsed = time.elapsed().as_secs_f32();
            hash_info.insert(
                "tiny_keccak::Keccak512",
                HashInfo {
                    speed: (ITER_COUNT * CHUNK_SIZE) as f32 / elapsed,
                },
            );
        }
    }

    hash_info
}

fn main() {
    let _: &[u8] = BYTES.as_ref();

    {
        const ITER_COUNT: usize = 10_000_000;
        const CHUNK_SIZE: usize = 64;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!("\nChunk size: {}b, iter_count: {}", CHUNK_SIZE, ITER_COUNT);
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 5_000_000;
        const CHUNK_SIZE: usize = 128;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!("\nChunk size: {}b, iter_count: {}", CHUNK_SIZE, ITER_COUNT);
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 2_500_000;
        const CHUNK_SIZE: usize = 256;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!("\nChunk size: {}b, iter_count: {}", CHUNK_SIZE, ITER_COUNT);
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 1_250_000;
        const CHUNK_SIZE: usize = 512;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!("\nChunk size: {}b, iter_count: {}", CHUNK_SIZE, ITER_COUNT);
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 500_000;
        const CHUNK_SIZE: usize = 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 250_000;
        const CHUNK_SIZE: usize = 2 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 100_000;
        const CHUNK_SIZE: usize = 4 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 50_000;
        const CHUNK_SIZE: usize = 8 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 20_000;
        const CHUNK_SIZE: usize = 16 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 10_000;
        const CHUNK_SIZE: usize = 32 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 5_000;
        const CHUNK_SIZE: usize = 64 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 2_500;
        const CHUNK_SIZE: usize = 128 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 1_000;
        const CHUNK_SIZE: usize = 256 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 1_000;
        const CHUNK_SIZE: usize = 512 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}KB, iter_count: {}",
            CHUNK_SIZE / 1024,
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 1_000;
        const CHUNK_SIZE: usize = 1 * 1024 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}MB, iter_count: {}",
            CHUNK_SIZE / (1024 * 1024),
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }

    {
        const ITER_COUNT: usize = 1000;
        const CHUNK_SIZE: usize = 2 * 1024 * 1024;
        let result = test::<ITER_COUNT, CHUNK_SIZE>();
        println!(
            "\nChunk size: {}MB, iter_count: {}",
            CHUNK_SIZE / (1024 * 1024),
            ITER_COUNT
        );
        for (name, info) in result {
            println!(
                "Name: {} \t speed: {:#?}MB/s",
                name,
                info.speed / (1024. * 1024.)
            );
        }
    }
}
