# bevy_mod_clipboard

Basic clipboard support for Bevy.

## Features

* Simple API using the `Clipboard` resource.
* Asynchronous cross-platform clipboard access.
* Supports `windows`, `linux`, `macos` and `wasm32` targets.
* Send and fetch text to and from the clipboard to Bevy.

## Usage

To use, first add the `ClipboardPlugin` plugin to your Bevy app:

```rust
    app.add_plugins((DefaultPlugins, ClipboardPlugin));
```

Interact with the clipboard through the `Clipboard` resource.

To read from the clipboard call `Clipboard::fetch_text`. 
It returns a `ClipboardRead`:
* On windows and Unix-like platforms the result is ready immediately.
* On `wasm32`, reads are asynchronous — store the `ClipboardRead` and call `poll_result()` on subsequent frames until it returns Some.

 To write to the clipboard, call `Clipboard::set_text`.