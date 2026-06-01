### Run Example with Patched Imgui Branch

Source: https://github.com/imgui-rs/imgui-rs/blob/main/docs/upgrading-imgui.md

Test the updated imgui-rs by running an example after patching it to refer to the new imgui branch. This helps verify the integration.

```sh
cargo run --example test_window_impl
```

--------------------------------

### igBeginChild Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for starting a child window.

```APIDOC
## igBeginChild Overloads

### Description
Starts a child window, which is an independent sub-window.

### Overloads
- `bool igBeginChild_Str(const char* str_id, const ImVec2 size, ImGuiChildFlags flags, ImGuiWindowFlags window_flags)`
- `bool igBeginChild_ID(ImGuiID id, const ImVec2 size, ImGuiChildFlags flags, ImGuiWindowFlags window_flags)`
```

--------------------------------

### Run imgui-examples

Source: https://github.com/imgui-rs/imgui-rs/blob/main/README.md

Clone the imgui-examples repository and run specific examples using Cargo.

```bash
git clone https://github.com/imgui-rs/imgui-examples
cd imgui-examples

# At the reposity root
cargo test

cargo run --example hello_world
cargo run --example test_window
cargo run --example test_window_impl
```

--------------------------------

### Setup UI Styles

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Configures the global UI style using `ctx.style_mut()`. This includes setting colors, rounding, spacing, and scaling for high-DPI displays.

```rust
use imgui::{StyleColor, StyleVar};

fn setup_style(ctx: &mut imgui::Context) {
    let style = ctx.style_mut();

    // Switch to built-in dark theme
    style.use_dark_colors();

    // Tweak specific colors
    style[StyleColor::WindowBg]   = [0.10, 0.10, 0.12, 1.00];
    style[StyleColor::TitleBgActive] = [0.18, 0.35, 0.58, 1.00];
    style[StyleColor::Button]     = [0.25, 0.50, 0.75, 0.70];

    // Adjust geometry
    style.window_rounding  = 6.0;
    style.frame_rounding   = 4.0;
    style.grab_rounding    = 4.0;
    style.item_spacing     = [8.0, 6.0];
    style.scale_all_sizes(1.25); // Scale for high-DPI displays
}
```

--------------------------------

### igBeginChild Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Starts a child window, which can be identified by a string ID or an ImGuiID, with options for size and flags.

```APIDOC
## igBeginChild Overloads

### Description
Begins a child window. Child windows are useful for splitting up a window into smaller sections.

### Overloads

1.  **`igBeginChild_Str(const char* str_id, const ImVec2 size, ImGuiChildFlags flags, ImGuiWindowFlags window_flags)`**: Begins a child window identified by a string ID.
2.  **`igBeginChild_ID(ImGuiID id, const ImVec2 size, ImGuiChildFlags flags, ImGuiWindowFlags window_flags)`**: Begins a child window identified by an `ImGuiID`.
```

--------------------------------

### imgui-rs Cargo.toml Setup

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Add imgui-rs and its platform and renderer backends to your Cargo.toml. Optional features like freetype, docking, and tables-api can be enabled.

```toml
[dependencies]
# High-level safe API
imgui = "0.12"

# Platform backend (winit)
imgui-winit-support = "0.12"

# Renderer (OpenGL via glow)
imgui-glow-renderer = "0.12"

# Enable optional features
# imgui = { version = "0.12", features = ["freetype", "docking", "tables-api"] }
```

--------------------------------

### Format code with rustfmt

Source: https://github.com/imgui-rs/imgui-rs/blob/main/README.md

Install the rustfmt component and format the project code to ensure style conformance.

```bash
rustup component add rustfmt
cargo fmt
```

--------------------------------

### igGetColorU32 Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for getting a color as an ImU32.

```APIDOC
## igGetColorU32 Overloads

### Description
Retrieves a color value as an `ImU32` (unsigned 32-bit integer).

### Overloads
- `ImU32 igGetColorU32_Col(ImGuiCol idx, float alpha_mul)`
- `ImU32 igGetColorU32_Vec4(const ImVec4 v)`
- `ImU32 igGetColorU32_U32(ImU32 val, float alpha_mul)`
```

--------------------------------

### ImSpan_begin Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for getting the beginning iterator of an ImSpan.

```APIDOC
## ImSpan_begin

### Description
Returns an iterator to the beginning of the span.

### Overloads
- `T* ImSpan_begin_Nil()`
- `const T* ImSpan_begin__const() const`
```

--------------------------------

### Implement Sliders and Drag Widgets in imgui-rs

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Provides examples for sliders and drag widgets, which allow for bounded or unbounded numeric editing. These widgets are generic and support custom formatting and step configurations.

```rust
fn render(ui: &imgui::Ui, state: &mut SliderState) {
    // Basic float slider 0..=1
    ui.slider("Volume", 0.0f32, 1.0, &mut state.volume);

    // Integer slider with custom format
    ui.slider_config("Level", 1i32, 100)
        .display_format("%d%%")
        .build(&mut state.level);

    // Drag float – drag to edit, no hard bounds by default unless clamped
    ui.drag_float("Mass", &mut state.mass)
        .speed(0.01)
        .range(0.0, f32::MAX)
        .display_format("%.3f kg")
        .build();

    // Drag int
    ui.drag_int("Iteration", &mut state.iterations)
        .speed(1.0)
        .range(1, 1000)
        .build();

    // Multi-component drag
    ui.drag_float3("Position", &mut state.position).build();
}

struct SliderState {
    volume: f32, level: i32, mass: f32, iterations: i32, position: [f32; 3],
}
```

--------------------------------

### ImVector_begin Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for getting the beginning iterator of an ImVector.

```APIDOC
## ImVector_begin

### Description
Returns an iterator to the beginning of the vector.

### Overloads
- `T* ImVector_begin_Nil()`
- `const T* ImVector_begin__const() const`
```

--------------------------------

### igGetForegroundDrawList Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for getting the foreground draw list.

```APIDOC
## igGetForegroundDrawList

### Description
Returns a pointer to the foreground draw list for rendering.

### Overloads
- `ImDrawList* igGetForegroundDrawList_ViewportPtr(ImGuiViewport* viewport)`
- `ImDrawList* igGetForegroundDrawList_WindowPtr(ImGuiWindow* window)`
```

--------------------------------

### Generate Rust Bindings with Cargo xtask

Source: https://github.com/imgui-rs/imgui-rs/blob/main/docs/upgrading-imgui.md

Execute the 'cargo xtask bindgen' command to generate the Rust bindings (e.g., imgui-sys/src/bindings.rs). This step requires bindgen to be installed.

```sh
cargo xtask bindgen
Finished dev [unoptimized + debuginfo] target(s) in 0.04s
Running `target/debug/xtask bindgen`
Executing bindgen [output = .../imgui-rs/imgui-sys/src/bindings.rs]
Success [output = .../imgui-rs/imgui-sys/src/bindings.rs]
Executing bindgen [output = .../imgui-rs/imgui-sys/src/wasm_bindings.rs]
Success [output = .../imgui-rs/imgui-sys/src/wasm_bindings.rs]
```

--------------------------------

### Create Child Windows for Layout

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Embeds self-contained scrolling and clipping regions within a parent window using child windows. This example shows a fixed-width left panel and a right panel that occupies the remaining space, useful for complex layouts.

```rust
fn render(ui: &imgui::Ui) {
    ui.window("Parent").size([500.0, 400.0], imgui::Condition::FirstUseEver).build(|| {
        // Left panel – fixed width child window
        ui.child_window("left_panel")
            .size([150.0, 0.0])   // 0 height = fill remaining
            .border(true)
            .build(|| {
                for i in 0..20 {
                    ui.text(format!("Item {:02}", i));
                }
            });

        ui.same_line();

        // Right panel – takes remaining space
        ui.child_window("right_panel")
            .size([0.0, 0.0])
            .build(|| {
                ui.text("Detail view here");
            });
    });
}
```

--------------------------------

### ImGuiWindow_GetID Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for getting an ID for an item within an ImGui window.

```APIDOC
## ImGuiWindow_GetID

### Description
Retrieves a unique identifier for an item within the current window context.

### Overloads
- `ImGuiID ImGuiWindow_GetID_Str(const char* str_id, const char* str_id_end)`
- `ImGuiID ImGuiWindow_GetID_Ptr(const void* ptr_id)`
- `ImGuiID ImGuiWindow_GetID_Int(int int_id)`
```

--------------------------------

### ImVector_end Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for getting the end iterator of an ImVector.

```APIDOC
## ImVector_end

### Description
Returns an iterator to the end of the vector.

### Overloads
- `T* ImVector_end_Nil()`
- `const T* ImVector_end__const() const`
```

--------------------------------

### ImSpan_end Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for getting the end iterator of an ImSpan.

```APIDOC
## ImSpan_end

### Description
Returns an iterator to the end of the span.

### Overloads
- `T* ImSpan_end_Nil()`
- `const T* ImSpan_end__const() const`
```

--------------------------------

### igTableGetColumnName

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Gets the name of a table column. Overloaded to accept a column index or a table pointer and index.

```APIDOC
## igTableGetColumnName_Int

### Description
Gets the name of a table column using its integer index. This assumes the table is currently active.

### Method
const char*

### Parameters
- **int**: The index of the column.
```

```APIDOC
## igTableGetColumnName_TablePtr

### Description
Gets the name of a table column from a specific table using its integer index.

### Method
const char*

### Parameters
- **const ImGuiTable***: Pointer to the table.
- **int**: The index of the column.
```

--------------------------------

### Create a basic 'Hello world' window in imgui-rs

Source: https://github.com/imgui-rs/imgui-rs/blob/main/README.md

This snippet demonstrates how to create a simple window with text, a separator, and display mouse coordinates. It requires the `imgui` crate and a suitable backend/renderer.

```rust
ui.window("Hello world")
    .size([300.0, 100.0], Condition::FirstUseEver)
    .build(|| {
        ui.text("Hello world!");
        ui.text("こんにちは世界！");
        ui.text("This...is...imgui-rs!");
        ui.separator();
        let mouse_pos = ui.io().mouse_pos;
        ui.text(format!(
            "Mouse Position: ({:.1},{:.1})",
            mouse_pos[0],
            mouse_pos[1]
        ));
    });
```

--------------------------------

### Build ImGui Window with Options

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Demonstrates building a window using the builder pattern with closure-based content. Configurable options include size, position, opened state, menu bar, and resizability. Content within the closure only executes if the window is visible.

```rust
use imgui::{Condition, WindowFlags};

fn render(ui: &imgui::Ui) {
    // Closure style — content only runs when the window is open/visible
    let mut opened = true;
    ui.window("Settings")
        .size([400.0, 300.0], Condition::FirstUseEver)
        .position([50.0, 50.0], Condition::Once)
        .opened(&mut opened)           // shows a close [x] button
        .menu_bar(true)
        .resizable(true)
        .collapsible(false)
        .build(|| {
            ui.text("Window is visible!");
            ui.separator();
            ui.text_colored([0.0, 1.0, 0.0, 1.0], "Green text");
        });

    // Token style — explicit end() call
    if let Some(_token) = ui
        .window("Overlay")
        .no_decoration()
        .no_inputs()
        .bg_alpha(0.35)
        .size([120.0, 40.0], Condition::Always)
        .position([10.0, 10.0], Condition::Always)
        .begin()
    {
        ui.text("FPS: 60");
        // _token dropped here → igEnd() called automatically
    }
}
```

--------------------------------

### Create ImGui Context and Main Loop

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Initializes the Dear ImGui context and demonstrates the per-frame loop structure. Ensure only one context is active; suspend others if necessary. The loop feeds input, begins a new frame, builds the UI, and renders.

```rust
use imgui::{Context, Condition};

fn main() {
    // Create context (panics if one already exists)
    let mut ctx = Context::create();

    // Disable auto-saving of window layout
    ctx.set_ini_filename(None);

    // --- Per-frame loop (driven by your windowing backend) ---
    loop {
        // Feed input from platform backend into ctx.io_mut() here ...

        // Begin frame
        let ui = ctx.new_frame();

        // Build UI
        ui.window("Hello")
            .size([300.0, 150.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Frame count:");
                ui.same_line();
                ui.text(format!("{}", ctx.frame_count()));
            });

        // End frame and obtain draw data for the renderer
        let draw_data = ctx.render();

        // Pass draw_data to your renderer (e.g. imgui-glow-renderer)
        // renderer.render(draw_data);

        break; // remove in a real loop
    }
}
```

--------------------------------

### Build Menu Bar and Menu Items

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Shows how to construct menu bars and individual menu items using `menu_bar`, `menu`, and `menu_item`. Supports shortcuts and enabling/disabling items.

```rust
fn render(ui: &imgui::Ui, state: &mut MenuState) {
    ui.window("Editor")
        .menu_bar(true)
        .size([600.0, 400.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.menu_bar(|| {
                ui.menu("File", || {
                    if ui.menu_item("New")  { state.new_file(); }
                    if ui.menu_item("Open") { state.open_file(); }
                    ui.separator();
                    if ui.menu_item_config("Save")
                        .shortcut("Ctrl+S")
                        .enabled(state.has_changes)
                        .build()
                    {
                        state.save_file();
                    }
                    ui.separator();
                    if ui.menu_item("Quit") { state.request_quit = true; }
                });
                ui.menu("View", || {
                    ui.menu_item_config("Show Grid")
                        .selected(&mut state.show_grid)
                        .build();
                });
            });

            ui.text("Editor content here");
        });
}

struct MenuState {
    has_changes: bool, show_grid: bool, request_quit: bool,
}
impl MenuState {
    fn new_file(&self) {} fn open_file(&self) {} fn save_file(&self) {}
}
```

--------------------------------

### Create Combo Box and List Box Widgets

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Demonstrates `combo_simple_string`, `begin_combo`, and `list_box` for creating selection widgets. Use `combo_simple_string` for basic dropdowns and `list_box` for fixed-height scrollable lists.

```rust
fn render(ui: &imgui::Ui, state: &mut SelectionState) {
    let items = ["Apple", "Banana", "Cherry", "Date", "Elderberry"];

    // Simple combo — preview is items[current]
    ui.combo_simple_string("Fruit", &mut state.combo_idx, &items);

    // Custom combo with arbitrary popup content
    if let Some(_token) = ui.begin_combo("Custom Combo", items[state.combo_idx]) {
        for (i, item) in items.iter().enumerate() {
            let selected = state.combo_idx == i;
            if ui.selectable_config(item).selected(selected).build() {
                state.combo_idx = i;
            }
            if selected {
                ui.set_item_default_focus();
            }
        }
    }

    // List box (shows height_in_items visible rows)
    ui.list_box("Pick one", &mut state.list_idx, &items, 4);
}

struct SelectionState { combo_idx: usize, list_idx: i32 }
```

--------------------------------

### Build and Test imgui-rs

Source: https://github.com/imgui-rs/imgui-rs/blob/main/docs/upgrading-imgui.md

After generating bindings, build the project with 'cargo build' and run tests with 'cargo test' to identify and fix any issues arising from upstream changes.

