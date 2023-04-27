//! Events implementation. Events behave in a similar way to JavaScript Events. When an event is
//! emitted, it is propagated in three stages: capturing, target, and bubbling. Each stage is
//! configurable and some events propagation can be cancelled. To learn more about the mechanics,
//! see: https://javascript.info/bubbling-and-capturing.

use crate::prelude::*;

use crate::display::object::instance::Instance;
use crate::display::object::instance::WeakInstance;



// =============
// === State ===
// =============

/// Event state. It can be used to determine whether the event is being propagated, its propagation
/// is cancelled, or that the propagation cannot be cancelled. See docs of this module to learn
/// more.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Running,
    RunningNonCancellable,
    Cancelled,
}



// =================
// === SomeEvent ===
// =================

/// Similar to [`Event`] but with a hidden payload type. It is used to construct, configure, and
/// emit new events.
#[allow(missing_docs)]
#[derive(Clone, CloneRef, Debug)]
pub struct SomeEvent {
    pub data:       frp::AnyData,
    state:          Rc<Cell<State>>,
    current_target: Rc<RefCell<Option<WeakInstance>>>,
    /// Indicates whether the event participates in the capturing phase.
    pub captures:   Rc<Cell<bool>>,
    /// Indicates whether the event participates in the bubbling phase.
    pub bubbles:    Rc<Cell<bool>>,
}

impl SomeEvent {
    /// Constructor.
    pub fn new<T: 'static>(target: Option<WeakInstance>, payload: T) -> Self {
        let event = Event::new(target, payload);
        let state = event.state.clone_ref();
        let current_target = event.current_target.clone_ref();
        let captures = Rc::new(Cell::new(true));
        let bubbles = Rc::new(Cell::new(true));
        Self { data: frp::AnyData::new(event), state, current_target, captures, bubbles }
    }

    /// The [`State]` of the event.
    pub fn state(&self) -> State {
        self.state.get()
    }

    /// Check whether the event was cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.state() == State::Cancelled
    }

    /// Enables or disables bubbling for this event.
    pub fn set_bubbling(&self, value: bool) {
        self.bubbles.set(value);
    }

    /// Set the current target of the event. This is internal function and should not be used
    /// directly.
    pub(crate) fn set_current_target(&self, target: Option<&Instance>) {
        self.current_target.replace(target.map(|t| t.downgrade()));
    }
}

impl Default for SomeEvent {
    fn default() -> Self {
        Self::new::<()>(None, ())
    }
}



// =============
// === Event ===
// =============

/// The [`Event`] interface represents an event which takes place in the EnsoGL display object
/// hierarchy.
///
/// An event can be triggered by the user action e.g. clicking the mouse button or tapping keyboard,
/// or generated by APIs to represent the progress of an asynchronous task. It can also be triggered
/// programmatically, such as by calling the [`display::object::Instance::focus()`] method of an
/// element, or by defining the event, then sending it to a specified target using
/// [`display::object::Instance::event_source::emit(...)`].
///
/// See the JavaScript counterpart of this struct:
/// https://developer.mozilla.org/en-US/docs/Web/API/Event.
#[derive(Derivative, Deref)]
#[derivative(Clone(bound = ""))]
#[derivative(Default(bound = "T: Default"))]
pub struct Event<T> {
    data: Rc<EventData<T>>,
}

impl<T: Debug> Debug for Event<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

/// Internal representation of [`Event`].
#[allow(missing_docs)]
#[derive(Deref, Derivative)]
#[derivative(Default(bound = "T: Default"))]
pub struct EventData<T> {
    #[deref]
    pub payload:    T,
    target:         Option<WeakInstance>,
    current_target: Rc<RefCell<Option<WeakInstance>>>,
    state:          Rc<Cell<State>>,
}

impl<T: Debug> Debug for EventData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Event")
            .field("payload", &self.payload)
            .field("state", &self.state.get())
            .finish()
    }
}

impl<T> Event<T> {
    fn new(target: Option<WeakInstance>, payload: T) -> Self {
        let state = default();
        let current_target = Rc::new(RefCell::new(target.clone()));
        let data = Rc::new(EventData { payload, target, current_target, state });
        Self { data }
    }

    /// Prevents further propagation of the current event in the capturing and bubbling phases. It
    /// also does NOT prevent immediate propagation to other event-handlers.
    ///
    /// See: https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation.
    pub fn stop_propagation(&self) {
        if self.state.get() == State::RunningNonCancellable {
            warn!("Trying to cancel a non-cancellable event.");
        } else {
            self.state.set(State::Cancelled);
        }
    }

    /// A reference to the object onto which the event was dispatched.
    ///
    /// See: https://developer.mozilla.org/en-US/docs/Web/API/Event/target.
    pub fn target(&self) -> Option<Instance> {
        self.data.target.as_ref().and_then(|t| t.upgrade())
    }

    /// The current target for the event, as the event traverses the display object hierarchy. It
    /// always refers to the element to which the event handler has been attached, as opposed to
    /// [`Self::target`], which identifies the element on which the event occurred and which may be
    /// its descendant.
    ///
    /// # Important Note
    /// The value of [`Self::current_target`] is only available while the event is being handled. If
    /// store the event in a variable and read this property later, the value will be [`None`].
    pub fn current_target(&self) -> Option<Instance> {
        self.data.current_target.borrow().as_ref().and_then(|t| t.upgrade())
    }
}



// ====================
// === Basic Events ===
// ====================

/// The [`Focus`] event fires when an element has received focus. The event does not bubble, but the
/// related [`FocusIn`] event that follows does bubble.
///
/// The opposite of [`Focus`] is the [`Blur`] event, which fires when the element has lost focus.
///
/// The [`Focus`] event is not cancelable.
///
/// See: https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event.
#[derive(Clone, Copy, Debug, Default)]
pub struct Focus;

/// The [`Blur`] event fires when an element has lost focus. The event does not bubble, but the
/// related [`FocusOut`] event that follows does bubble.
///
/// The opposite of [`Blur`] is the [Focus] event, which fires when the element has received focus.
/// The [`Blur`] event is not cancelable.
///
/// See: https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event.
#[derive(Clone, Copy, Debug, Default)]
pub struct Blur;

/// The [`FocusIn`] event fires when an element has received focus, after the [`Focus`] event. The
/// two events differ in that [`FocusIn`] bubbles, while [`Focus`] does not.
///
/// The opposite of [`FocusIn`] is the [`FocusOut`] event, which fires when the element has lost
/// focus. The [`FocusIn`] event is not cancelable.
///
/// See: https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event.
#[derive(Clone, Copy, Debug, Default)]
pub struct FocusIn;

/// The [`FocusOut`] event fires when an element has lost focus, after the [`Blur`] event. The two
/// events differ in that [`FocusOut`] bubbles, while [`Blur`] does not.
///
/// The opposite of [`FocusOut`] is the [`FocusIn`] event, which fires when the element has received
/// focus. The [`FocusOut`] event is not cancelable.
///
/// See: https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event.
#[derive(Clone, Copy, Debug, Default)]
pub struct FocusOut;
