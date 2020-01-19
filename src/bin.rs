extern crate ski;
use ski::{string, I, K, P, S, G};

fn main() {
    let t = K;
    let f = S.app(&K);
    let ifz = I;

    // P.app(&ifz.app(&f).app(&G).app(&P).app(&string("> ")));
    
    
    
    
    for _ in 0..1000000 {
        P.app(&ifz.app(&f).app(&G).app(&P).app(&string("> ")));
        // ifz.app(&f).app(&t).app(&string("test"));
    }
}
