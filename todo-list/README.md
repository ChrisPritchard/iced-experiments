# Todo list

A basic to-do list application where users can add, delete, and mark tasks as completed. This will help you understand how to handle user input, manage data, and update the UI accordingly.

Functionality:

- default task items, sorted into three status': TODO, DOING, DONE
- can add new task items to any column
- can edit task items, and stop editing
- can delete task items
- can drag an item from one row to another, changing its status

Most complicated part was applying style to a container:

- `.style` must accept something that matches `Fn(&Theme) -> Container::Appearance`
- so passing a `fn task_style(theme: &Theme) -> Appearance {}` *should* work at first glance
- or a `let s = |t: &Theme| -> Appearance {}`...
- however they don't, becuase in rust these don't auto-coerce to a generic '`Fn(&Theme) -> Container::Appearance`' type, but rather have a specific generated type for their specific value (e.g. type of task_style whatever in the first example
- so to use them, they need to be casted, specifically using something like `as for<'r> fn(&'r _) -> _`.

I figured this out with something like:

```rust
let style = |theme: &Theme| -> Appearance {
    let palette = theme.extended_palette();
    container::Appearance {
        border_width: 2.,
        border_color: palette.primary.strong.color,
        ..Default::default()
    }
} as for<'r> fn(&'r _) -> _;

container(content)
    .style(style);
```

This could be also written as:

```rust
let style: for<'r> fn(&'r _) -> _ = |theme: &Theme| -> Appearance {
    let palette = theme.extended_palette();
    container::Appearance {
        border_width: 2.,
        border_color: palette.primary.strong.color,
        ..Default::default()
    }
};

container(content)
    .style(style);
```