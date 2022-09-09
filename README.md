# Volume Mixer for Arctis Nova Headsets
Utility to manage the ChatMix dial for Arctis Nova headsets

## Current Status
⛔ Non-functional

## TODO
- [x] Read headset volume mixer state
  - [ ] Add product IDs for other models
- [ ] Use Windows [Core Audio API][win-core-audio] to access Windows mixer

## Prerequisites
### Linux
Grant read access for users in the `audio` group to the raw HID device by adding a `udev` rule:
```shell
make system-permissions
```

[win-core-audio]: https://docs.microsoft.com/en-us/windows/win32/coreaudio/audio-sessions