```sh
cargo build
```

```sh
cargo test
```

--------------------------------

### Configure Input Widgets in imgui-rs

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Illustrates the use of various input widgets like `input_text`, `input_float`, `input_int`, and `input_scalar`. These widgets support configuration via a builder pattern for hints, flags, and formatting.

```rust
fn render(ui: &imgui::Ui, state: &mut FormState) {
    // Single-line text field
    ui.input_text("Name", &mut state.name)
        .hint("Enter your name")
        .build();

    // Multi-line text editor
    ui.input_text_multiline("Notes", &mut state.notes, [0.0, 80.0])
        .build();

    // Float input with step buttons
    ui.input_float("Speed", &mut state.speed)
        .step(0.1)
        .step_fast(1.0)
        .display_format("%.2f")
        .build();

    // Integer input
    ui.input_int("Count", &mut state.count).build();

    // Generic scalar — works with u8, u16, f64, etc.
    ui.input_scalar("Threshold (u8)", &mut state.threshold).build();

    // Two-element float (position, UV, etc.)
    ui.input_float2("Offset", &mut state.offset).build();
}

struct FormState {
    name: String, notes: String, speed: f32,
    count: i32, threshold: u8, offset: [f32; 2],
}
```

--------------------------------

### ImSpan_begin Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Returns a pointer to the beginning of the ImSpan.

```APIDOC
## ImSpan_begin Overloads

### Description
Returns a pointer to the beginning of the `ImSpan`.

### Overloads

1. **`ImSpan_begin_Nil() -> T*`**: Returns a non-const pointer to the beginning.
2. **`ImSpan_begin__const() const -> const T*`**: Returns a const pointer to the beginning.
```

--------------------------------

### Render Various Text Widgets in imgui-rs

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Demonstrates the usage of different text rendering functions like `text`, `text_colored`, `text_disabled`, `text_wrapped`, `label_text`, and `bullet_text`. Also shows how to calculate text size using `calc_text_size`.

```rust
fn render(ui: &imgui::Ui) {
    // Plain text
    ui.text("Plain text");

    // Colored text [r, g, b, a] in 0..=1 range
    ui.text_colored([1.0, 0.4, 0.0, 1.0], "Orange warning!");

    // Greyed-out text
    ui.text_disabled("This feature is unavailable");

    // Wraps at the window width
    ui.text_wrapped("A very long string that will automatically wrap when it reaches the right edge of the current window or column.");

    // Label + value on same line (right-aligned label column)
    ui.label_text("Player", "Alice");

    // Bulleted list item
    ui.bullet_text("First point");
    ui.bullet_text("Second point");

    // Measure text before rendering
    let size = ui.calc_text_size("Hello");
    ui.text(format!("'Hello' is {:.1} x {:.1} px", size[0], size[1]));
}
```

--------------------------------

### ImGuiStoragePair Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating ImGuiStoragePair objects.

```APIDOC
## ImGuiStoragePair Overloads

### Description
Constructors for `ImGuiStoragePair` to store different data types.

### Overloads
- `ImGuiStoragePair_ImGuiStoragePair_Int(ImGuiID key, int val)`
- `ImGuiStoragePair_ImGuiStoragePair_Float(ImGuiID key, float val)`
- `ImGuiStoragePair_ImGuiStoragePair_Ptr(ImGuiID key, void* val)`
```

--------------------------------

### Publish Crates to Crates.io

Source: https://github.com/imgui-rs/imgui-rs/blob/main/docs/development-process.md

Use this command sequence to publish the imgui-rs crates to crates.io. Note that imgui-sys and imgui must be published before other dependent crates. The --no-verify flag is used for some crates to skip pre-publish checks.

```bash
cargo publish -p imgui-sys

```

```bash
cargo publish -p imgui

```

```bash
cargo publish -p imgui-winit-support --no-verify

```

```bash
cargo publish -p imgui-glium-renderer

```

```bash
cargo publish -p imgui-glow-renderer

```

```bash
cargo publish -p imgui-sdl2-support

```

```bash
cargo publish -p imgui-winit-glow-renderer-viewports

```

--------------------------------

### Render Popups and Modals

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Demonstrates how to open, render, and close standard popups and modal dialogs. Modals dim the background and block interaction.

```rust
fn render(ui: &imgui::Ui, state: &mut PopupState) {
    if ui.button("Open Popup") {
        ui.open_popup("my_popup");
    }

    ui.popup("my_popup", || {
        ui.text("Choose an option:");
        if ui.selectable("Option A") { state.choice = 0; }
        if ui.selectable("Option B") { state.choice = 1; }
        ui.separator();
        if ui.button("Cancel") { ui.close_current_popup(); }
    });

    // Modal dialog
    if ui.button("Delete?") {
        ui.open_popup("confirm_delete");
    }
    ui.modal_popup_config("confirm_delete")
        .resizable(false)
        .build(|| {
            ui.text("This action cannot be undone.");
            ui.separator();
            if ui.button_with_size("Confirm", [120.0, 0.0]) {
                state.delete_confirmed = true;
                ui.close_current_popup();
            }
            ui.same_line();
            if ui.button_with_size("Cancel", [120.0, 0.0]) {
                ui.close_current_popup();
            }
        });
}

struct PopupState { choice: i32, delete_confirmed: bool }
```

--------------------------------

### Handle Button and Checkbox Interactions in imgui-rs

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Shows how to use buttons, checkboxes, and radio buttons for user interaction. Buttons return `true` when clicked, and checkboxes/radio buttons modify boolean or integer states.

```rust
fn render(ui: &imgui::Ui, state: &mut AppState) {
    if ui.button("Click me") {
        state.count += 1;
    }
    ui.same_line();
    if ui.button_with_size("Big Button", [120.0, 40.0]) {
        println!("Big button clicked");
    }

    ui.separator();
    ui.checkbox("Enable shadows", &mut state.shadows);
    ui.checkbox("Vsync", &mut state.vsync);

    ui.separator();
    // Radio buttons bound to an enum/integer
    ui.radio_button("Low",    &mut state.quality, 0);
    ui.same_line();
    ui.radio_button("Medium", &mut state.quality, 1);
    ui.same_line();
    ui.radio_button("High",   &mut state.quality, 2);

    ui.separator();
    if ui.arrow_button("##left", imgui::Direction::Left) {
        state.index = state.index.saturating_sub(1);
    }
    ui.same_line();
    ui.text(format!("Index: {}", state.index));
    ui.same_line();
    if ui.arrow_button("##right", imgui::Direction::Right) {
        state.index += 1;
    }
}

struct AppState {
    count: u32, shadows: bool, vsync: bool, quality: i32, index: usize,
}
```

--------------------------------

### Implement Collapsible Headers and Tree Nodes

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Demonstrates `collapsing_header`, `tree_node`, and `tree_node_config` for creating collapsible sections and hierarchical views. Supports nesting and custom configurations like bullet points.

```rust
fn render(ui: &imgui::Ui) {
    // Collapsing header (always shown, toggles body)
    if ui.collapsing_header("Scene Objects", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        // Tree node (indented, disclosure arrow)
        if let Some(_node) = ui.tree_node("Root") {
            if let Some(_child) = ui.tree_node("Child A") {
                ui.text("Leaf content A");
            }
            if let Some(_child) = ui.tree_node_config("Child B")
                .selected(true)
                .open_on_arrow(true)
                .push()
            {
                ui.text("Leaf content B");
            }
        }
    }

    // Bullet-style tree
    ui.tree_node_config("Bullet Node")
        .bullet(true)
        .push()
        .map(|_n| ui.text("Bullet child"));
}
```

--------------------------------

### Create Tab Bars and Tab Items

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Illustrates the creation of tab bars and individual tab items using `tab_bar` and `tab_item`. Content within a tab is only rendered when the tab is active. Supports closable tabs.

```rust
fn render(ui: &imgui::Ui, state: &mut TabState) {
    if let Some(_bar) = ui.tab_bar("main_tabs") {
        if let Some(_tab) = ui.tab_item("Properties") {
            ui.text("Object properties here");
            ui.input_text("Name", &mut state.obj_name).build();
        }
        if let Some(_tab) = ui.tab_item("Materials") {
            ui.text("Material list here");
        }
        // Closable tab
        let mut open = state.log_open;
        if let Some(_tab) = ui.tab_item_button_with_close("Log", &mut open) {
            ui.text_wrapped(&state.log_output);
        }
        state.log_open = open;
    }
}

struct TabState { obj_name: String, log_open: bool, log_output: String }
```

--------------------------------

### igMenuItem Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Creates a menu item that can be selected, with options for a shortcut key and a boolean pointer to control its enabled state.

```APIDOC
## igMenuItem Overloads

### Description
Creates a menu item within a menu bar or popup menu.

### Overloads

1.  **`igMenuItem_Bool(const char* label, const char* shortcut, bool selected, bool enabled)`**: Creates a menu item with a label, optional shortcut, selection state, and enabled state.
2.  **`igMenuItem_BoolPtr(const char* label, const char* shortcut, bool* p_selected, bool enabled)`**: Creates a menu item where the selection state is controlled by a boolean pointer.
```

--------------------------------

### ImGuiStyleMod Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for creating ImGuiStyleMod objects.

```APIDOC
## ImGuiStyleMod_ImGuiStyleMod

### Description
Constructors for creating ImGuiStyleMod objects.

### Overloads
- `ImGuiStyleMod_ImGuiStyleMod_Int(ImGuiStyleVar idx, int val)`
- `ImGuiStyleMod_ImGuiStyleMod_Float(ImGuiStyleVar idx, float val)`
- `ImGuiStyleMod_ImGuiStyleMod_Vec2(ImGuiStyleVar idx, ImVec2 val)`
```

--------------------------------

### igListBox Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Creates a list box with different data source options.

```APIDOC
## igListBox

### Description
Creates a list box.

### Overloads
- `bool igListBox_Str_arr(const char*,int*,const char* const[],int,int)`
- `bool igListBox_FnStrPtr(const char*,int*,const char*(*)(void*,int),void*,int,int)`
```

--------------------------------

### Clone cimgui Repository

Source: https://github.com/imgui-rs/imgui-rs/blob/main/docs/upgrading-imgui.md

Clone the cimgui repository to generate C bindings for Dear ImGui. Ensure it's updated to a recent version, ideally corresponding to the latest Dear ImGui release tag.

```sh
git clone --recursive https://github.com/cimgui/cimgui.git /tmp/cimgui
```

--------------------------------

### igOpenPopup Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Opens a popup menu, identified by either a string ID or an ImGuiID.

```APIDOC
## igOpenPopup Overloads

### Description
Opens a popup menu.

### Overloads

1.  **`igOpenPopup_Str(const char* str_id, ImGuiPopupFlags flags)`**: Opens a popup identified by a string.
2.  **`igOpenPopup_ID(ImGuiID id, ImGuiPopupFlags flags)`**: Opens a popup identified by an `ImGuiID`.
```

--------------------------------

### igSetWindowSize

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Sets the size of a window. Overloaded for different ways to specify the window and size.

```APIDOC
## igSetWindowSize

### Description
Sets the size of a window.

### Method
void

### Parameters
- **size** (const ImVec2) - The desired size for the window.
- **cond** (ImGuiCond) - Condition for setting the size.

### Overloads
- `void igSetWindowSize_Vec2(const ImVec2 size, ImGuiCond cond)`
- `void igSetWindowSize_Str(const char* name, const ImVec2 size, ImGuiCond cond)`
- `void igSetWindowSize_WindowPtr(ImGuiWindow* window, const ImVec2 size, ImGuiCond cond)`
```

--------------------------------

### igSetWindowSize Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for setting the size of a window.

```APIDOC
## igSetWindowSize Overloads

### Description
Sets the size of a window.

### Overloads
- `void igSetWindowSize_Vec2(const ImVec2 size, ImGuiCond cond)`
- `void igSetWindowSize_Str(const char* name, const ImVec2 size, ImGuiCond cond)`
```

--------------------------------

### igMenuItem Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating menu items.

```APIDOC
## igMenuItem Overloads

### Description
Creates a menu item within a menu.

### Overloads
- `bool igMenuItem_Bool(const char* label, const char* shortcut, bool selected, bool enabled)`
- `bool igMenuItem_BoolPtr(const char* label, const char* shortcut, bool* p_selected, bool enabled)`
```

--------------------------------

### ImGuiTextRange Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Initializes an ImGuiTextRange, representing a range of text, either as an empty range or from two character pointers.

```APIDOC
## ImGuiTextRange Overloads

### Description
Initializes an `ImGuiTextRange` to define a segment of text.

### Overloads

1.  **`ImGuiTextRange_ImGuiTextRange_Nil()`**: Creates an empty text range.
2.  **`ImGuiTextRange_ImGuiTextRange_Str(const char* start, const char* end)`**: Creates a text range from start and end character pointers.
```

--------------------------------

### igCombo Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Creates a combo box (dropdown list) with options for displaying string arrays, a single string, or a function pointer for generating items.

```APIDOC
## igCombo Overloads

### Description
Creates a combo box widget, which is a dropdown list.

### Overloads

1.  **`igCombo_Str_arr(const char* label, int* current_item, const char* const items[], int items_count, int popup_height)`**: Creates a combo box from a null-terminated array of strings.
2.  **`igCombo_Str(const char* label, int* current_item, const char* items_separated_by_zeros, int popup_height)`**: Creates a combo box from a single string containing items separated by null characters.
3.  **`igCombo_FnStrPtr(const char* label, int* current_item, const char*(*getter)(void* data, int idx), void* data, int items_count, int popup_height)`**: Creates a combo box where items are generated by a callback function.
```

--------------------------------

### igOpenPopup Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for opening a popup.

```APIDOC
## igOpenPopup Overloads

### Description
Opens a modal or regular popup menu.

### Overloads
- `void igOpenPopup_Str(const char* str_id, ImGuiPopupFlags flags)`
- `void igOpenPopup_ID(ImGuiID id, ImGuiPopupFlags flags)`
```

--------------------------------

### igGetKeyData Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Retrieves data associated with an ImGui key.

```APIDOC
## igGetKeyData Overloads

### Description
Retrieves the `ImGuiKeyData` structure for a given key, providing information about its state.

### Overloads

1. **`igGetKeyData_ContextPtr(ImGuiContext* ctx, ImGuiKey key)`**: Gets key data from a specific `ImGuiContext`.
2. **`igGetKeyData_Key(ImGuiKey key)`**: Gets key data for the current `ImGuiContext`.
```

--------------------------------

### igCombo Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating a combo box.

```APIDOC
## igCombo Overloads

### Description
Creates a combo box (dropdown list).

### Overloads
- `bool igCombo_Str_arr(const char* label, int* current_item, const char* const items[], int items_count, int popup_height)`
- `bool igCombo_Str(const char* label, int* current_item, const char* items_separated_by_zeros, int popup_height)`
- `bool igCombo_FnStrPtr(const char* label, int* current_item, const char* (*getter)(void* data, int idx), void* data, int items_count, int popup_height)`
```

