#![cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "android",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]

use crate::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget, Hook},
};

/// Additional methods on `EventLoop` to return control flow to the caller.
pub trait EventLoopExtRunReturn {
    /// A type provided by the user that can be passed through `Event::UserEvent`.
    type UserEvent;

    /// Initializes the `winit` event loop.
    ///
    /// Unlike `run`, this function accepts non-`'static` (i.e. non-`move`) closures and returns
    /// control flow to the caller when `control_flow` is set to `ControlFlow::Exit`.
    ///
    /// # Caveats
    /// Despite its appearance at first glance, this is *not* a perfect replacement for
    /// `poll_events`. For example, this function will not return on Windows or macOS while a
    /// window is getting resized, resulting in all application logic outside of the
    /// `event_handler` closure not running until the resize operation ends. Other OS operations
    /// may also result in such freezes. This behavior is caused by fundamental limitations in the
    /// underlying OS APIs, which cannot be hidden by `winit` without severe stability repercussions.
    ///
    /// You are strongly encouraged to use `run`, unless the use of this is absolutely necessary.
    fn run_return<F>(&mut self, event_handler: F)
    where
        F: FnMut(
            Event<'_, Self::UserEvent>,
            &EventLoopWindowTarget<Self::UserEvent>,
            &mut ControlFlow,
        );
}

impl<T, H> EventLoopExtRunReturn for EventLoop<T, H>
where
    H: Hook<T>,
{
    type UserEvent = T;

    fn run_return<F>(&mut self, mut event_handler: F)
    where
        F: FnMut(
            Event<'_, Self::UserEvent>,
            &EventLoopWindowTarget<Self::UserEvent>,
            &mut ControlFlow,
        ),
    {
        let hook = &mut self.hook;
        self.event_loop
            .run_return(move |event, target, control_flow| {
                hook.run(&mut event_handler, event, target, control_flow);
            })
    }
}