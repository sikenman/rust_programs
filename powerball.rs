use rand::{seq::SliceRandom};

fn main() {

    let mut rng = rand::thread_rng();
    
    for _ in 1..=10 {
        let white_balls: Vec<u8> = (1..70).collect();
        let _power_ball: Vec<u8> = (1..27).collect();
        
        let mut selected_white_balls: Vec<u8> = white_balls
            .choose_multiple(&mut rng, 5)
            .copied()
            .collect();
        
        selected_white_balls.sort();
        
        let selected_power_ball = *_power_ball.choose(&mut rng).unwrap();
        
        println!("WBs {:?}, PB ({selected_power_ball})", selected_white_balls);
    }
}
