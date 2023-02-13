
use rand::Rng;
use std::f64::consts::PI;

fn is_inside((x,y):(f64,f64)) -> bool {

    x * x + y * y <= 1.0
}
fn simulate<R: Rng>(rng: &mut R, samples: usize) -> f64 {

    let mut count = 0;
    for _ in 0..samples { 
        if is_inside(rng.gen()) {
            count +=1;
        }
    }
    (count as f64) / (samples as f64)
}
fn main() {
    //let mut rng = SmallRng::from_entropy();
    let mut rng = rand::thread_rng();
    println!("Real pi: {}",PI);
    for samples in (3..9).map(|e| 10_usize.pow(e)) { 
        let estimate = 4.0 * simulate(&mut rng,samples);
        let deviation = 100.0 * (1.0 - estimate / PI).abs();
        println!("{:9}: {:<11} dev: {:.5}%", samples, estimate, deviation);
    }
}
