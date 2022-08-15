use gloo::timers::callback::Interval;
use yew::prelude::*;

fn use_interval<S, IS, US>(millis: u32, init_state: IS, update_state: US) -> S
where
    S: Clone + 'static,
    IS: FnOnce() -> S,
    US: FnMut(&mut S) + 'static,
{
    struct IntervalState<S, US> {
        interval: Option<Interval>,
        state: S,
        update_state: US,
    }

    use_hook(
        move || IntervalState {
            interval: None,
            state: init_state(),
            update_state,
        },
        |state, updater| {
            state.interval.get_or_insert_with(|| {
                Interval::new(millis, move || {
                    updater.callback(|state: &mut IntervalState<S, US>| {
                        (state.update_state)(&mut state.state);
                        true
                    });
                })
            });
            state.state.clone()
        },
        |state| drop(state.interval.take()),
    )
}