--------------------------------

### ImGuiTextRange Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating ImGuiTextRange objects.

```APIDOC
## ImGuiTextRange Overloads

### Description
Constructors for `ImGuiTextRange` to define a range of text.

### Overloads
- `ImGuiTextRange_ImGuiTextRange_Nil()`
- `ImGuiTextRange_ImGuiTextRange_Str(const char* str_start, const char* str_end)`
```

--------------------------------

### igMenuItem Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Creates a menu item with different boolean or pointer options.

```APIDOC
## igMenuItem

### Description
Creates a menu item.

### Overloads
- `bool igMenuItem_Bool(const char*,const char*,bool,bool)`
- `bool igMenuItem_BoolPtr(const char*,const char*,bool*,bool)`
```

--------------------------------

### igTreeNodeExV Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Variadic versions of igTreeNodeEx for advanced formatting.

```APIDOC
## igTreeNodeExV_Str

### Description
Creates a tree node with a string label, flags, and a format string with a va_list.

### Signature
bool igTreeNodeExV_Str(const char* label, ImGuiTreeNodeFlags flags, const char* fmt, va_list args)

### Parameters
- **label** (const char*) - The label for the tree node.
- **flags** (ImGuiTreeNodeFlags) - Flags to control the appearance and behavior of the tree node.
- **fmt** (const char*) - Format string for the node label.
- **args** (va_list) - Variable arguments list for formatting the label.
```

```APIDOC
## igTreeNodeExV_Ptr

### Description
Creates a tree node using a pointer as the identifier, with flags, a format string, and a va_list.

### Signature
bool igTreeNodeExV_Ptr(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, va_list args)

### Parameters
- **ptr_id** (const void*) - Pointer to be used as the node identifier.
- **flags** (ImGuiTreeNodeFlags) - Flags to control the appearance and behavior of the tree node.
- **fmt** (const char*) - Format string for the node label.
- **args** (va_list) - Variable arguments list for formatting the label.
```

--------------------------------

### ImGuiStoragePair Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Constructs an ImGuiStoragePair, which associates an ID with a value, supporting integer, float, or pointer types.

```APIDOC
## ImGuiStoragePair Overloads

### Description
Constructs an `ImGuiStoragePair` to store key-value data.

### Overloads

1.  **`ImGuiStoragePair_ImGuiStoragePair_Int(ImGuiID key, int val)`**: Creates a pair with an integer value.
2.  **`ImGuiStoragePair_ImGuiStoragePair_Float(ImGuiID key, float val)`**: Creates a pair with a float value.
3.  **`ImGuiStoragePair_ImGuiStoragePair_Ptr(ImGuiID key, void* val)`**: Creates a pair with a pointer value.
```

--------------------------------

### ImSpan Constructor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for constructing ImSpan objects.

```APIDOC
## ImSpan_ImSpan

### Description
Constructors for creating ImSpan objects.

### Overloads
- `ImSpan_ImSpan_Nil()`
- `ImSpan_ImSpan_TPtrInt(T* ptr, int count)`
- `ImSpan_ImSpan_TPtrTPtr(T* begin, T* end)`
```

--------------------------------

### igListBox Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating a list box.

```APIDOC
## igListBox Overloads

### Description
Creates a list box widget.

### Overloads
- `bool igListBox_Str_arr(const char* label, int* current_item, const char* const items[], int items_count, int height)`
- `bool igListBox_FnStrPtr(const char* label, int* current_item, const char* (*getter)(void* data, int idx), void* data, int items_count, int height)`
```

--------------------------------

### igSetWindowSize

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Sets the size of a window. Overloaded to accept a string ID, a vector, or a window pointer.

```APIDOC
## igSetWindowSize_Vec2

### Description
Sets the size of the current or focused window using a vector.

### Method
void

### Parameters
- **const ImVec2**: The desired size.
- **ImGuiCond**: The condition under which the size will be set.
```

```APIDOC
## igSetWindowSize_Str

### Description
Sets the size of a window identified by its string name.

### Method
void

### Parameters
- **const char***: The name of the window.
- **const ImVec2**: The desired size.
- **ImGuiCond**: The condition under which the size will be set.
```

```APIDOC
## igSetWindowSize_WindowPtr

### Description
Sets the size of a window using a direct pointer to the ImGuiWindow.

### Method
void

### Parameters
- **ImGuiWindow***: Pointer to the window.
- **const ImVec2**: The desired size.
- **ImGuiCond**: The condition under which the size will be set.
```

--------------------------------

### igTreeNodeExV

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Creates a tree node with extended flags and va_list arguments. Overloaded for string label or pointer.

```APIDOC
## igTreeNodeExV_Str

### Description
Creates a tree node with a string label, flags, and va_list for formatted content.

### Method
bool

### Parameters
- **const char***: The label for the tree node.
- **ImGuiTreeNodeFlags**: Flags to control the appearance and behavior of the node.
- **const char***: A format string for the node content.
- **va_list**: Variable arguments list for the format string.
```

```APIDOC
## igTreeNodeExV_Ptr

### Description
Creates a tree node using a pointer as an identifier, with flags and va_list for formatted content.

### Method
bool

### Parameters
- **const void***: A pointer to identify the node.
- **ImGuiTreeNodeFlags**: Flags to control the appearance and behavior of the node.
- **const char***: A format string for the node content.
- **va_list**: Variable arguments list for the format string.
```

--------------------------------

### Custom Drawing with DrawListMut

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Render arbitrary primitives like lines, rectangles, circles, and text directly onto the ImGui window, background, or foreground. Use `DrawListMut` obtained from `ui.get_window_draw_list()` or similar.

```rust
use imgui::{DrawListMut, ImColor32};

fn custom_draw(ui: &imgui::Ui) {
    let draw = ui.get_window_draw_list();

    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.2, 0.2, 1.0];
    const BLUE:  [f32; 4] = [0.2, 0.4, 1.0, 0.8];

    // Line
    draw.add_line([100.0, 100.0], [200.0, 150.0], WHITE)
        .thickness(2.0)
        .build();

    // Filled rectangle
    draw.add_rect([50.0, 200.0], [150.0, 260.0], RED)
        .filled(true)
        .rounding(6.0)
        .build();

    // Circle outline
    draw.add_circle([300.0, 180.0], 40.0, BLUE)
        .thickness(3.0)
        .num_segments(32)
        .build();

    // Polyline (open path)
    draw.add_polyline(
        &[[250.0, 100.0], [280.0, 130.0], [310.0, 110.0], [340.0, 140.0]],
        WHITE,
    )
    .thickness(1.5)
    .build();

    // Text at arbitrary position
    draw.add_text([100.0, 280.0], WHITE, "Custom label");
}
```

--------------------------------

### ImGuiStyleMod Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Applies modifications to ImGui style properties.

```APIDOC
## ImGuiStyleMod Overloads

### Description
Represents a modification to an `ImGuiStyle` property.

### Overloads

1. **`ImGuiStyleMod_ImGuiStyleMod_Int(ImGuiStyleVar idx, int val)`**: Modifies an integer style property.
2. **`ImGuiStyleMod_ImGuiStyleMod_Float(ImGuiStyleVar idx, float val)`**: Modifies a float style property.
3. **`ImGuiStyleMod_ImGuiStyleMod_Vec2(ImGuiStyleVar idx, ImVec2 val)`**: Modifies a `ImVec2` style property.
```

--------------------------------

### igOpenPopup Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Opens a popup identified by string or ID.

```APIDOC
## igOpenPopup

### Description
Opens a popup.

### Overloads
- `void igOpenPopup_Str(const char*,ImGuiPopupFlags)`
- `void igOpenPopup_ID(ImGuiID,ImGuiPopupFlags)`
```

--------------------------------

### igGetID Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for generating ImGui IDs.

```APIDOC
## igGetID Overloads

### Description
Generates a unique `ImGuiID` based on various inputs.

### Overloads
- `ImGuiID igGetID_Str(const char* str_id)`
- `ImGuiID igGetID_StrStr(const char* str_id_begin, const char* str_id_end)`
- `ImGuiID igGetID_Ptr(const void* ptr_id)`
- `ImGuiID igGetID_Int(int int_id)`
```

--------------------------------

### ImRect Constructor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for constructing ImRect objects.

```APIDOC
## ImRect_ImRect

### Description
Constructors for creating ImRect objects.

### Overloads
- `ImRect_ImRect_Nil()`
- `ImRect_ImRect_Vec2(const ImVec2 min, const ImVec2 max)`
- `ImRect_ImRect_Vec4(const ImVec4 v)`
- `ImRect_ImRect_Float(float x1, float y1, float x2, float y2)`
```

--------------------------------

### igImFloor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions for flooring numbers to the nearest integer.

```APIDOC
## igImFloor

### Description
Floors a number to the nearest integer less than or equal to it.

### Overloads
- `float igImFloor_Float(float)`
- `ImVec2 igImFloor_Vec2(const ImVec2)`
```

--------------------------------

### igTreeNodeExV Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Functions for creating tree nodes with flags and a va_list for formatted strings, suitable for variadic arguments.

```APIDOC
## igTreeNodeExV_Str

### Description
Creates a tree node using a string identifier, ImGuiTreeNodeFlags, and a va_list for formatted string content.

### Signature
bool igTreeNodeExV_Str(const char* str_id, ImGuiTreeNodeFlags flags, const char* fmt, va_list args)

## igTreeNodeExV_Ptr

### Description
Creates a tree node using a pointer identifier, ImGuiTreeNodeFlags, and a va_list for formatted string content.

### Signature
bool igTreeNodeExV_Ptr(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, va_list args)

```

--------------------------------

### igTreeNodeV

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Creates a tree node with va_list arguments. Overloaded for string label or pointer.

```APIDOC
## igTreeNodeV_Str

### Description
Creates a tree node with a string label and va_list for formatted content.

### Method
bool

### Parameters
- **const char***: The label for the tree node.
- **const char***: A format string for the node content.
- **va_list**: Variable arguments list for the format string.
```

```APIDOC
## igTreeNodeV_Ptr

### Description
Creates a tree node using a pointer as an identifier and va_list for formatted content.

### Method
bool

### Parameters
- **const void***: A pointer to identify the node.
- **const char***: A format string for the node content.
- **va_list**: Variable arguments list for the format string.
```

--------------------------------

### Display Plots and Histograms

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Visualize data using `plot_lines` for sparkline graphs and `plot_histogram` for bar charts. Both accept `&[f32]` slices and are configured using a builder pattern for graph size, scaling, and overlays.

```rust
use imgui::Ui;

fn render(ui: &Ui, state: &AppMetrics) {
    // Line graph
    ui.plot_lines("Frame Times", &state.frame_times)
        .graph_size([200.0, 60.0])
        .scale_min(0.0)
        .scale_max(33.0)
        .overlay_text("ms")
        .build();

    // Histogram
    ui.plot_histogram("Buckets", &state.histogram)
        .graph_size([200.0, 60.0])
        .scale_min(0.0)
        .overlay_text("distribution")
        .build();
}

struct AppMetrics {
    frame_times: Vec<f32>,
    histogram: Vec<f32>,
}
```

--------------------------------

### ImColor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating ImColor objects.

```APIDOC
## ImColor Overloads

### Description
Provides multiple ways to construct an `ImColor` object.

### Overloads
- `ImColor_ImColor_Nil()`
- `ImColor_ImColor_Float(float r, float g, float b, float a)`
- `ImColor_ImColor_Vec4(const ImVec4 v)`
- `ImColor_ImColor_Int(int r, int g, int b, int a)`
- `ImColor_ImColor_U32(ImU32 c)`
```

--------------------------------

### ImGuiPtrOrIndex Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for creating ImGuiPtrOrIndex objects.

```APIDOC
## ImGuiPtrOrIndex_ImGuiPtrOrIndex

### Description
Constructors for creating ImGuiPtrOrIndex objects.

### Overloads
- `ImGuiPtrOrIndex_ImGuiPtrOrIndex_Ptr(void* ptr)`
- `ImGuiPtrOrIndex_ImGuiPtrOrIndex_Int(int index)`
```

--------------------------------

### igCollapsingHeader Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating a collapsible header.

```APIDOC
## igCollapsingHeader Overloads

### Description
Creates a collapsible header that can be expanded or collapsed.

### Overloads
- `bool igCollapsingHeader_TreeNodeFlags(const char* label, ImGuiTreeNodeFlags flags)`
- `bool igCollapsingHeader_BoolPtr(const char* label, bool* p_open, ImGuiTreeNodeFlags flags)`
```

--------------------------------

### Update cimgui Output Scripts

Source: https://github.com/imgui-rs/imgui-rs/blob/main/docs/upgrading-imgui.md

Run the update-cimgui-output.sh script from within the imgui-sys/third-party/imgui-* directories to generate new C bindings. This script uses the cloned cimgui repository to update files like cimgui.cpp and definitions.json.

```sh
pwd
.../imgui-sys/third-party/imgui-master
./update-cimgui-output.sh /tmp/cimgui/
[...]
copyfile ./output/cimgui.h ../cimgui.h
copyfile ./output/cimgui.cpp ../cimgui.cpp
all done!!
cd ../imgui-docking/
./update-cimgui-output.sh /tmp/cimgui/
...
all done!!
```

--------------------------------

### Create a New Stable Branch

Source: https://github.com/imgui-rs/imgui-rs/blob/main/docs/development-process.md

This command creates a new branch named '0.9-stable' from the current branch, typically 'main', to prepare for a new release.

```bash
git switch -c 0.9-stable
```

--------------------------------

### igSetWindowFocus Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for setting the focus of a window.

```APIDOC
## igSetWindowFocus Overloads

### Description
Sets the focus of a window.

### Overloads
- `void igSetWindowFocus_Nil()`
- `void igSetWindowFocus_Str(const char* name)`
```

--------------------------------

### Render Tooltips

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Shows how to display custom content or plain text tooltips when an item is hovered. Use `is_item_hovered()` to guard tooltip visibility.

```rust
fn render(ui: &imgui::Ui) {
    ui.text("Hover over buttons for help");

    ui.button("Save");
    if ui.is_item_hovered() {
        ui.tooltip_text("Save the current file (Ctrl+S)");
    }

    ui.button("Advanced");
    if ui.is_item_hovered() {
        ui.tooltip(|| {
            ui.text_colored([1.0, 0.8, 0.0, 1.0], "Advanced Options");
            ui.separator();
            ui.text("These settings affect performance.");
            ui.text_disabled("Requires restart to apply.");
        });
    }
}
```

--------------------------------

### igPushStyleColor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for pushing a style color.

```APIDOC
## igPushStyleColor Overloads

### Description
Pushes a style color onto the style stack.

### Overloads
- `void igPushStyleColor_U32(ImGuiCol idx, ImU32 col)`
- `void igPushStyleColor_Vec4(ImGuiCol idx, const ImVec4 col)`
```

--------------------------------

### igTreeNodeV Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Variadic versions of igTreeNode for advanced formatting.

