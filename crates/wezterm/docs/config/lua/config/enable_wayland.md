# `enable_wayland`

If `false`, do not try to use a Wayland protocol connection
when starting the gui frontend, and instead use X11.

This option is only considered on X11/Wayland systems and
has no effect on macOS or Windows.

The default is false.

```lua
return {
  enable_wayland = true,
}
```

*Since: 20220624-141144-bd1b7c5d*

The default is now `true`.
