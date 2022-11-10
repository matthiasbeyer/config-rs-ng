# Vision

This document describes the vision for this crate. It is a work-in-progress
document and may be adapted over the development time of the codebase.

## config-rs

The effort to write this crate is made to revolutionize and evolve the
[config-rs] crate and solve some design issues with it as well as doing a
complete rework of its interface and API to streamline it and clean it up.

### Features

The [config-rs] crate is a crate that helps developers _read_ configuration. It
does not (yet?) offer functionality to _alter_ configuration more than
in-process, so there's no functionality for writing an adapted configuration
back to disc.

The crate offers a _layering_ technique, which can be used to "merge"
configuration files as "layers". For example, a system-wide configuration file
in `/etc/myapp.config` may define some system-wide configuration settings
whereas a user-specific configuration file `/home/alice/.myapp.config` defines
some user-specific configuration settings. These can be layered by the
[config-rs] crate when loading.
This way, a user-specific setting would override a system-wide one.

The [config-rs] crate does not define where these configuration files reside (a
crate such as `xdg` should be used for that).

The [config-rs] crate does offer functionality to layer environment variables as
well. In the above example, settings could be defined by the system-wide
configuration file as well as by the user-specific one, but the user is also
able to override configuration for a single invokation of the app in question by
defining environment variables.

The [config-rs] crate offers functionality to "watch" the configuration file(s)
for changes and update the configuration state of the app.

The [config-rs] crate offers several "backends", so that the user can freely
choose between TOML, YAML, JSON or others as configuration file language.

### Problems

The Author (@matthiasbeyer) identifies one major design issue in the [config-rs]
crate: The layering of the configuration is _eager_.

That means: As soon as a configuration file is read into memory, its contents
are parsed using the format specified by the developer and then it is layered
and merged with the other configuration objects that are already read. This
results in losing all context information. If a "further up" layer is changed
(by a configuration file change on disk) and some configuration value gets
"un-shadowed" by that, the crate cannot ensure proper function.

Example: Consider the following configurations:

`System-wide configuration`:

```toml
a = 1
b = 2
```

`User specific configuration`:

```toml
a = 2
```

The resulting configuration values after layering are `a = 2`, `b = 2`. If,
during the runtime of the application, the user changes the User-specific
configuration file and removes that setting, the crate cannot un-shadow the `a =
1` from the system configuration.

Another issue that arises from this, is that writing configuration values back
to file is not possible with eagerly layered values. The application de-facto
does not know where a configuration value comes from and thus cannot decide
where to write that value to. The developer may decide that all values to into
the most-specific configuration file. But that should be the decision of the
developer, not of the configuration crate.

<!-- TODO: more? -->

## Users

The config-rs-ng crate aims for the following users:

* Small CLI App developers. A developer that implements a "small CLI tool", a
  tool that might run for a few seconds and is non-interactive or has minimal
  interactive features (it does not have to be explicitely stopped by the user
  but runs to completion by itself). E.G.: `cat`, `toot`
* Interactive CLI/TUI App developers: A developer that implements a CLI or TUI
  that runs interactively and is a long-running process. The application
  interacts with the user and waits for their input and is exited if the user
  requests. E.G.: `less`, `vim`
* GUI App developers: A developer that implements a graphical application. E.G.:
  `libreoffice`, `ghostwriter`, `element` [^1]
* Server App developers: A server binary that is started by some process manager
  (`systemd`) and does interact with other software or Users via some remote
  functionality but not interactively on CLI [^1]

<!-- TODO: more? -->

## Hard Requirements

* Data sources
  * A developer MUST be able to define own configuration data sources
  * The crate MUST ship default configuration sources (From static string,
    from file, from environment) Layering of configuration values from
    different sources MUST be possible
  * The crate MUST support layering of configuration sources, that is,
    shadowning of configuration values by loading other configuration sources
    and effectively "layer" them over the previous loaded configuration values.
    * The crate MUST support "un-shadowing" configuration values if a "higher"
      layer gets changed (via user interaction, config source changes or else)
      and a value from a "lower" layer is no longer shadowed
  * The crate MUST support loading configuration data sources in an synchronous
    way as well as in an asynchronous way. The developer MUST be able to switch
    loading mechanisms without having to rewrite their configuration types
* The crate MUST allow deserializing the loaded configuration objects to a type
  that implements `serde::Deserialize` although layering-information and context
  is lost this way
* The developer of an app MUST be able to find where a specific value of
  configuration was defined
* Configuration updates
  * The developer of an app MUST be able to implement configuration updates
    while their app runs, thus the crate MUST implement interfaces where the
    developer can receive configuration updates.
  * The crate MUST be able to "un-shadow" (read above) configuration values if
    some configuration changes
  * Still, the developer of an app SHOULD NOT be forced to use this
    functionality.
* Configuration file formats
  * The crate SHOULD ship backends for the most common configuration file
    formats (TOML, JSON, YAML)
  * The crate SHOULD ship a backend for reading configuration from environment
    variables
  * A developer MUST be able to implement custom backends for the crate, to
    define how configuration sources are read
  * A developer SHOULD be able to strip down the crate and remove support for
    configuration file formats at compiletime to shrink the binary size of the
    crate
  * The crate MIGHT ship extended functionality, such as specific configuration
    file location helpers (via crates like [xdg] or [directories]), but always
    hidden behind compiletime feature flags that can be used to disable this
    functionality if desired.
* Configuration errors MUST point to the source of the error, that is, the
  source the error came from

## Soft Requirements

The crate SHOULD should enable simple functionality using simple APIs. That is:
loading a configuration, even layered, should involve only few lines of code
Still, the crate SHOULD support complex setups.

The Mantra should be "Make simple things simple, make complex things possible".

Mixing synchronous and asynchronous configuration sources is not a goal, but
could be made possible if it turns out that it is possible without much
complexity implementation-wise.

## Non-Goals

It is, at least for the initial developement phase, not a goal to be extra
resource sensitive. Convenience in use is a more important goal than saving
kilobytes of memory. That said, it might become a goal to be extra sensitive
about memory usage down the road.

It is also not an explicit goal for the initial development phase to be
especially _fast_ as in execution speed. Saving milliseconds is less important
than providing a good developer and/or user experience.



[^1]: The user of this app should be able to reconfigure the application without
  necessarily restarting it, thus the developer of this app might want to
  provide functionality to reconfigure the already-running application

[config-rs]: https://github.com/mehcode/config-rs
[xdg]: https://crates.io/crates/xdg
[directories]: https://crates.io/crates/directories
