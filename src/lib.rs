//! This is a small library that defines a SM2 inspired spaced repetition algorithm. It tries to
//! assume as little as possible about your items. It also tries to include some self adjusting
//! behaviour such that we model the forgetting curve as well as possible. We also include some
//! randomness such that we decouple items that were created together and allow their review events
//! to spread out in time.
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]


// stdlib imports
use std::f32::consts::E;

// external crate imports
use rand::Rng;

/// User evaluation of review event. Did it feel too hard, just difficult enough or too easy?
pub enum UserReview {
    /// The forgetting curve decreases faster than expected, increase difficulty of item
    TooHard, 
    /// The forgetting curve decreases just enough,
    JustEnough, 
    /// The forgetting curve decreases slower than expected, decrease difficulty of item
    TooEasy, 
}

/// computes the number of days between the current review event and the next given a wanted recall
/// probability. Expects a positive forgetting rate. Is not used directly can but is exposed
/// anyway.
#[inline]
pub fn compute_intervall(forgetting_rate: f32, probability: f32) -> i32 {
    assert!(forgetting_rate.is_sign_positive());
    assert!(probability < 1.0);

    let n_days_f = probability.log(E) / (-forgetting_rate);
    n_days_f as i32
}

/// Struct containing item specific data related to it's scheduling. 
pub struct SchedulingData {
    intervall: i32,
    difficulty: f32,
    memory_strength: f32,
    adjusting_factor: f32,
    times_reviewed: i32,
    times_recalled: i32,
}

/// struct containing various parameters used to update the scheduling data of an item
#[derive(Clone, Debug)]
pub struct UpdateParameters {
    /// the factor (in percent) that the difficulty is increased/decreased if the user finds the
    /// item to hard/easy
    pub difficulty_change_factor: f32,
    /// the faactor (in percent) that the memory_strength is increased when reviewing an item
    pub memory_strength_change_factor: f32,
}


impl Default for SchedulingData {
    fn default() -> Self {
        // Here we want the initial ratio between the difficulty and the memory strength to be
        // around -ln(0.9) =approx 0.1 (this results in the first interval being around 1 day)
        // I therefore simply set the difficulty to that value, then scale both it and the memory
        // strength by 100
        SchedulingData { intervall: 1, difficulty: 10.0, memory_strength: 100.0, adjusting_factor: 1.0, times_reviewed: 0, times_recalled: 0 }
    }
}

impl Default for UpdateParameters {
    fn default() -> Self {
        Self {
            difficulty_change_factor: 1.1,
            memory_strength_change_factor: 1.60,
        }

    }
}

/// main scheduling function. Takes the scheduling data of an item, and the result of the review
/// event and computes the next intervall + changes to the item parameters.
pub fn schedule(item_data: SchedulingData, user_review: UserReview, update_parameters: UpdateParameters, probability: f32) -> SchedulingData {
    // The value of f will be the quotient difficulty/memory_strength.
    // If we want to the ratio between the new and old interval to be A then that formes the
    // following equation: t2 = A * t1. Which if expanded becomes:  A * ln(P)/-f1 = ln(P)/-f2. Out
    // of this we then get the equation f1 * 1/A = f2 which can then be used to calculate the new
    // value of f. Since f is a quotient we calculate this by first multiplying our wanted ratio
    // with the memory_strength, update the difficulty with the user review and then compute the
    // quotient.

    // old data
    let SchedulingData { intervall: _, difficulty, memory_strength, adjusting_factor, times_reviewed, times_recalled } = item_data;
    
    let new_difficulty = match user_review {
        UserReview::TooHard => difficulty * update_parameters.difficulty_change_factor,
        UserReview::JustEnough => difficulty,
        UserReview::TooEasy => difficulty * (2.0  - update_parameters.difficulty_change_factor),
    };

    let new_memory_strength = memory_strength * update_parameters.memory_strength_change_factor;
    let new_forgetting_rate = (1.0 / adjusting_factor) * (difficulty / memory_strength);
    let next_interval_no_random = compute_intervall(new_forgetting_rate, probability);
    
    // we then want to introduce some noise in the interval
    let mut rng = rand::thread_rng();
    let random_range = next_interval_no_random / 10;
    let random_change = rng.gen_range(0..random_range*2) - random_range;
    let next_interval = next_interval_no_random + random_change;

    SchedulingData {
        intervall: next_interval,
        difficulty: new_difficulty,
        memory_strength: new_memory_strength,
        adjusting_factor,
        times_reviewed: times_reviewed + 1,
        times_recalled: times_recalled + 1,
    }
}

/// Computes how the ratio between review intervalls should be scaled to more accurately
/// align with the true forgetting curve. Computed as explained [here](https://docs.ankiweb.net/deck-options.html#interval-modifier)
pub fn update_adjusting_factor(item_data: SchedulingData, target_probability: f32) -> SchedulingData { 
    let SchedulingData { intervall, difficulty, memory_strength, adjusting_factor: _, times_reviewed, times_recalled } = item_data;

    // the actual recall probability for this item
    let actual_probability = times_recalled as f32 / times_reviewed as f32;

    let new_adjusting_factor = target_probability.log(E) / actual_probability.log(E);

    SchedulingData {
        intervall,
        difficulty,
        memory_strength,
        adjusting_factor: new_adjusting_factor,
        times_reviewed,
        times_recalled
    }
}