```APIDOC
## igTreeNodeV_Str

### Description
Creates a tree node with a string label, a format string, and a va_list.

### Signature
bool igTreeNodeV_Str(const char* str_id, const char* fmt, va_list args)

### Parameters
- **str_id** (const char*) - Unique identifier for the tree node.
- **fmt** (const char*) - Format string for the node label.
- **args** (va_list) - Variable arguments list for formatting the label.
```

```APIDOC
## igTreeNodeV_Ptr

### Description
Creates a tree node using a pointer as the identifier, a format string, and a va_list.

### Signature
bool igTreeNodeV_Ptr(const void* ptr_id, const char* fmt, va_list args)

### Parameters
- **ptr_id** (const void*) - Pointer to be used as the node identifier.
- **fmt** (const char*) - Format string for the node label.
- **args** (va_list) - Variable arguments list for formatting the label.
```

--------------------------------

### ImColor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Provides multiple ways to construct an ImColor object, accepting different parameter types like floats, ImVec4, integers, or a U32 value.

```APIDOC
## ImColor Overloads

### Description
Constructs an `ImColor` object using various input types.

### Overloads

1.  **`ImColor_ImColor_Nil()`**: Creates a default `ImColor`.
2.  **`ImColor_ImColor_Float(float r, float g, float b, float a)`**: Creates an `ImColor` from four float values (RGBA).
3.  **`ImColor_ImColor_Vec4(const ImVec4 v)`**: Creates an `ImColor` from an `ImVec4` struct.
4.  **`ImColor_ImColor_Int(int r, int g, int b, int a)`**: Creates an `ImColor` from four integer values (RGBA).
5.  **`ImColor_ImColor_U32(ImU32 color)`**: Creates an `ImColor` from a single `ImU32` value.
```

--------------------------------

### igGetIDWithSeed Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for generating a unique ImGui ID with a seed.

```APIDOC
## igGetIDWithSeed

### Description
Generates a unique identifier for an item with an additional seed value.

### Overloads
- `ImGuiID igGetIDWithSeed_Str(const char* str_id, const char* str_id_end, ImGuiID seed)`
- `ImGuiID igGetIDWithSeed_Int(int int_id, ImGuiID seed)`
```

--------------------------------

### ImVec2 Constructor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for constructing ImVec2 objects.

```APIDOC
## ImVec2_ImVec2

### Description
Constructors for creating ImVec2 objects.

### Overloads
- `ImVec2_ImVec2_Nil()`
- `ImVec2_ImVec2_Float(float x, float y)`
```

--------------------------------

### Suspend and Activate ImGui Contexts

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Use context suspension to manage multiple independent ImGui contexts, such as for different OS windows. Ensure proper activation and deactivation to avoid errors.

```rust
fn multi_context_example() {
    let ctx1 = imgui::Context::create();
    // ... configure ctx1 ...

    // Suspend ctx1 so a second context can be created
    let suspended1 = ctx1.suspend();

    let mut ctx2 = imgui::Context::create();
    // ... use ctx2 for its window's frame ...
    let suspended2 = ctx2.suspend();

    // Reactivate ctx1
    let ctx1 = match suspended1.activate() {
        Ok(ctx) => ctx,
        Err(_suspended) => panic!("Another context is still active"),
    };
    // ... render ctx1's frame ...
    drop(ctx1); // destroys context, no active context remains
    drop(suspended2);
}
```

--------------------------------

### igTreeNode Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating a tree node.

```APIDOC
## igTreeNode Overloads

### Description
Creates a tree node that can be expanded or collapsed.

### Overloads
- `bool igTreeNode_Str(const char* label)`
```

--------------------------------

### ImVec4 Constructor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for constructing ImVec4 objects.

```APIDOC
## ImVec4_ImVec4

### Description
Constructors for creating ImVec4 objects.

### Overloads
- `ImVec4_ImVec4_Nil()`
- `ImVec4_ImVec4_Float(float x, float y, float z, float w)`
```

--------------------------------

### Implement Drag and Drop

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Handle drag and drop operations by marking widgets as sources with `drag_drop_source_config` and defining drop targets with `drag_drop_target`. Payloads can be any `'static + Copy` type or raw bytes.

```rust
use imgui::{DragDropFlags, ImColor32};

fn render(ui: &imgui::Ui, state: &mut DndState) {
    for (i, item) in state.items.iter().enumerate() {
        ui.text(format!("Item {}: {}", i, item));

        // Register this item as a drag source
        if let Some(src) = ui
            .drag_drop_source_config("ITEM_PAYLOAD")
            .flags(DragDropFlags::SOURCE_NO_PREVIEW_TOOLTIP)
            .begin_payload(i as u32)
        {
            ui.text(format!("Dragging: {}", item));
            src.end(); // must end the source
        }

        // Register this slot as a drop target
        if let Some(target) = ui.drag_drop_target() {
            if let Some(payload) = target
                .accept_payload::<u32>("ITEM_PAYLOAD", DragDropFlags::empty())
            {
                let from = *payload.data as usize;
                state.items.swap(from, i);
            }
            target.pop(); // must end the target
        }
    }
}

struct DndState { items: Vec<String> }
```

--------------------------------

### Input Widgets

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Provides editable text fields, float inputs, and integer inputs. Supports configuration via a builder pattern for hints, flags, and callbacks. Includes multi-element variants for common vector types.

```APIDOC
## Input Widgets

`ui.input_text`, `ui.input_float`, `ui.input_int`, and `ui.input_scalar` provide editable fields. They use the builder pattern for optional configuration (hints, flags, callbacks). Multi-element variants (`input_float2`, `input_int3`, etc.) accept array or mint vector types.

```rust
fn render(ui: &imgui::Ui, state: &mut FormState) {
    // Single-line text field
    ui.input_text("Name", &mut state.name)
        .hint("Enter your name")
        .build();

    // Multi-line text editor
    ui.input_text_multiline("Notes", &mut state.notes, [0.0, 80.0])
        .build();

    // Float input with step buttons
    ui.input_float("Speed", &mut state.speed)
        .step(0.1)
        .step_fast(1.0)
        .display_format("%.2f")
        .build();

    // Integer input
    ui.input_int("Count", &mut state.count).build();

    // Generic scalar — works with u8, u16, f64, etc.
    ui.input_scalar("Threshold (u8)", &mut state.threshold).build();

    // Two-element float (position, UV, etc.)
    ui.input_float2("Offset", &mut state.offset).build();
}

struct FormState {
    name: String, notes: String, speed: f32,
    count: i32, threshold: u8, offset: [f32; 2],
}
```
```

--------------------------------

### igImLog Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Calculates the natural logarithm for float and double types.

```APIDOC
## igImLog

### Description
Calculates the natural logarithm of a number.

### Overloads
- `float igImLog_Float(float)`
- `double igImLog_double(double)`
```

--------------------------------

### igTreeNodeEx

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Creates a tree node with extended flags. Overloaded to accept a string label, a formatted string, or a pointer.

```APIDOC
## igTreeNodeEx_Str

### Description
Creates a tree node with a label and specific ImGuiTreeNodeFlags.

### Method
bool

### Parameters
- **const char***: The label for the tree node.
- **ImGuiTreeNodeFlags**: Flags to control the appearance and behavior of the node.
```

```APIDOC
## igTreeNodeEx_StrStr

### Description
Creates a tree node with a label, flags, and a formatted string.

### Method
bool

### Parameters
- **const char***: The label for the tree node.
- **ImGuiTreeNodeFlags**: Flags to control the appearance and behavior of the node.
- **const char***: A format string for the node content.
- **...**: Variable arguments for the format string.
```

```APIDOC
## igTreeNodeEx_Ptr

### Description
Creates a tree node using a pointer as an identifier, with flags and a formatted string.

### Method
bool

### Parameters
- **const void***: A pointer to identify the node.
- **ImGuiTreeNodeFlags**: Flags to control the appearance and behavior of the node.
- **const char***: A format string for the node content.
- **...**: Variable arguments for the format string.
```

--------------------------------

### igImFloor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Provides floor functions for float and ImVec2 types.

```APIDOC
## igImFloor

### Description
Calculates the floor of a value.

### Overloads
- `float igImFloor_Float(float)`
- `ImVec2 igImFloor_Vec2(const ImVec2)`
```

--------------------------------

### igImPow Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions for calculating exponentiation.

```APIDOC
## igImPow

### Description
Calculates a number raised to the power of another number.

### Overloads
- `float igImPow_Float(float, float)`
- `double igImPow_double(double, double)`
```

--------------------------------

### ImVec1 Constructor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for constructing ImVec1 objects.

```APIDOC
## ImVec1_ImVec1

### Description
Constructors for creating ImVec1 objects.

### Overloads
- `ImVec1_ImVec1_Nil()`
- `ImVec1_ImVec1_Float(float x)`
```

--------------------------------

### igSetWindowPos Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for setting the position of a window.

```APIDOC
## igSetWindowPos Overloads

### Description
Sets the position of a window.

### Overloads
- `void igSetWindowPos_Vec2(const ImVec2 pos, ImGuiCond cond)`
- `void igSetWindowPos_Str(const char* name, const ImVec2 pos, ImGuiCond cond)`
```

--------------------------------

### igItemSize Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions for setting the size of an item.

```APIDOC
## igItemSize

### Description
Sets the size of the current item. Used for layout calculations.

### Overloads
- `void igItemSize_Vec2(const ImVec2, float)`
- `void igItemSize_Rect(const ImRect, float)`
```

--------------------------------

### igMarkIniSettingsDirty Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to mark ini settings as dirty.

```APIDOC
## igMarkIniSettingsDirty

### Description
Marks the ImGui ini settings as dirty, indicating they need to be saved.

### Overloads
- `void igMarkIniSettingsDirty_Nil()`
- `void igMarkIniSettingsDirty_WindowPtr(ImGuiWindow*)`
```

--------------------------------

### igMarkIniSettingsDirty Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Marks ini settings as dirty, with options for global or window-specific.

```APIDOC
## igMarkIniSettingsDirty

### Description
Marks the ImGui ini settings as dirty, triggering a save.

### Overloads
- `void igMarkIniSettingsDirty_Nil()`
- `void igMarkIniSettingsDirty_WindowPtr(ImGuiWindow*)`
```

--------------------------------

### ImSpan_set Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for setting the contents of an ImSpan.

```APIDOC
## ImSpan_set

### Description
Sets the contents of the span.

### Overloads
- `void ImSpan_set_Int(T* ptr, int count)`
- `void ImSpan_set_TPtr(T* begin, T* end)`
```

--------------------------------

### igPushStyleColor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Pushes a style color, supporting U32 and ImVec4 types.

```APIDOC
## igPushStyleColor

### Description
Pushes a style color onto the style stack.

### Overloads
- `void igPushStyleColor_U32(ImGuiCol,ImU32)`
- `void igPushStyleColor_Vec4(ImGuiCol,const ImVec4)`
```

--------------------------------

### igGetID Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Generates a unique ImGuiID based on various input types like strings, pointers, or integers.

```APIDOC
## igGetID Overloads

### Description
Generates a unique identifier (`ImGuiID`) for various ImGui elements.

### Overloads

1.  **`igGetID_Str(const char* str_id)`**: Generates an ID from a string.
2.  **`igGetID_StrStr(const char* str_id, const char* str_id_end)`**: Generates an ID from a string range.
3.  **`igGetID_Ptr(const void* ptr_id)`**: Generates an ID from a pointer.
4.  **`igGetID_Int(int int_id)`**: Generates an ID from an integer.
```

--------------------------------

### igSetWindowPos

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Sets the position of a window. Overloaded to accept a window pointer directly.

```APIDOC
## igSetWindowPos_WindowPtr

### Description
Sets the position of a window using a direct pointer to the ImGuiWindow.

### Method
void

### Parameters
- **ImGuiWindow***: Pointer to the window.
- **const ImVec2**: The desired position.
- **ImGuiCond**: The condition under which the position will be set.
```

--------------------------------

### ImVector::begin Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for accessing the beginning iterator of an ImVector.

```APIDOC
## ImVector::begin Overloads

### Description
Provides an iterator to the beginning of the vector.

### Overloads
- `T* ImVector_begin_Nil()`
- `const T* ImVector_begin__const() const`
```

--------------------------------

### igTreeNodeEx

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Creates a tree node with extended flags. Overloaded for different ways to identify the node and provide a label.

```APIDOC
## igTreeNodeEx

### Description
Creates a tree node with extended flags.

### Method
bool

### Parameters
- **label** (const char*) - The label for the tree node.
- **flags** (ImGuiTreeNodeFlags) - Flags to control the appearance and behavior of the tree node.

### Overloads
- `bool igTreeNodeEx_Str(const char* label, ImGuiTreeNodeFlags flags)`
- `bool igTreeNodeEx_StrStr(const char* str_id, ImGuiTreeNodeFlags flags, const char* fmt, ...)`
- `bool igTreeNodeEx_Ptr(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, ...)`
```

--------------------------------

### igTreeNode

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Creates a tree node. Overloaded to accept a string label, a formatted string, or a pointer.

```APIDOC
## igTreeNode_Str

### Description
Creates a tree node with a simple string label.

### Method
bool

### Parameters
- **const char***: The label for the tree node.
```

```APIDOC
## igTreeNode_StrStr

### Description
Creates a tree node with a label and a formatted string.

### Method
bool

### Parameters
- **const char***: The label for the tree node.
- **const char***: A format string for the node content.
- **...**: Variable arguments for the format string.
```

```APIDOC
## igTreeNode_Ptr

### Description
Creates a tree node using a pointer as an identifier and a formatted string.

### Method
bool

### Parameters
- **const void***: A pointer to identify the node.
- **const char***: A format string for the node content.
- **...**: Variable arguments for the format string.
```

--------------------------------

### igPushStyleVar Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Pushes a style variable, supporting float and ImVec2 types.

```APIDOC
## igPushStyleVar

### Description
Pushes a style variable onto the style stack.

### Overloads
- `void igPushStyleVar_Float(ImGuiStyleVar,float)`
- `void igPushStyleVar_Vec2(ImGuiStyleVar,const ImVec2)`
```

--------------------------------

### igPushStyleVar Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for pushing a style variable.

```APIDOC
## igPushStyleVar Overloads

### Description
Pushes a style variable onto the style stack.

### Overloads
- `void igPushStyleVar_Float(ImGuiStyleVar idx, float val)`
- `void igPushStyleVar_Vec2(ImGuiStyleVar idx, const ImVec2 val)`
```

--------------------------------

### ImVec2 Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating ImVec2 objects.

```APIDOC
## ImVec2 Overloads

### Description
Constructors for `ImVec2` to represent 2D vectors.

### Overloads
- `ImVec2_ImVec2_Nil()`
- `ImVec2_ImVec2_Float(float x, float y)`
```

--------------------------------

### Buttons and Checkboxes

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Provides functionality for creating clickable buttons of various sizes and interactive checkboxes and radio buttons. Also includes an arrow button for directional input and a checkbox for toggling bitflags.

