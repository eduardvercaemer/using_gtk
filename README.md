# GTK applciations on rust

Creating simple GUI applications using GTK on rust.

_(Note: inspiration and structure taken from gnome's fractal, see the code [here](https://gitlab.gnome.org/GNOME/fractal/-/tree/master/fractal-gtk/src))_

# The App

We start by creating a GTK app in our main function, which we then
initialize via a startup callback, and then we just run it.
_(See [main.rs](./src/main.rs))_

#### Startup

The startup code simply initializes the _runtime_ we need to interact
with the app (more on this later), which is just setting up the app's
_state_ (including setting up the _ui_) and attaching a channel to the
main app context.

#### State

We have a _state_ struct defining all the context we care for in our
application, during the startup section from before, we have the main
context _own_ this state, we can then interact with it as expplained in
the next section.

#### Interacting with the App

To have any sort of interaction with our app, we use the created
_runtime_ from before, we can then use this to call _update_ functions
on our app, which work by getting a mutable reference to the app state
and doing any work we need to do (e.g. `state.main_menu()` changes the
current state to display the main menu).

#### UI

The UI part of our application holds a _builder_ (from a GTK builder
file) that we can use to get objects on demand (e.g. `state.about()`
uses the `about` object to create the window).

#### Actions

Here we define any type of actions we want to interact with on our
application, the global actions are instantiated when we first init
the app's _state_ when we set up the runtime.

One example is `main_menu`, which is an action attached to a button
or any other object (like a key-binding) that has a callback that
updates the runtime by calling the `state.main_menu()` function from
before.

# Summary

- We start an application _runtime_
- Initializes app state
- We use said runtime to run _update_ functions
- We use the _state_ to change ui elements or set up callbacks

Once we have the working template, most work is done
in the [_actions_](./src/actions/mod.rs) and the
[_state_](./src/state.mod.rs) modules (also check
builder.ui to see what the object references are).