# glade Demo

A demo application showcasing all glade components.

## Prerequisites

Install the required tools:

```bash
# Dioxus CLI
cargo install dioxus-cli

# Stylance CLI (for CSS module compilation)
cargo install stylance-cli
```

## Running the Demo

You need two terminals - one for the stylance CSS watcher and one for the Dioxus dev server.

**Terminal 1 - CSS watcher:**
```bash
cd /path/to/glade
stylance --watch .
```

This watches all `.module.scss` files and regenerates `css/stylance.scss`.

**Terminal 2 - Dev server:**
```bash
cd /path/to/glade/demo
dx serve
```

Then open http://localhost:8080 in your browser.

## Structure

The demo organizes components by category:

- **Primitives** - Button, Avatar, Badge, Card, Icons
- **Form** - Input, Select, Checkbox, Radio, Toggle, Slider, etc.
- **Loading** - Progress, Spinner
- **Feedback** - Alert, Tooltip, Toast, EmptyState
- **Navigation** - Tabs, Modal, Drawer, Dropdown, Accordion, etc.
- **Layout** - Separator, Carousel, SplitPane, ScrollArea
- **Chat** - ChatBubble, MessageList, MessageComposer, ThreadList
- **Data** - CodeBlock, Table, Stats, DiffStats, etc.
- **Misc** - TodoList, ColorSwatch, PulsingDots, Footer