```APIDOC
## Buttons and Checkboxes

`ui.button`, `ui.small_button`, `ui.checkbox`, `ui.radio_button`, and `ui.arrow_button` return `true` when activated (clicked). `ui.checkbox_flags` toggles individual bitflags in a mask.

```rust
fn render(ui: &imgui::Ui, state: &mut AppState) {
    if ui.button("Click me") {
        state.count += 1;
    }
    ui.same_line();
    if ui.button_with_size("Big Button", [120.0, 40.0]) {
        println!("Big button clicked");
    }

    ui.separator();
    ui.checkbox("Enable shadows", &mut state.shadows);
    ui.checkbox("Vsync", &mut state.vsync);

    ui.separator();
    // Radio buttons bound to an enum/integer
    ui.radio_button("Low",    &mut state.quality, 0);
    ui.same_line();
    ui.radio_button("Medium", &mut state.quality, 1);
    ui.same_line();
    ui.radio_button("High",   &mut state.quality, 2);

    ui.separator();
    if ui.arrow_button("##left", imgui::Direction::Left) {
        state.index = state.index.saturating_sub(1);
    }
    ui.same_line();
    ui.text(format!("Index: {}", state.index));
    ui.same_line();
    if ui.arrow_button("##right", imgui::Direction::Right) {
        state.index += 1;
    }
}

struct AppState {
    count: u32, shadows: bool, vsync: bool, quality: i32, index: usize,
}
```
```

--------------------------------

### igListBox Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Creates a list box widget, allowing selection from a list of strings provided as an array or generated by a callback function.

```APIDOC
## igListBox Overloads

### Description
Creates a list box widget for selecting an item from a list.

### Overloads

1.  **`igListBox_Str_arr(const char* label, int* current_item, const char* const items[], int items_count, int height)`**: Creates a list box from a null-terminated array of strings.
2.  **`igListBox_FnStrPtr(const char* label, int* current_item, const char*(*getter)(void* data, int idx), void* data, int items_count, int height)`**: Creates a list box where items are generated by a callback function.
```

--------------------------------

### igSetItemKeyOwner Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Sets the key owner for an item, with options for nil or input flags.

```APIDOC
## igSetItemKeyOwner

### Description
Sets the key owner for the current item.

### Overloads
- `void igSetItemKeyOwner_Nil(ImGuiKey)`
- `void igSetItemKeyOwner_InputFlags(ImGuiKey,ImGuiInputFlags)`
```

--------------------------------

### igGetColorU32 Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Retrieves a color as an ImU32 value, supporting conversion from ImGuiCol, ImVec4, or another ImU32 with an alpha multiplier.

```APIDOC
## igGetColorU32 Overloads

### Description
Converts various color representations into an `ImU32` format.

### Overloads

1.  **`igGetColorU32_Col(ImGuiCol idx, float alpha_mul)`**: Gets the color associated with an `ImGuiCol` index, with an optional alpha multiplier.
2.  **`igGetColorU32_Vec4(const ImVec4 v)`**: Converts an `ImVec4` color to `ImU32`.
3.  **`igGetColorU32_U32(ImU32 color, float alpha_mul)`**: Applies an alpha multiplier to an existing `ImU32` color.
```

--------------------------------

### igTreeNodeV Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Functions for creating tree nodes with a va_list for formatted strings, suitable for variadic arguments.

```APIDOC
## igTreeNodeV_Str

### Description
Creates a tree node using a string identifier and a va_list for formatted string content.

### Signature
bool igTreeNodeV_Str(const char* str_id, const char* fmt, va_list args)

## igTreeNodeV_Ptr

### Description
Creates a tree node using a pointer identifier and a va_list for formatted string content.

### Signature
bool igTreeNodeV_Ptr(const void* ptr_id, const char* fmt, va_list args)

```

--------------------------------

### igSetItemKeyOwner Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions for setting the key owner for an item.

```APIDOC
## igSetItemKeyOwner

### Description
Sets the key owner for the current item, used for input handling.

### Overloads
- `void igSetItemKeyOwner_Nil(ImGuiKey)`
- `void igSetItemKeyOwner_InputFlags(ImGuiKey, ImGuiInputFlags)`
```

--------------------------------

### ImRect_Expand Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for expanding an ImRect.

```APIDOC
## ImRect_Expand

### Description
Expands the rectangle outwards by a given amount.

### Overloads
- `void ImRect_Expand_Float(const float amount)`
- `void ImRect_Expand_Vec2(const ImVec2 amount)`
```

--------------------------------

### igTabBarQueueFocus

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Queues focus for a tab bar. Overloaded to accept a tab item pointer or a string name.

```APIDOC
## igTabBarQueueFocus_TabItemPtr

### Description
Queues focus for a specific tab item within a tab bar using a pointer to the tab item.

### Method
void

### Parameters
- **ImGuiTabBar***: Pointer to the tab bar.
- **ImGuiTabItem***: Pointer to the tab item to focus.
```

```APIDOC
## igTabBarQueueFocus_Str

### Description
Queues focus for a specific tab item within a tab bar using its string name.

### Method
void

### Parameters
- **ImGuiTabBar***: Pointer to the tab bar.
- **const char***: The name of the tab item to focus.
```

--------------------------------

### igTreeNode

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Creates a tree node. Overloaded for different ways to identify the node and provide a label.

```APIDOC
## igTreeNode

### Description
Creates a tree node.

### Method
bool

### Parameters
- **label** (const char*) - The label for the tree node.

### Overloads
- `bool igTreeNode_Str(const char* label)`
- `bool igTreeNode_StrStr(const char* str_id, const char* fmt, ...)`
- `bool igTreeNode_Ptr(const void* ptr_id, const char* fmt, ...)`
```

--------------------------------

### igSelectable Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating selectable items.

```APIDOC
## igSelectable Overloads

### Description
Creates a selectable item that can be selected or deselected.

### Overloads
- `bool igSelectable_Bool(const char* label, bool selected, ImGuiSelectableFlags flags, const ImVec2 size)`
- `bool igSelectable_BoolPtr(const char* label, bool* p_selected, ImGuiSelectableFlags flags, const ImVec2 size)`
```

--------------------------------

### ImVec2ih Constructor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for constructing ImVec2ih objects.

```APIDOC
## ImVec2ih_ImVec2ih

### Description
Constructors for creating ImVec2ih objects.

### Overloads
- `ImVec2ih_ImVec2ih_Nil()`
- `ImVec2ih_ImVec2ih_short(short x, short y)`
- `ImVec2ih_ImVec2ih_Vec2(const ImVec2 v)`
```

--------------------------------

### igRadioButton Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating radio buttons.

```APIDOC
## igRadioButton Overloads

### Description
Creates a radio button.

### Overloads
- `bool igRadioButton_Bool(const char* label, bool active)`
- `bool igRadioButton_IntPtr(const char* label, int* v, int v_button)`
```

--------------------------------

### igPushStyleVar Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Pushes a style variable onto the ImGui style stack, allowing temporary modification of style properties like rounding or spacing. Supports float or ImVec2 values.

```APIDOC
## igPushStyleVar Overloads

### Description
Pushes a style variable onto the style stack, affecting subsequent UI elements.

### Overloads

1.  **`igPushStyleVar_Float(ImGuiStyleVar idx, float val)`**: Pushes a style variable with a float value.
2.  **`igPushStyleVar_Vec2(ImGuiStyleVar idx, const ImVec2 val)`**: Pushes a style variable with an `ImVec2` value.
```

--------------------------------

### ImVec4 Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating ImVec4 objects.

```APIDOC
## ImVec4 Overloads

### Description
Constructors for `ImVec4` to represent 4D vectors (often used for colors or rectangles).

### Overloads
- `ImVec4_ImVec4_Nil()`
- `ImVec4_ImVec4_Float(float x, float y, float z, float w)`
```

--------------------------------

### ImVector Constructor Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for constructing ImVector objects.

```APIDOC
## ImVector_ImVector

### Description
Constructors for creating ImVector objects.

### Overloads
- `ImVector_ImVector_Nil()`
- `ImVector_ImVector_Vector_T_(const ImVector_T& v)`
```

--------------------------------

### igCollapsingHeader Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Collapsing header widget that can be expanded or collapsed.

```APIDOC
## igCollapsingHeader Overloads

### Description
Creates a collapsing header widget that can be expanded or collapsed by the user.

### Overloads

1. **`igCollapsingHeader_TreeNodeFlags(const char* label, ImGuiTreeNodeFlags flags)`**: Creates a collapsing header with specified tree node flags.
2. **`igCollapsingHeader_BoolPtr(const char* label, bool* p_open, ImGuiTreeNodeFlags flags)`**: Creates a collapsing header with a boolean pointer to control its open state.
```

--------------------------------

### igSetWindowSize Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Sets the size of a window, identified by its string name or implicitly for the current window, using an ImVec2 for the new dimensions.

```APIDOC
## igSetWindowSize Overloads

### Description
Sets the size of a window.

### Overloads

1.  **`igSetWindowSize_Vec2(const ImVec2 size, ImGuiCond cond)`**: Sets the size for the current window.
2.  **`igSetWindowSize_Str(const char* name, const ImVec2 size, ImGuiCond cond)`**: Sets the size for a window identified by its name.
```

--------------------------------

### igItemSize Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Sets the size of the current item.

```APIDOC
## igItemSize

### Description
Sets the size of the current item.

### Overloads
- `void igItemSize_Vec2(const ImVec2,float)`
- `void igItemSize_Rect(const ImRect,float)`
```

--------------------------------

### igTreeNodeEx Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Extended functions for creating tree nodes with explicit flags.

```APIDOC
## igTreeNodeEx_Str

### Description
Creates a tree node with a string label and specified ImGuiTreeNodeFlags.

### Signature
bool igTreeNodeEx_Str(const char* label, ImGuiTreeNodeFlags flags)

### Parameters
- **label** (const char*) - The label for the tree node.
- **flags** (ImGuiTreeNodeFlags) - Flags to control the appearance and behavior of the tree node.
```

```APIDOC
## igTreeNodeEx_StrStr

### Description
Creates a tree node with a label derived from two strings, flags, and variable arguments.

### Signature
bool igTreeNodeEx_StrStr(const char* str_id, ImGuiTreeNodeFlags flags, const char* fmt, ...)

### Parameters
- **str_id** (const char*) - Unique identifier for the tree node.
- **flags** (ImGuiTreeNodeFlags) - Flags to control the appearance and behavior of the tree node.
- **fmt** (const char*) - Format string for the node label.
- **...** - Variable arguments for formatting the label.
```

```APIDOC
## igTreeNodeEx_Ptr

### Description
Creates a tree node using a pointer as the identifier, with flags and a formatted string label.

### Signature
bool igTreeNodeEx_Ptr(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, ...)

### Parameters
- **ptr_id** (const void*) - Pointer to be used as the node identifier.
- **flags** (ImGuiTreeNodeFlags) - Flags to control the appearance and behavior of the tree node.
- **fmt** (const char*) - Format string for the node label.
- **...** - Variable arguments for formatting the label.
```

--------------------------------

### igGetKeyData Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for retrieving data about a specific key.

```APIDOC
## igGetKeyData

### Description
Retrieves the data associated with a given ImGui key.

### Overloads
- `ImGuiKeyData* igGetKeyData_ContextPtr(ImGuiContext* ctx, ImGuiKey key)`
- `ImGuiKeyData* igGetKeyData_Key(ImGuiKey key)`
```

--------------------------------

### igSetWindowPos

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Sets the position of a window. Overloaded to accept a window pointer, or implicitly uses the current window.

```APIDOC
## igSetWindowPos

### Description
Sets the position of a window.

### Method
void

### Parameters
- **window** (ImGuiWindow*) - Pointer to the ImGuiWindow.
- **pos** (const ImVec2) - The desired position for the window.
- **cond** (ImGuiCond) - Condition for setting the position (e.g., ImGuiCond_Once, ImGuiCond_FirstUseEver, ImGuiCond_Appearing).

### Overloads
- `void igSetWindowPos_WindowPtr(ImGuiWindow* window, const ImVec2 pos, ImGuiCond cond)`
```

--------------------------------

### igImPow Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Calculates exponentiation for float and double types.

```APIDOC
## igImPow

### Description
Calculates a number raised to the power of another number.

### Overloads
- `float igImPow_Float(float,float)`
- `double igImPow_double(double,double)`
```

--------------------------------

### igTabItemCalcSize

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Calculates the size of a tab item. Overloaded to accept a string name or a window pointer.

```APIDOC
## igTabItemCalcSize_Str

### Description
Calculates the size required for a tab item identified by its string name.

### Method
ImVec2

### Parameters
- **const char***: The name of the tab item.
- **bool**: Whether the tab item is closable.
```

```APIDOC
## igTabItemCalcSize_WindowPtr

### Description
Calculates the size required for a tab item associated with a given window pointer.

### Method
ImVec2

### Parameters
- **ImGuiWindow***: Pointer to the window associated with the tab item.
```

--------------------------------

### igPushID Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Pushes an identifier onto the ImGui ID stack, allowing for unique identification of elements within nested scopes. Supports string, pointer, and integer inputs.

```APIDOC
## igPushID Overloads

### Description
Pushes an identifier onto the current ID stack.

### Overloads

1.  **`igPushID_Str(const char* str_id)`**: Pushes a string ID onto the stack.
2.  **`igPushID_StrStr(const char* str_id, const char* str_id_end)`**: Pushes a string range ID onto the stack.
3.  **`igPushID_Ptr(const void* ptr_id)`**: Pushes a pointer ID onto the stack.
4.  **`igPushID_Int(int int_id)`**: Pushes an integer ID onto the stack.
```

--------------------------------

### igTreeNode Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Creates a tree node that can be expanded or collapsed, identified by a string label.

```APIDOC
## igTreeNode Overloads

### Description
Creates a tree node widget that can be expanded or collapsed.

### Overloads

1.  **`igTreeNode_Str(const char* label)`**: Creates a tree node with a given label. Returns true if the node is open.
```

--------------------------------

### igTreeNodeEx Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Functions for creating tree nodes with flags, allowing for more control over their appearance and behavior.

```APIDOC
## igTreeNodeEx_Str

### Description
Creates a tree node using a string identifier and ImGuiTreeNodeFlags to control its appearance and behavior.

### Signature
bool igTreeNodeEx_Str(const char* str_id, ImGuiTreeNodeFlags flags)

## igTreeNodeEx_StrStr

### Description
Creates a tree node using a string identifier, ImGuiTreeNodeFlags, and a format string with variable arguments.

### Signature
bool igTreeNodeEx_StrStr(const char* str_id, ImGuiTreeNodeFlags flags, const char* fmt, ...)

## igTreeNodeEx_Ptr

### Description
Creates a tree node using a pointer identifier, ImGuiTreeNodeFlags, and a format string with variable arguments.

### Signature
bool igTreeNodeEx_Ptr(const void* ptr_id, ImGuiTreeNodeFlags flags, const char* fmt, ...)

```

--------------------------------

### igPushID Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Pushes an ID onto the stack, supporting string, pointer, and integer types.

