# Sway Tray Status

This is a helper program to show basic information in swaybar. See [Sway](https://github.com/swaywm/sway) a Wayland Compositor similar to i3 for more information.

The Information shown includes:

- Load avg (avg number of jobs waiting) from /proc/loadavg
- Memory available, total and free memory from /proc/meminfo.
- Battery level in percent, if battery_id is provided.
- Date and time

It uses unicode characters to show symbols. So, make sure the swaybar is configured to display information using a nerd font. I recommend "RobotoMono Nerd Font Mono".

## Configuration

```
#
# Status Bar:
#
# Read `man 5 sway-bar` for more information about this section.
bar {
    position bottom 
    status_command /<some_dir>/sway_tray_status BAT0
    colors {
        statusline #ffffff
        background #323232
        inactive_workspace #32323200 #32323200 #5c5c5c
    }
}
```

