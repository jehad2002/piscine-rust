use ref_cell::*;

fn main() {
    let logger = Worker::new(1);
    let track = messenger::Tracker::new(&logger, 10);

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

    for (k, v) in logger.mapped_messages.into_inner() {
        println!("{:?}", (k, v));
    }

    println!("{:?}", logger.all_messages.into_inner());
}
