# glade

A comprehensive Dioxus component library with CSS modules via [stylance](https://crates.io/crates/stylance).

## Components

### Form Controls
- **Button** - Primary, Secondary, Danger, Ghost variants with sizes
- **Input** - Text input with label and error states
- **Textarea** - Multi-line text input
- **Checkbox** - Checkbox with label
- **Radio** - Radio buttons and RadioGroup
- **Select** - Dropdown select
- **Slider** - Range input
- **Toggle** - On/off toggle switch
- **TagInput** - Tag input with removable chips
- **SegmentedInput** - OTP/TOTP code input
- **Rating** - Star rating input
- **FileInput** - File input with drag-and-drop
- **CopyInput** - Input with copy-to-clipboard button

### Display
- **Alert** - Info, success, warning, error banners
- **Badge** - Status pills and labels
- **Avatar** - User images with AvatarGroup
- **Card** - Container with header, content, footer
- **CodeBlock** - Code display with language icons
- **Toast** - Snackbar notifications
- **Tooltip** - Hover info popups
- **Popover** - Click-triggered popups
- **Progress** - Progress bars and circular progress
- **Spinner** - Loading indicators
- **Skeleton** - Loading placeholders
- **TimeAgo** - Relative time display

### Layout
- **Stack** - Vertical flex layout
- **Row** - Horizontal flex layout
- **Grid** - CSS grid layout
- **Container** - Centered max-width container
- **Section** - Content organization
- **SidebarLayout** - Sidebar + main layout
- **SplitPane** - Resizable panels

### Navigation
- **Navbar** - Navigation bar
- **Sidebar** - Side navigation
- **Tabs** - Tab panels
- **Breadcrumb** - Breadcrumb navigation
- **Pagination** - Page navigation
- **CommandPalette** - Search and keyboard navigation

### Overlays
- **Modal** - Dialog with header, body, footer
- **Drawer** - Slide-over panel
- **AlertDialog** - Confirmation dialog
- **ContextMenu** - Right-click menu
- **Dropdown** - Dropdown menu
- **HoverCard** - Rich content on hover

### Chat/Messaging
- **ChatBubble** - Message bubbles
- **MessageList** - Message container
- **MessageComposer** - Message input
- **ThreadList** - Conversation list

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
glade = { git = "https://github.com/bearcove/glade" }
```

Import and use components:

```rust
use glade::{Button, ButtonVariant, Input, Card, CardHeader, CardContent};

rsx! {
    Card {
        CardHeader { "Login" }
        CardContent {
            Input { label: "Email" }
            Button { variant: ButtonVariant::Primary, "Submit" }
        }
    }
}
```

Include the CSS in your app:

```rust
use glade::{GLADE_BASE_CSS, GLADE_STYLANCE_CSS};

rsx! {
    document::Link { rel: "stylesheet", href: GLADE_BASE_CSS }
    document::Link { rel: "stylesheet", href: GLADE_STYLANCE_CSS }
    // your components...
}
```

## Features

- `web` - Enables WASM-specific functionality (clipboard, timers, etc.)
- `buck2` - Disables asset!() macro for Buck2 builds

## Styling

Components use [stylance](https://crates.io/crates/stylance) for CSS modules. Run the stylance CLI watcher alongside your dev server:

```bash
stylance --watch .
```

This watches `.module.scss` files and generates `css/stylance.scss`.

## Demo

See all components in action:

```bash
# Install tools
cargo install dioxus-cli stylance-cli

# Terminal 1: Watch CSS
stylance --watch .

# Terminal 2: Run demo
cd demo && dx serve
```

Then open http://localhost:8080

## License

MIT OR Apache-2.0