```APIDOC
## igPushID

### Description
Pushes an identifier onto the current ID stack.

### Overloads
- `void igPushID_Str(const char*)`
- `void igPushID_StrStr(const char*,const char*)`
- `void igPushID_Ptr(const void*)`
- `void igPushID_Int(int)`
```

--------------------------------

### igImTrunc Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions for truncating numbers.

```APIDOC
## igImTrunc

### Description
Truncates a number towards zero.

### Overloads
- `float igImTrunc_Float(float)`
- `ImVec2 igImTrunc_Vec2(const ImVec2)`
```

--------------------------------

### igSelectable Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Creates a selectable item that can be clicked, often used in lists or menus. Supports controlling the selected state via a boolean or a boolean pointer.

```APIDOC
## igSelectable Overloads

### Description
Creates a selectable item that can be activated by the user.

### Overloads

1.  **`igSelectable_Bool(const char* label, bool selected, ImGuiSelectableFlags flags, const ImVec2 size)`**: Creates a selectable item with a label, selection state, flags, and optional size.
2.  **`igSelectable_BoolPtr(const char* label, bool* p_selected, ImGuiSelectableFlags flags, const ImVec2 size)`**: Creates a selectable item where the selection state is controlled by a boolean pointer.
```

--------------------------------

### igSetWindowPos Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Sets the position of a window, identified by its string name or implicitly for the current window, using an ImVec2 for the new position.

```APIDOC
## igSetWindowPos Overloads

### Description
Sets the position of a window.

### Overloads

1.  **`igSetWindowPos_Vec2(const ImVec2 pos, ImGuiCond cond)`**: Sets the position for the current window.
2.  **`igSetWindowPos_Str(const char* name, const ImVec2 pos, ImGuiCond cond)`**: Sets the position for a window identified by its name.
```

--------------------------------

### ImVector::resize Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for resizing an ImVector.

```APIDOC
## ImVector::resize Overloads

### Description
Changes the size of the vector.

### Overloads
- `void ImVector_resize_Nil(int new_size)`
- `void ImVector_resize_T(int new_size, const T value)`
```

--------------------------------

### igSetWindowFocus Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Sets the focus to a specific window, identified by its string name, or clears focus if no name is provided.

```APIDOC
## igSetWindowFocus Overloads

### Description
Sets the focus to a specific window.

### Overloads

1.  **`igSetWindowFocus_Nil()`**: Clears focus from the current window.
2.  **`igSetWindowFocus_Str(const char* name)`**: Sets focus to the window with the given name.
```

--------------------------------

### Disable Widgets

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Demonstrates how to temporarily disable widgets using closure or token-based scopes. Disabled widgets are greyed out and non-interactive.

```rust
fn render(ui: &imgui::Ui, state: &mut AppState) {
    // Closure form
    ui.disabled(!state.is_admin, || {
        if ui.button("Delete All Users") {
            state.delete_all();
        }
    });

    // Token form — scope ends when token drops
    let _d = ui.begin_enabled(state.network_available);
    if ui.button("Sync Now") {
        state.start_sync();
    }
    // _d drops here, re-enabling subsequent widgets
    drop(_d);

    if ui.button("Always Enabled") { /* ... */ }
}

struct AppState { is_admin: bool, network_available: bool }
impl AppState { fn delete_all(&self) {} fn start_sync(&self) {} }
```

--------------------------------

### igImRsqrt Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions for calculating the inverse square root.

```APIDOC
## igImRsqrt

### Description
Calculates the inverse square root (1/sqrt(x)) of a number.

### Overloads
- `float igImRsqrt_Float(float)`
- `double igImRsqrt_double(double)`
```

--------------------------------

### ImRect_Add Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for adding to an ImRect.

```APIDOC
## ImRect_Add

### Description
Expands the rectangle to include the given point or another rectangle.

### Overloads
- `void ImRect_Add_Vec2(const ImVec2 p)`
- `void ImRect_Add_Rect(const ImRect r)`
```

--------------------------------

### Text Widgets

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Renders various forms of text including plain, colored, disabled, wrapped, labeled, and bulleted text. Also includes functionality to calculate text size.

```APIDOC
## Text Widgets

imgui-rs exposes several text rendering functions on `Ui`: `text`, `text_colored`, `text_disabled`, `text_wrapped`, `label_text`, `bullet_text`, and `calc_text_size`. All accept any type implementing `AsRef<str>`.

```rust
fn render(ui: &imgui::Ui) {
    // Plain text
    ui.text("Plain text");

    // Colored text [r, g, b, a] in 0..=1 range
    ui.text_colored([1.0, 0.4, 0.0, 1.0], "Orange warning!");

    // Greyed-out text
    ui.text_disabled("This feature is unavailable");

    // Wraps at the window width
    ui.text_wrapped("A very long string that will automatically wrap when it reaches the right edge of the current window or column.");

    // Label + value on same line (right-aligned label column)
    ui.label_text("Player", "Alice");

    // Bulleted list item
    ui.bullet_text("First point");
    ui.bullet_text("Second point");

    // Measure text before rendering
    let size = ui.calc_text_size("Hello");
    ui.text(format!("'Hello' is {:.1} x {:.1} px", size[0], size[1]));
}
```
```

--------------------------------

### igRadioButton Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Creates a radio button, which can be used to select one option from a group. Supports controlling the selected state via a boolean or an integer pointer.

```APIDOC
## igRadioButton Overloads

### Description
Creates a radio button widget.

### Overloads

1.  **`igRadioButton_Bool(const char* label, bool active)`**: Creates a radio button with a label and a boolean indicating its active state.
2.  **`igRadioButton_IntPtr(const char* label, int* v, int v_off)`**: Creates a radio button where the selection state is managed by an integer pointer, and `v_off` is the value when unselected.
```

--------------------------------

### ImVector_resize Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for resizing an ImVector.

```APIDOC
## ImVector_resize

### Description
Resizes the vector to a specified size.

### Overloads
- `void ImVector_resize_Nil(int size)`
- `void ImVector_resize_T(int size, const T val)`
```

--------------------------------

### igPushID Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for pushing an ID onto the ID stack.

```APIDOC
## igPushID Overloads

### Description
Pushes an identifier onto the current ID stack.

### Overloads
- `void igPushID_Str(const char* str_id)`
- `void igPushID_StrStr(const char* str_id_begin, const char* str_id_end)`
- `void igPushID_Ptr(const void* ptr_id)`
- `void igPushID_Int(int int_id)`
```

--------------------------------

### ImVector_front Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for accessing the first element of an ImVector.

```APIDOC
## ImVector_front

### Description
Returns a reference to the first element in the vector.

### Overloads
- `T* ImVector_front_Nil()`
- `const T* ImVector_front__const() const`
```

--------------------------------

### Add Custom Fonts to ImGui Context

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Add default, TTF, or other font sources to the ImGui context before the first frame. The font atlas is rebuilt automatically on `new_frame()`.

```rust
use imgui::{FontSource, FontConfig, FontGlyphRanges};

fn setup_fonts(ctx: &mut imgui::Context) {
    let fonts = ctx.fonts();

    // Add the built-in default font at 13px
    fonts.add_font(&[FontSource::DefaultFontData {
        config: Some(FontConfig {
            size_pixels: 13.0,
            ..FontConfig::default()
        }),
    }]);

    // Add a TTF file at 18px
    let ttf_data = std::fs::read("assets/Roboto-Regular.ttf")
        .expect("font file not found");
    let _roboto = fonts.add_font(&[FontSource::TtfData {
        data: &ttf_data,
        size_pixels: 18.0,
        config: Some(FontConfig {
            rasterizer_multiply: 1.5,
            glyph_ranges: FontGlyphRanges::japanese(),
            ..FontConfig::default()
        }),
    }]);

    // The atlas texture must be rebuilt – happens automatically on new_frame()
}

fn render_with_font(ui: &imgui::Ui, big_font: imgui::FontId) {
    // Temporarily switch font
    let _font = ui.push_font(big_font);
    ui.text("Large text");
    // _font drops here → previous font restored
}
```

--------------------------------

### igPlotLines Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for plotting lines.

```APIDOC
## igPlotLines Overloads

### Description
Plots a series of connected line segments.

### Overloads
- `void igPlotLines_FloatPtr(const char* label, const float* values, int values_count, int offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size, int stride)`
- `void igPlotLines_FnFloatPtr(const char* label, float (*getter)(void* data, int idx), void* data, int values_count, int offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size)`
```

--------------------------------

### ImVector Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating ImVector objects.

```APIDOC
## ImVector Overloads

### Description
Constructors for `ImVector` (a dynamic array implementation).

### Overloads
- `ImVector_ImVector_Nil()`
- `ImVector_ImVector_Vector_T_(const ImVector_T other)`
```

--------------------------------

### igTreePush

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Pushes a new tree node onto the stack. Overloaded to accept a string label or a pointer.

```APIDOC
## igTreePush_Str

### Description
Pushes a new tree node onto the stack, identified by a string label.

### Method
void

### Parameters
- **const char***: The label for the tree node.
```

```APIDOC
## igTreePush_Ptr

### Description
Pushes a new tree node onto the stack, identified by a pointer.

### Method
void

### Parameters
- **const void***: A pointer to identify the tree node.
```

--------------------------------

### igImLengthSqr Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions for calculating the squared length of vectors.

```APIDOC
## igImLengthSqr

### Description
Calculates the squared length (magnitude squared) of a 2D or 4D vector.

### Overloads
- `float igImLengthSqr_Vec2(const ImVec2)`
- `float igImLengthSqr_Vec4(const ImVec4)`
```

--------------------------------

### igCheckboxFlags Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for creating a checkbox that controls flags.

```APIDOC
## igCheckboxFlags Overloads

### Description
Creates a checkbox that allows toggling specific bits within an integer or unsigned integer.

### Overloads
- `bool igCheckboxFlags_IntPtr(const char* label, int* flags, int flags_value)`
- `bool igCheckboxFlags_UintPtr(const char* label, unsigned int* flags, unsigned int flags_value)`
```

--------------------------------

### igIsKeyReleased Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a key was just released.

```APIDOC
## igIsKeyReleased

### Description
Checks if a specific key was released during the current frame.

### Overloads
- `bool igIsKeyReleased_Nil(ImGuiKey)`
- `bool igIsKeyReleased_ID(ImGuiKey, ImGuiID)`
```

--------------------------------

### igCheckboxFlags Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Checkbox widget that allows selection of multiple flags.

```APIDOC
## igCheckboxFlags Overloads

### Description
Creates a checkbox widget that allows toggling individual bits within an integer value.

### Overloads

1. **`igCheckboxFlags_IntPtr(const char* label, int* flags, int flags_value)`**: Checkbox for `int` flags.
2. **`igCheckboxFlags_UintPtr(const char* label, unsigned int* flags, unsigned int flags_value)`**: Checkbox for `unsigned int` flags.
3. **`igCheckboxFlags_S64Ptr(const char* label, ImS64* flags, ImS64 flags_value)`**: Checkbox for `ImS64` flags.
4. **`igCheckboxFlags_U64Ptr(const char* label, ImU64* flags, ImU64 flags_value)`**: Checkbox for `ImU64` flags.
```

--------------------------------

### ImVector::front Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for accessing the first element of an ImVector.

```APIDOC
## ImVector::front Overloads

### Description
Provides access to the first element of the vector.

### Overloads
- `T* ImVector_front_Nil()`
- `const T* ImVector_front__const() const`
```

--------------------------------

### igImSign Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Returns the sign of a number for float and double types.

```APIDOC
## igImSign

### Description
Returns the sign of a number (-1, 0, or 1).

### Overloads
- `float igImSign_Float(float)`
- `double igImSign_double(double)`
```

--------------------------------

### igTreeNode Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Functions for creating tree nodes with different identifier types and formatting options.

```APIDOC
## igTreeNode_StrStr

### Description
Creates a tree node using a string label and a format string with variable arguments. The node is identified by a string pointer.

### Signature
bool igTreeNode_StrStr(const char* str_id, const char* fmt, ...)

## igTreeNode_Ptr

### Description
Creates a tree node using a pointer as an identifier and a format string with variable arguments.

### Signature
bool igTreeNode_Ptr(const void* ptr_id, const char* fmt, ...)

```

--------------------------------

### igTreeNode Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Functions for creating tree nodes with string labels or pointer-based identifiers.

```APIDOC
## igTreeNode_StrStr

### Description
Creates a tree node with a label derived from two strings and variable arguments.

### Signature
bool igTreeNode_StrStr(const char* str_id, const char* fmt, ...)

### Parameters
- **str_id** (const char*) - Unique identifier for the tree node.
- **fmt** (const char*) - Format string for the node label.
- **...** - Variable arguments for formatting the label.
```

```APIDOC
## igTreeNode_Ptr

### Description
Creates a tree node using a pointer as the identifier and a formatted string label.

### Signature
bool igTreeNode_Ptr(const void* ptr_id, const char* fmt, ...)

### Parameters
- **ptr_id** (const void*) - Pointer to be used as the node identifier.
- **fmt** (const char*) - Format string for the node label.
- **...** - Variable arguments for formatting the label.
```

--------------------------------

### igSetWindowCollapsed Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for setting the collapsed state of a window.

```APIDOC
## igSetWindowCollapsed Overloads

### Description
Sets the collapsed state of a window.

### Overloads
- `void igSetWindowCollapsed_Bool(bool collapsed, ImGuiCond cond)`
- `void igSetWindowCollapsed_Str(const char* name, bool collapsed, ImGuiCond cond)`
```

--------------------------------

### igImLengthSqr Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Calculates the squared length of ImVec2 and ImVec4 vectors.

```APIDOC
## igImLengthSqr

### Description
Calculates the squared length of a vector.

### Overloads
- `float igImLengthSqr_Vec2(const ImVec2)`
- `float igImLengthSqr_Vec4(const ImVec4)`
```

--------------------------------

### igTreePush Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Functions to push a new level onto the tree stack.

```APIDOC
## igTreePush_Str

### Description
Pushes a new level onto the tree stack with a string identifier.

### Signature
void igTreePush_Str(const char* str_id)

### Parameters
- **str_id** (const char*) - The string identifier for the new tree level.
```

```APIDOC
## igTreePush_Ptr

### Description
Pushes a new level onto the tree stack using a pointer as the identifier.

### Signature
void igTreePush_Ptr(const void* ptr_id)

### Parameters
- **ptr_id** (const void*) - The pointer identifier for the new tree level.
```

--------------------------------

### ImDrawList_AddText Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for adding text to an ImDrawList.

```APIDOC
## ImDrawList_AddText Overloads

### Description
Adds text to the draw list with different parameter options.

### Overloads
- `ImDrawList_AddText_Vec2(const ImVec2 pos, ImU32 col, const char* text_start, const char* text_end)`
- `ImDrawList_AddText_FontPtr(const ImFont* font, float font_size, const ImVec2 pos, ImU32 col, const char* text_start, const char* text_end, float wrap_width, const ImVec4* cpu_fine_clip_rect)`
```

--------------------------------

### ImVec4 Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Constructs an ImVec4 vector, either with default values or from four float components.

