//! Custom hooks for Dioxus components

use std::future::Future;
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::{use_future, use_hook, ReadableExt, Signal, WritableExt};
#[cfg(target_arch = "wasm32")]
use futures_channel::mpsc;
#[cfg(target_arch = "wasm32")]
use futures_util::StreamExt;

/// A sender that can be used to trigger async callbacks from sync contexts.
///
/// This is useful when you need to run async code from places that don't have
/// access to the Dioxus runtime (like gloo EventListener callbacks).
#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct CallbackSender<T> {
    tx: mpsc::UnboundedSender<T>,
}

#[cfg(target_arch = "wasm32")]
impl<T> CallbackSender<T> {
    /// Send a value to trigger the async callback.
    /// This is a sync operation that won't block.
    pub fn send(&self, value: T) {
        let _ = self.tx.unbounded_send(value);
    }
}

/// Creates a channel that bridges sync code to async callbacks in the Dioxus runtime.
///
/// Returns a [`CallbackSender`] that can be cloned and used from any sync context
/// (like gloo EventListener callbacks). When you call `send()`, the provided async
/// callback will be executed in the component's scope.
///
/// # Example
///
/// ```ignore
/// let focus_sender = use_callback_channel(move |()| async move {
///     if let Some(el) = input_element.cloned() {
///         let _ = el.set_focus(true).await;
///     }
/// });
///
/// // Later, in a gloo EventListener:
/// let tx = focus_sender.clone();
/// EventListener::new(&window, "keydown", move |_| {
///     tx.send(()); // Triggers the async callback
/// });
/// ```
///
/// # Why this exists
///
/// gloo EventListener callbacks run outside the Dioxus runtime context, so you can't:
/// - Do async work (no `spawn`, no `.await`)
/// - Reliably use signals
///
/// But you CAN send through an unbounded channel (sync operation), which this hook
/// uses to bridge to a `use_future` that runs in the component scope.
#[cfg(target_arch = "wasm32")]
pub fn use_callback_channel<T, F, Fut>(callback: F) -> CallbackSender<T>
where
    T: 'static,
    F: Fn(T) -> Fut + 'static,
    Fut: Future<Output = ()> + 'static,
{
    use std::rc::Rc;

    // Create channel and callback in signals - signals are Copy, no cloning needed
    let (tx, mut rx_signal, callback) = use_hook(|| {
        let (tx, rx) = mpsc::unbounded::<T>();
        let rx_signal: Signal<Option<mpsc::UnboundedReceiver<T>>> = Signal::new(Some(rx));
        let callback: Signal<Rc<dyn Fn(T) -> Fut>> = Signal::new(Rc::new(callback) as Rc<dyn Fn(T) -> Fut>);
        (tx, rx_signal, callback)
    });

    // Run the receiver loop in a future
    let _receiver = use_future(move || async move {
        // Use try_write in case the component is unmounting
        let Ok(mut guard) = rx_signal.try_write() else {
            return;
        };
        let Some(mut rx) = guard.take() else {
            return;
        };
        drop(guard); // Release the write lock before awaiting

        while let Some(value) = rx.next().await {
            // Use try_read in case the component died
            let Ok(cb) = callback.try_read() else {
                return;
            };
            cb(value).await;
        }
    });

    CallbackSender { tx }
}

/// Non-wasm stub for CallbackSender to allow compilation on server
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct CallbackSender<T> {
    _marker: std::marker::PhantomData<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<T> CallbackSender<T> {
    /// Send a value (no-op on non-wasm targets)
    pub fn send(&self, _value: T) {}
}

/// Non-wasm stub for use_callback_channel (no-op on server)
#[cfg(not(target_arch = "wasm32"))]
pub fn use_callback_channel<T, F, Fut>(_callback: F) -> CallbackSender<T>
where
    T: 'static,
    F: Fn(T) -> Fut + 'static,
    Fut: Future<Output = ()> + 'static,
{
    CallbackSender {
        _marker: std::marker::PhantomData,
    }
}
