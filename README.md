# Meeting Countdown

![Rust](https://github.com/EdJoPaTo/meeting-countdown/workflows/Rust/badge.svg)

Is made to work with these projects:

- [esp-http-neomatrix-text](https://github.com/EdJoPaTo/esp-http-neomatrix-text)
- [esp-remotecontrolled-led-matrix](https://github.com/EdJoPaTo/esp-remotecontrolled-led-matrix)

The led matrix will then show a countdown in minutes / seconds until the end of the meeting.
The color is slowly moving from green towards red.

Enjoy your meetings with hopefully better timeboxes.

# Usage

```bash
export MEETING_PIXELMATRIX='espPixelmatrix:1337'
# or
export MEETING_HTTP_TEXTMATRIX='http://esp-matrix/'

meeting-countdown 10:00 11:30
```

Check `--help` for more details.