```APIDOC
## ImVec4 Overloads

### Description
Constructs an `ImVec4` vector representing a 4D color or bounding box.

### Overloads

1.  **`ImVec4_ImVec4_Nil()`**: Creates an `ImVec4` with zero values (0, 0, 0, 0).
2.  **`ImVec4_ImVec4_Float(float x, float y, float z, float w)`**: Creates an `ImVec4` from four float components.
```

--------------------------------

### Slider and Drag Widgets

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Enables numeric input through sliders and drag widgets, supporting bounded and unbounded editing. These widgets are generic over numeric types and allow customization of speed, range, and display format.

```APIDOC
## Slider and Drag Widgets

Sliders (`ui.slider`) and drag widgets (`ui.drag_scalar`, `ui.drag_float`, etc.) allow bounded or unbounded numeric editing. Both families are generic over the numeric type via `DataTypeKind`.

```rust
fn render(ui: &imgui::Ui, state: &mut SliderState) {
    // Basic float slider 0..=1
    ui.slider("Volume", 0.0f32, 1.0, &mut state.volume);

    // Integer slider with custom format
    ui.slider_config("Level", 1i32, 100)
        .display_format("%d%%")
        .build(&mut state.level);

    // Drag float – drag to edit, no hard bounds by default unless clamped
    ui.drag_float("Mass", &mut state.mass)
        .speed(0.01)
        .range(0.0, f32::MAX)
        .display_format("%.3f kg")
        .build();

    // Drag int
    ui.drag_int("Iteration", &mut state.iterations)
        .speed(1.0)
        .range(1, 1000)
        .build();

    // Multi-component drag
    ui.drag_float3("Position", &mut state.position).build();
}

struct SliderState {
    volume: f32, level: i32, mass: f32, iterations: i32, position: [f32; 3],
}
```
```

--------------------------------

### igImLerp Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Performs linear interpolation between vectors and scalars.

```APIDOC
## igImLerp

### Description
Performs linear interpolation between two values.

### Overloads
- `ImVec2 igImLerp_Vec2Float(const ImVec2,const ImVec2,float)`
- `ImVec2 igImLerp_Vec2Vec2(const ImVec2,const ImVec2,const ImVec2)`
- `ImVec4 igImLerp_Vec4(const ImVec4,const ImVec4,float)`
```

--------------------------------

### igIsPopupOpen Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a popup is currently open.

```APIDOC
## igIsPopupOpen

### Description
Checks if a popup with the given identifier is currently open.

### Overloads
- `bool igIsPopupOpen_Str(const char*, ImGuiPopupFlags)`
- `bool igIsPopupOpen_ID(ImGuiID, ImGuiPopupFlags)`
```

--------------------------------

### igShortcut

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a shortcut is pressed. Overloaded to accept a key chord with or without an ID.

```APIDOC
## igShortcut_Nil

### Description
Checks if a shortcut is pressed without an associated ID.

### Method
bool

### Parameters
- **ImGuiKeyChord**: The key combination representing the shortcut.
- **ImGuiInputFlags**: Flags to modify input behavior.
```

```APIDOC
## igShortcut_ID

### Description
Checks if a shortcut is pressed with an associated ID.

### Method
bool

### Parameters
- **ImGuiKeyChord**: The key combination representing the shortcut.
- **ImGuiInputFlags**: Flags to modify input behavior.
- **ImGuiID**: The identifier for the shortcut.
```

--------------------------------

### ImSpan_set Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Sets the range of the ImSpan.

```APIDOC
## ImSpan_set Overloads

### Description
Sets the range of the `ImSpan`.

### Overloads

1. **`ImSpan_set_Int(T* ptr, int count)`**: Sets the span using a pointer and a count.
2. **`ImSpan_set_TPtr(T* begin, T* end)`**: Sets the span using begin and end pointers.
```

--------------------------------

### igCheckboxFlags Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for creating a checkbox that controls a set of flags.

```APIDOC
## igCheckboxFlags

### Description
Creates a checkbox that modifies a specific flag within a set of flags.

### Overloads
- `bool igCheckboxFlags_IntPtr(const char* label, int* flags, int flags_value)`
- `bool igCheckboxFlags_UintPtr(const char* label, unsigned int* flags, unsigned int flags_value)`
- `bool igCheckboxFlags_S64Ptr(const char* label, ImS64* flags, ImS64 flags_value)`
- `bool igCheckboxFlags_U64Ptr(const char* label, ImU64* flags, ImU64 flags_value)`
```

--------------------------------

### ImVec2 Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Constructs an ImVec2 vector, either with default values or from two float components.

```APIDOC
## ImVec2 Overloads

### Description
Constructs an `ImVec2` vector representing a 2D point or size.

### Overloads

1.  **`ImVec2_ImVec2_Nil()`**: Creates an `ImVec2` with zero values (0, 0).
2.  **`ImVec2_ImVec2_Float(float x, float y)`**: Creates an `ImVec2` from two float components.
```

--------------------------------

### igTreePush Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Functions for pushing a new tree level onto the tree stack.

```APIDOC
## igTreePush_Str

### Description
Pushes a new tree level onto the tree stack, identified by a string.

### Signature
void igTreePush_Str(const char* str_id)

## igTreePush_Ptr

### Description
Pushes a new tree level onto the tree stack, identified by a pointer.

### Signature
void igTreePush_Ptr(const void* ptr_id)

```

--------------------------------

### igImAbs Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for calculating the absolute value of an integer.

```APIDOC
## igImAbs

### Description
Calculates the absolute value of an integer.

### Overloads
- `int igImAbs_Int(int x)`
```

--------------------------------

### igImTrunc Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Truncates a float or ImVec2 to its integer part.

```APIDOC
## igImTrunc

### Description
Truncates a floating-point number towards zero.

### Overloads
- `float igImTrunc_Float(float)`
- `ImVec2 igImTrunc_Vec2(const ImVec2)`
```

--------------------------------

### igImAbs Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Provides absolute value functions for float and double types.

```APIDOC
## igImAbs

### Description
Calculates the absolute value of a number.

### Overloads
- `float igImAbs_Float(float)`
- `double igImAbs_double(double)`
```

--------------------------------

### igImRsqrt Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Calculates the reciprocal square root for float and double types.

```APIDOC
## igImRsqrt

### Description
Calculates the reciprocal square root of a number.

### Overloads
- `float igImRsqrt_Float(float)`
- `double igImRsqrt_double(double)`
```

--------------------------------

### igTableGcCompactTransientBuffers

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Compacts transient buffers for a table. Overloaded to accept a table pointer or a temporary data pointer.

```APIDOC
## igTableGcCompactTransientBuffers_TablePtr

### Description
Compacts transient buffers for a table using a pointer to the ImGuiTable.

### Method
void

### Parameters
- **ImGuiTable***: Pointer to the table.
```

```APIDOC
## igTableGcCompactTransientBuffers_TableTempDataPtr

### Description
Compacts transient buffers for a table using a pointer to its temporary data.

### Method
void

### Parameters
- **ImGuiTableTempData***: Pointer to the table's temporary data.
```

--------------------------------

### igPlotHistogram Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for plotting histograms.

```APIDOC
## igPlotHistogram Overloads

### Description
Plots a histogram.

### Overloads
- `void igPlotHistogram_FloatPtr(const char* label, const float* values, int values_count, int offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size, int stride)`
- `void igPlotHistogram_FnFloatPtr(const char* label, float (*getter)(void* data, int idx), void* data, int values_count, int offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size)`
```

--------------------------------

### igIsPopupOpen Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a popup is currently open, identified by string or ID.

```APIDOC
## igIsPopupOpen

### Description
Checks if a popup is currently open.

### Overloads
- `bool igIsPopupOpen_Str(const char*,ImGuiPopupFlags)`
- `bool igIsPopupOpen_ID(ImGuiID,ImGuiPopupFlags)`
```

--------------------------------

### igSetScrollFromPosX Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Sets the scroll position from the X coordinate.

```APIDOC
## igSetScrollFromPosX

### Description
Sets the scroll position from the X coordinate.

### Overloads
- `void igSetScrollFromPosX_Float(float,float)`
- `void igSetScrollFromPosX_WindowPtr(ImGuiWindow*,float,float)`
```

--------------------------------

### igCheckboxFlags Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Creates a checkbox that controls a set of flags within an integer or unsigned integer variable.

```APIDOC
## igCheckboxFlags Overloads

### Description
Creates a checkbox widget that allows toggling individual bits within a set of flags.

### Overloads

1.  **`igCheckboxFlags_IntPtr(const char* label, int* flags, int flags_value)`**: Creates a checkbox for an integer flag variable.
2.  **`igCheckboxFlags_UintPtr(const char* label, unsigned int* flags, unsigned int flags_value)`**: Creates a checkbox for an unsigned integer flag variable.
```

--------------------------------

### igGetID Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Generates a unique identifier for an item.

```APIDOC
## igGetID Overloads

### Description
Generates a unique `ImGuiID` for an item within the current context. This is useful for uniquely identifying widgets or elements.

### Overloads

1. **`igGetID_Str(const char* str_id)`**: Generates an ID from a string.
2. **`igGetID_StrStr(const char* str_id_begin, const char* str_id_end)`**: Generates an ID from a range of characters within a string.
3. **`igGetID_Ptr(const void* ptr_id)`**: Generates an ID from a pointer.
4. **`igGetID_Int(int int_id)`**: Generates an ID from an integer.
```

--------------------------------

### igIsKeyChordPressed Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a key chord is pressed.

```APIDOC
## igIsKeyChordPressed

### Description
Checks if a combination of keys (chord) is currently pressed.

### Overloads
- `bool igIsKeyChordPressed_Nil(ImGuiKeyChord)`
- `bool igIsKeyChordPressed_InputFlags(ImGuiKeyChord, ImGuiInputFlags, ImGuiID)`
```

--------------------------------

### ImVector::find Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for finding elements within an ImVector.

```APIDOC
## ImVector::find Overloads

### Description
Searches for an element within the vector.

### Overloads
- `T* ImVector_find_Nil(const T val)`
- `const T* ImVector_find__const(const T val) const`
```

--------------------------------

### igIsKeyDown Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a key is currently held down.

```APIDOC
## igIsKeyDown

### Description
Checks if a specific key is currently being held down.

### Overloads
- `bool igIsKeyDown_Nil(ImGuiKey)`
- `bool igIsKeyDown_ID(ImGuiKey, ImGuiID)`
```

--------------------------------

### ImDrawList AddText Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Allows adding text to an ImDrawList using either a string with a font pointer or a string with a position and color.

```APIDOC
## ImDrawList AddText Overloads

### Description
Adds text to the draw list with different signature options.

### Overloads

1.  **`ImDrawList_AddText_Vec2(const ImVec2 pos, ImU32 color, const char* text_begin, const char* text_end)`**: Adds text at a specific position with a given color.
2.  **`ImDrawList_AddText_FontPtr(const ImFont* font, float font_size, const ImVec2 pos, ImU32 color, const char* text_begin, const char* text_end, float wrap_width, const ImVec4* cpu_fine_clip_rect)`**: Adds text using a specific font and size, with additional parameters for clipping and wrapping.
```

--------------------------------

### igValue Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Functions for displaying different data types as values within the ImGui interface.

```APIDOC
## igValue_Bool

### Description
Displays a boolean value with a label.

### Signature
void igValue_Bool(const char* label, bool value)

## igValue_Int

### Description
Displays an integer value with a label.

### Signature
void igValue_Int(const char* label, int value)

## igValue_Uint

### Description
Displays an unsigned integer value with a label.

### Signature
void igValue_Uint(const char* label, unsigned int value)

## igValue_Float

### Description
Displays a float value with a label and an optional format string.

### Signature
void igValue_Float(const char* label, float value, const char* format)

```

--------------------------------

### ImVector Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Provides methods for managing dynamic vectors, including construction and accessing elements.

```APIDOC
## ImVector Overloads

### Description
Manages dynamic arrays (vectors) with various operations.

### Overloads

**Construction:**

1.  **`ImVector_ImVector_Nil()`**: Creates an empty `ImVector`.
2.  **`ImVector_ImVector_Vector_T_(const ImVector_T other)`**: Creates an `ImVector` by copying another `ImVector`.

**Element Access:**

1.  **`ImVector_back_Nil()`**: Returns a pointer to the last element.
2.  **`ImVector_back__const() const`**: Returns a const pointer to the last element.
3.  **`ImVector_begin_Nil()`**: Returns a pointer to the first element.
4.  **`ImVector_begin__const() const`**: Returns a const pointer to the first element.
5.  **`ImVector_end_Nil()`**: Returns a pointer to the element after the last element.
6.  **`ImVector_end__const() const`**: Returns a const pointer to the element after the last element.
7.  **`ImVector_find_Nil(const T value)`**: Returns a pointer to the first occurrence of a value.
8.  **`ImVector_find__const(const T value) const`**: Returns a const pointer to the first occurrence of a value.
9.  **`ImVector_front_Nil()`**: Returns a pointer to the first element.
10. **`ImVector_front__const() const`**: Returns a const pointer to the first element.

**Modification:**

1.  **`ImVector_erase_Nil(const T* value)`**: Removes the first occurrence of a value.
2.  **`ImVector_erase_TPtr(const T* start, const T* end)`**: Removes elements within a range.
3.  **`ImVector_resize_Nil(int size)`**: Resizes the vector to a specific size.
4.  **`ImVector_resize_T(int size, const T value)`**: Resizes the vector and fills new elements with a value.
```

--------------------------------

### igIsKeyDown Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a key is currently held down.

```APIDOC
## igIsKeyDown

### Description
Checks if a key is currently held down.

### Overloads
- `bool igIsKeyDown_Nil(ImGuiKey)`
- `bool igIsKeyDown_ID(ImGuiKey,ImGuiID)`
```

--------------------------------

### igSetScrollY Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Sets the vertical scroll position.

```APIDOC
## igSetScrollY

### Description
Sets the vertical scroll position.

### Overloads
- `void igSetScrollY_Float(float)`
- `void igSetScrollY_WindowPtr(ImGuiWindow*,float)`
```

--------------------------------

### igImIsPowerOfTwo Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a number is a power of two for integer and unsigned 64-bit types.

```APIDOC
## igImIsPowerOfTwo

### Description
Checks if the given integer is a power of two.

### Overloads
- `bool igImIsPowerOfTwo_Int(int)`
- `bool igImIsPowerOfTwo_U64(ImU64)`
```

--------------------------------

### igPlotLines Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Plots a line graph, either from a pre-allocated float array or by using a callback function to retrieve data points.

```APIDOC
## igPlotLines Overloads

### Description
Plots a line graph.

### Overloads

1.  **`igPlotLines_FloatPtr(const char* label, const float* values, int values_count, int offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size, int stride)`**: Plots a line graph from a float array.
2.  **`igPlotLines_FnFloatPtr(const char* label, float(*getter)(void* data, int idx), void* data, int values_count, int offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size)`**: Plots a line graph using a callback function to get data points.
```

--------------------------------

### igIsKeyChordPressed Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a key chord is pressed with optional input flags.

