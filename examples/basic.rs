use bumper_alloc::BumpAllocator;

#[derive(Debug)]
struct Particle {
    x: f32,
    y: f32,
    velocity: f32,
}

#[derive(Debug)]
struct Request {
    id: u32,
    payload_size: usize,
}

fn main() {
    println!("=== Bumper Arena Allocator Demo ===\n");

    // --- Example 1: Basic typed allocation ---
    println!("[ 1 ] Typed allocation");
    let mut arena = BumpAllocator::new(1024);

    let x = arena.alloc_val(42u32).expect("alloc failed");
    println!("    allocated u32: {}", x);

    let p = arena.alloc_val(Particle {
        x: 1.5,
        y: 2.5,
        velocity: 9.8,
    }).expect("alloc failed");
    println!("    allocated Particle: {:?}", p);
    println!("    used: {} bytes, remaining: {} bytes\n", arena.used(), arena.remaining());

    // --- Example 2: Simulate per-frame game allocations ---
    println!("[ 2 ] Simulating game loop (3 frames)");
    let mut arena = BumpAllocator::new(1024 * 64);

    for frame in 0..3 {
        for i in 0..5 {
            arena.alloc_val(Particle {
                x: i as f32 * 0.1,
                y: i as f32 * 0.2,
                velocity: 9.8,
            }).expect("alloc failed");
        }
        println!("    frame {}: allocated 5 particles, used {} bytes", frame, arena.used());
        arena.reset(); // free everything at end of frame
        println!("    frame {}: reset, used {} bytes", frame, arena.used());
    }

    // --- Example 3: Simulate per-request server allocations ---
    println!("\n[ 3 ] Simulating web server requests");
    let mut arena = BumpAllocator::new(1024 * 64);

    for req_id in 0..4 {
        let req = arena.alloc_val(Request {
            id: req_id,
            payload_size: 256,
        }).expect("alloc failed");
        println!("    handling request #{}, payload: {} bytes", req.id, req.payload_size);
        arena.reset(); // free after each request
    }

    // --- Example 4: Out of memory handling ---
    println!("\n[ 4 ] Out of memory handling");
    let mut arena = BumpAllocator::new(4);
    match arena.alloc_val(42u32) {
        Some(v) => println!("    allocated: {}", v),
        None => println!("    allocation failed: out of memory"),
    }
    match arena.alloc_val(42u32) {
        Some(v) => println!("    allocated: {}", v),
        None => println!("    allocation failed: out of memory"),
    }

    println!("\n=== done ===");
}