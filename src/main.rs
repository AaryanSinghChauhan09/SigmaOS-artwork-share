//! SigmaOS-mint-artwork-share — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_mint_artwork_share_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-mint-artwork-share: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-mint-artwork-share: ok");
}
