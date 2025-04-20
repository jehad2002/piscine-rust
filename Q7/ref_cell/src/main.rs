use ref_cell::*;
use ref_cell::messenger::Tracker;

fn main() {
    let logger = Worker::new(1);
    let track = Tracker::new(&logger, 10);

    let _a = logger.track_value.clone();
    let _a1 = logger.track_value.clone();
    let _a2 = logger.track_value.clone();

    track.peek(&logger.track_value);

    let _b = logger.track_value.clone();
    let _b1 = logger.track_value.clone();
    let _b2 = logger.track_value.clone();
    let _b3 = logger.track_value.clone();

    track.set_value(&logger.track_value);

    let _c = logger.track_value.clone();
    track.set_value(&logger.track_value);

    let _c1 = logger.track_value.clone();
    track.set_value(&logger.track_value);

    for (k, v) in logger.mapped_messages.borrow().iter() {
        println!("{:?}", (k, v));
    }

    println!("{:?}", logger.all_messages.borrow());
}
