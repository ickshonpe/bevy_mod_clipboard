//! Display text from the clipboard to a UI node
use bevy::color::palettes::css::NAVY;
use bevy::prelude::*;
use bevy_mod_clipboard::Clipboard;
use bevy_mod_clipboard::ClipboardPlugin;
use bevy_mod_clipboard::ClipboardRead;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ClipboardPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            ..Default::default()
        })
        .with_child((
            // Updated when the clipboard changes
            Text::new("No clipboard contents fetched"),
            BackgroundColor(NAVY.into()),
        ));
}

/// Fetches the current text contents of the clipboard each frame and updates
/// the text node if it has changed.
pub fn update(
    mut clipboard: ResMut<Clipboard>,
    mut maybe_read: Local<Option<ClipboardRead>>,
    mut text_query: Query<&mut Text>,
) {
    // If no clipboard read is pending, fetch any text
    if maybe_read.is_none() {
        // `fetch_text` completes instantly on windows and unix.
        // On wasm32 the result is fetched asynchronously, and the `ClipboardRead` needs to stored and polled
        // on following frames until a result is available.
        *maybe_read = Some(clipboard.fetch_text());
    }

    // Check if the clipboard read is complete and, if so, display its result.
    if let Some(read) = maybe_read.as_mut() {
        if let Some(contents) = read.poll_result() {
            let clipboard_contents = contents.unwrap_or_else(|e| format!("{e:?}"));
            text_query
                .single_mut()
                .unwrap()
                .set_if_neq(Text::new(clipboard_contents));
            *maybe_read = None;
        }
    }
}