```APIDOC
## igIsKeyChordPressed

### Description
Checks if a key chord is pressed.

### Overloads
- `bool igIsKeyChordPressed_Nil(ImGuiKeyChord)`
- `bool igIsKeyChordPressed_InputFlags(ImGuiKeyChord,ImGuiInputFlags,ImGuiID)`
```

--------------------------------

### igImIsPowerOfTwo Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a number is a power of two.

```APIDOC
## igImIsPowerOfTwo

### Description
Checks if a given integer or unsigned 64-bit integer is a power of two.

### Overloads
- `bool igImIsPowerOfTwo_Int(int)`
- `bool igImIsPowerOfTwo_U64(ImU64)`
```

--------------------------------

### igSetScrollX Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Sets the horizontal scroll position.

```APIDOC
## igSetScrollX

### Description
Sets the horizontal scroll position.

### Overloads
- `void igSetScrollX_Float(float)`
- `void igSetScrollX_WindowPtr(ImGuiWindow*,float)`
```

--------------------------------

### igIsKeyPressed Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a key was just pressed.

```APIDOC
## igIsKeyPressed

### Description
Checks if a specific key was pressed during the current frame.

### Overloads
- `bool igIsKeyPressed_Bool(ImGuiKey, bool)`
- `bool igIsKeyPressed_InputFlags(ImGuiKey, ImGuiInputFlags, ImGuiID)`
```

--------------------------------

### ImVector_find Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for finding an element in an ImVector.

```APIDOC
## ImVector_find

### Description
Finds the first occurrence of an element in the vector.

### Overloads
- `T* ImVector_find_Nil(const T val)`
- `const T* ImVector_find__const(const T val) const`
```

--------------------------------

### ImVector_back Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Returns a reference to the last element in the ImVector.

```APIDOC
## ImVector_back Overloads

### Description
Returns a reference to the last element in the `ImVector`.

### Overloads

1. **`ImVector_back_Nil() -> T*`**: Returns a non-const reference to the last element.
2. **`ImVector_back__const() const -> const T*`**: Returns a const reference to the last element.
```

--------------------------------

### igShortcut

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Checks if a keyboard shortcut is pressed. Overloaded to use an implicit ID or an explicit ID.

```APIDOC
## igShortcut

### Description
Checks if a keyboard shortcut is pressed.

### Method
bool

### Parameters
- **key_chord** (ImGuiKeyChord) - The combination of keys representing the shortcut.
- **flags** (ImGuiInputFlags) - Flags to modify input behavior.

### Overloads
- `bool igShortcut_Nil(ImGuiKeyChord key_chord, ImGuiInputFlags flags)`
- `bool igShortcut_ID(ImGuiKeyChord key_chord, ImGuiInputFlags flags, ImGuiID id)`
```

--------------------------------

### igIsRectVisible Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for checking if a rectangle is visible.

```APIDOC
## igIsRectVisible Overloads

### Description
Checks if a given rectangle is within the visible clipping region.

### Overloads
- `bool igIsRectVisible_Nil(const ImVec2 min, const ImVec2 max)`
- `bool igIsRectVisible_Vec2(const ImVec2 rect_min, const ImVec2 rect_max)`
```

--------------------------------

### igPlotHistogram Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Plots a histogram, either from a pre-allocated float array or by using a callback function to retrieve data points.

```APIDOC
## igPlotHistogram Overloads

### Description
Plots a histogram graph.

### Overloads

1.  **`igPlotHistogram_FloatPtr(const char* label, const float* values, int values_count, int offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size, int stride)`**: Plots a histogram from a float array.
2.  **`igPlotHistogram_FnFloatPtr(const char* label, float(*getter)(void* data, int idx), void* data, int values_count, int offset, const char* overlay_text, float scale_min, float scale_max, ImVec2 graph_size)`**: Plots a histogram using a callback function to get data points.
```

--------------------------------

### igIsKeyReleased Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a key was released in the current frame.

```APIDOC
## igIsKeyReleased

### Description
Checks if a key was released in the current frame.

### Overloads
- `bool igIsKeyReleased_Nil(ImGuiKey)`
- `bool igIsKeyReleased_ID(ImGuiKey,ImGuiID)`
```

--------------------------------

### ImRect_Add Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Adds to the dimensions of an ImRect.

```APIDOC
## ImRect_Add Overloads

### Description
Expands the `ImRect` by adding a `ImVec2` or another `ImRect`.

### Overloads

1. **`ImRect_Add_Vec2(const ImVec2 rhs)`**: Adds a `ImVec2` to the current `ImRect`.
2. **`ImRect_Add_Rect(const ImRect rhs)`**: Adds another `ImRect` to the current `ImRect`.
```

--------------------------------

### igIsMouseDown Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a mouse button is currently held down.

```APIDOC
## igIsMouseDown

### Description
Checks if a specific mouse button is currently being held down.

### Overloads
- `bool igIsMouseDown_Nil(ImGuiMouseButton)`
- `bool igIsMouseDown_ID(ImGuiMouseButton, ImGuiID)`
```

--------------------------------

### igIsMouseClicked Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a mouse button was just clicked.

```APIDOC
## igIsMouseClicked

### Description
Checks if a specific mouse button was clicked during the current frame.

### Overloads
- `bool igIsMouseClicked_Bool(ImGuiMouseButton, bool)`
- `bool igIsMouseClicked_InputFlags(ImGuiMouseButton, ImGuiInputFlags, ImGuiID)`
```

--------------------------------

### igValue

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Outputs a value to the log. Overloaded for different data types.

```APIDOC
## igValue

### Description
Outputs a value to the log.

### Method
void

### Parameters
- **label** (const char*) - The label for the value.

### Overloads
- `void igValue_Bool(const char* label, bool value)`
- `void igValue_Int(const char* label, int value)`
- `void igValue_Uint(const char* label, unsigned int value)`
- `void igValue_Float(const char* label, float value, const char* float_format)`
```

--------------------------------

### ImPool_Remove Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Removes an item from an ImGui pool.

```APIDOC
## ImPool_Remove Overloads

### Description
Removes an element from a generic pool.

### Overloads

1. **`ImPool_Remove_TPtr(ImGuiID id, const T* ptr)`**: Removes an element by its pointer.
2. **`ImPool_Remove_PoolIdx(ImGuiID id, ImPoolIdx idx)`**: Removes an element by its pool index.
```

--------------------------------

### igValue

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Outputs a value to the log. Overloaded for boolean, integer, unsigned integer, and float types.

```APIDOC
## igValue_Bool

### Description
Outputs a boolean value to the log.

### Method
void

### Parameters
- **const char***: The label for the value.
- **bool**: The boolean value to log.
```

```APIDOC
## igValue_Int

### Description
Outputs an integer value to the log.

### Method
void

### Parameters
- **const char***: The label for the value.
- **int**: The integer value to log.
```

```APIDOC
## igValue_Uint

### Description
Outputs an unsigned integer value to the log.

### Method
void

### Parameters
- **const char***: The label for the value.
- **unsigned int**: The unsigned integer value to log.
```

```APIDOC
## igValue_Float

### Description
Outputs a float value to the log with a specified format.

### Method
void

### Parameters
- **const char***: The label for the value.
- **float**: The float value to log.
- **const char***: The format string for the float value.
```

--------------------------------

### igIsMouseReleased Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking/overloads.txt

Provides overloaded functions to check if a mouse button was just released.

```APIDOC
## igIsMouseReleased

### Description
Checks if a specific mouse button was released during the current frame.

### Overloads
- `bool igIsMouseReleased_Nil(ImGuiMouseButton)`
- `bool igIsMouseReleased_ID(ImGuiMouseButton, ImGuiID)`
```

--------------------------------

### ImVector::back Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for accessing the last element of an ImVector.

```APIDOC
## ImVector::back Overloads

### Description
Provides access to the last element of the vector.

### Overloads
- `T* ImVector_back_Nil()`
- `const T* ImVector_back__const() const`
```

--------------------------------

### igIsRectVisible Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Checks if a given rectangle is within the visible clipping area of the current viewport, using either a single ImVec2 for size or two ImVec2 for position and size.

```APIDOC
## igIsRectVisible Overloads

### Description
Checks if a rectangle is within the visible clipping area.

### Overloads

1.  **`igIsRectVisible_Nil(const ImVec2 size)`**: Checks if a rectangle of a given size (origin at 0,0) is visible.
2.  **`igIsRectVisible_Vec2(const ImVec2 rect_min, const ImVec2 rect_max)`**: Checks if the rectangle defined by two corner points is visible.
```

--------------------------------

### ImVector_back Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for accessing the last element of an ImVector.

```APIDOC
## ImVector_back

### Description
Returns a reference to the last element in the vector.

### Overloads
- `T* ImVector_back_Nil()`
- `const T* ImVector_back__const() const`
```

--------------------------------

### igSetScrollFromPosY Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Sets the scroll position from the Y coordinate.

```APIDOC
## igSetScrollFromPosY

### Description
Sets the scroll position from the Y coordinate.

### Overloads
- `void igSetScrollFromPosY_Float(float,float)`
- `void igSetScrollFromPosY_WindowPtr(ImGuiWindow*,float,float)`
```

--------------------------------

### igIsMouseDown Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a mouse button is currently held down.

```APIDOC
## igIsMouseDown

### Description
Checks if a mouse button is currently held down.

### Overloads
- `bool igIsMouseDown_Nil(ImGuiMouseButton)`
- `bool igIsMouseDown_ID(ImGuiMouseButton,ImGuiID)`
```

--------------------------------

### ImPool_Remove Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for removing an item from an ImPool.

```APIDOC
## ImPool_Remove

### Description
Removes an item from a pool based on its type or index.

### Overloads
- `void ImPool_Remove_TPtr(ImGuiID key, const T* ptr)`
- `void ImPool_Remove_PoolIdx(ImGuiID key, ImPoolIdx idx)`
```

--------------------------------

### igValue Functions

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master-freetype/overloads.txt

Functions to display values of different types with a label.

```APIDOC
## igValue_Bool

### Description
Displays a boolean value with a label.

### Signature
void igValue_Bool(const char* label, bool value)

### Parameters
- **label** (const char*) - The label for the value.
- **value** (bool) - The boolean value to display.
```

```APIDOC
## igValue_Int

### Description
Displays an integer value with a label.

### Signature
void igValue_Int(const char* label, int value)

### Parameters
- **label** (const char*) - The label for the value.
- **value** (int) - The integer value to display.
```

```APIDOC
## igValue_Uint

### Description
Displays an unsigned integer value with a label.

### Signature
void igValue_Uint(const char* label, unsigned int value)

### Parameters
- **label** (const char*) - The label for the value.
- **value** (unsigned int) - The unsigned integer value to display.
```

```APIDOC
## igValue_Float

### Description
Displays a float value with a label and a format string.

### Signature
void igValue_Float(const char* label, float value, const char* float_format)

### Parameters
- **label** (const char*) - The label for the value.
- **value** (float) - The float value to display.
- **float_format** (const char*) - The format string for the float value.
```

--------------------------------

### igIsKeyPressed Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a key was pressed in the current frame.

```APIDOC
## igIsKeyPressed

### Description
Checks if a key was pressed in the current frame.

### Overloads
- `bool igIsKeyPressed_Bool(ImGuiKey,bool)`
- `bool igIsKeyPressed_InputFlags(ImGuiKey,ImGuiInputFlags,ImGuiID)`
```

--------------------------------

### ImVector::erase Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for erasing elements from an ImVector.

```APIDOC
## ImVector::erase Overloads

### Description
Removes elements from the vector.

### Overloads
- `T* ImVector_erase_Nil(const T* it)`
- `T* ImVector_erase_TPtr(const T* first, const T* last)`
```

--------------------------------

### igIsMouseClicked Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a mouse button was clicked in the current frame.

```APIDOC
## igIsMouseClicked

### Description
Checks if a mouse button was clicked in the current frame.

### Overloads
- `bool igIsMouseClicked_Bool(ImGuiMouseButton,bool)`
- `bool igIsMouseClicked_InputFlags(ImGuiMouseButton,ImGuiInputFlags,ImGuiID)`
```

--------------------------------

### Scoped Style Overrides

Source: https://context7.com/imgui-rs/imgui-rs/llms.txt

Applies temporary style changes for a specific frame using `push_style_color` and `push_style_var`. These changes revert automatically when the scope ends.

```rust
use imgui::{StyleColor, StyleVar};

fn render_with_custom_style(ui: &imgui::Ui) {
    // Scoped color override (reverts automatically when token drops)
    let _c = ui.push_style_color(StyleColor::Button, [0.8, 0.2, 0.2, 1.0]);
    let _v = ui.push_style_var(StyleVar::FrameRounding(12.0));
    ui.button("Red Rounded Button");
    // _c and _v drop here, restoring original style
}
```

--------------------------------

### igIsRectVisible Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a rectangle is within the visible clipping region.

```APIDOC
## igIsRectVisible

### Description
Checks if a rectangle is within the visible clipping region.

### Overloads
- `bool igIsRectVisible_Nil(const ImVec2)`
- `bool igIsRectVisible_Vec2(const ImVec2,const ImVec2)`
```

--------------------------------

### ImVector::end Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-master/overloads.txt

Overloads for accessing the end iterator of an ImVector.

```APIDOC
## ImVector::end Overloads

### Description
Provides an iterator to the end of the vector.

### Overloads
- `T* ImVector_end_Nil()`
- `const T* ImVector_end__const() const`
```

--------------------------------

### ImVector_erase Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for erasing elements from an ImVector.

```APIDOC
## ImVector_erase

### Description
Removes elements from the vector.

### Overloads
- `T* ImVector_erase_Nil(const T* it)`
- `T* ImVector_erase_TPtr(const T* first, const T* last)`
```

--------------------------------

### igIsMouseDoubleClicked Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a mouse button was double-clicked in the current frame.

```APIDOC
## igIsMouseDoubleClicked

### Description
Checks if a mouse button was double-clicked in the current frame.

### Overloads
- `bool igIsMouseDoubleClicked_Nil(ImGuiMouseButton)`
- `bool igIsMouseDoubleClicked_ID(ImGuiMouseButton,ImGuiID)`
```

--------------------------------

### ImRect_Contains Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Overloads for checking if an ImRect contains a point or another rectangle.

```APIDOC
## ImRect_Contains

### Description
Checks if the rectangle contains a given point or another rectangle.

### Overloads
- `bool ImRect_Contains_Vec2(const ImVec2 p) const`
- `bool ImRect_Contains_Rect(const ImRect r) const`
```

--------------------------------

### igIsMouseReleased Overloads

Source: https://github.com/imgui-rs/imgui-rs/blob/main/imgui-sys/third-party/imgui-docking-freetype/overloads.txt

Checks if a mouse button was released in the current frame.

```APIDOC
## igIsMouseReleased

### Description
Checks if a mouse button was released in the current frame.

### Overloads
- `bool igIsMouseReleased_Nil(ImGuiMouseButton)`
- `bool igIsMouseReleased_ID(ImGuiMouseButton,ImGuiID)`
```