# Copilot Instructions for ITA Dashboard

## Project Overview
**ita_dashboard** is a Rust-based GUI dashboard application using the **Iced** framework. It provides a menu-driven interface with multiple views (Dashboard, Reports, Graph) and implements a modular component architecture.

**Key Dependencies:**
- `iced` (0.14) - GPU-accelerated cross-platform GUI framework with canvas support
- `tokio` (async runtime, used to pull data from database)

## Architecture

### Core Structure
- **`src/main.rs`** - Main application entry point using Iced's `Application` trait
  - `MainApp` struct manages view state with selected `MenuItem`
  - Message-driven update pattern: `Message::MenuSelected` updates selected view
  - Layout: Two-column UI (150px fixed left menu + responsive right content pane)
  - Theme: TokyoNightStorm

- **`src/graph.rs`** - Canvas-based graph component
  - `Graph` struct implements `canvas::Program<Message>` trait
  - Renders line graphs from f32 data vectors
  - Custom axis rendering with proper scaling (Y-axis: 40px left margin, X-axis: 40px bottom margin)
  - Auto-scales to maximum value; empty data returns blank frame

### Data Flow
1. User clicks menu button → `Message::MenuSelected(MenuItem)` emitted
2. `update()` changes `selected` state
3. `view()` renders appropriate content based on selected `MenuItem`
4. Graph view passes hardcoded test data: `vec![1.0, 3.0, 2.0, 5.0, 4.0]`

## Key Patterns & Conventions

### Message-Driven Updates (Iced Pattern)
All state changes flow through `Message` enums. Add new user interactions:
```rust
// In main.rs
pub enum Message {
    MenuSelected(MenuItem),
    // Add: GraphDataUpdated(Vec<f32>)
}

// In update()
Message::GraphDataUpdated(data) => {
    // Update state
}
```

### Component Integration
Components are stateless view functions returning `Element<Message>`:
```rust
pub fn view<Message>(data: Vec<f32>) -> canvas::Canvas<Graph, Message>
```
This allows reuse and loose coupling. All callbacks bubble up through Message types.

### UI Layout Pattern
- Left sidebar: Fixed width navigation menu with `column![]`
- Right pane: `container()` with padding for content area
- Combined with `row![]` for horizontal layout
- Always use `Length::Fill` for responsive sizing

## Known Issues

### Broken Pipe Errors on Window Close
When clicking the X button, you may see:
```
Io error: Broken pipe (os error 32)
```
**Cause:** Iced framework attempts I/O cleanup after OS closes the pipe.  
**Status:** Non-critical—application exits cleanly despite the error message.  
**Future Fix:** Implement graceful window close handler in `update()` to suppress these messages.

## Build & Run

```bash
cargo run --release  # Optimized for GUI performance
cargo build          # Debug build
```

The application uses `executor::Default` (non-blocking event loop via tokio in Iced internals).

## Future Architecture Considerations

- **Oracle DB Integration**: Commented dependency ready in `Cargo.toml`; implement in new module `src/database.rs`
- **Real-time Data Updates**: Replace static test data in graph view with Iced's subscription system
- **State Persistence**: Consider adding state serialization (serde) if needed

## Testing
No test files currently present. Consider `#[cfg(test)]` modules for:
- Graph scaling calculations (verify axis/data proportions)
- MenuItem state transitions
