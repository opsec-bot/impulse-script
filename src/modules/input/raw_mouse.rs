// Raw mouse delta accumulator.
//
// Deltas are fed in from winit's `DeviceEvent::MouseMotion` (see
// `ui/support.rs`) and drained via `take_delta()`. We intentionally do NOT
// register our own raw-input window: Windows delivers raw input to only the
// most-recently-registered window per device, and winit already registers the
// mouse for the process. A second registration would just fight winit and one
// of the two would silently receive nothing. Routing through winit's
// `DeviceEvent` (with `listen_device_events(Always)`) gives us focus-independent
// raw deltas from the single owner.

use std::sync::atomic::{ AtomicI32, Ordering };

static ACCUM_X: AtomicI32 = AtomicI32::new(0);
static ACCUM_Y: AtomicI32 = AtomicI32::new(0);

/// Feed a raw mouse delta into the accumulator (called from the winit loop).
pub fn add_delta(x: i32, y: i32) {
    if x != 0 {
        ACCUM_X.fetch_add(x, Ordering::Relaxed);
    }
    if y != 0 {
        ACCUM_Y.fetch_add(y, Ordering::Relaxed);
    }
}

/// Drain and reset the accumulated raw mouse delta since the last call.
pub fn take_delta() -> (i32, i32) {
    (ACCUM_X.swap(0, Ordering::Relaxed), ACCUM_Y.swap(0, Ordering::Relaxed))
}

/// Retained for API compatibility. Raw deltas are now sourced from the winit
/// event loop, so there is nothing to spawn here.
pub fn init() {}
