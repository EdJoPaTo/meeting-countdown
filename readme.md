# Meeting Countdown
![Rust](https://github.com/EdJoPaTo/meeting-countdown/workflows/Rust/badge.svg)

Prints out the rest time of a meeting compatible for [led-matrix-remote](https://github.com/EdJoPaTo/led-matrix-remote).

The led matrix will then show a countdown in minutes / seconds until the end of the meeting.
The color is slowly moving from green towards red.
It ends with the `end-text` in blue.

Enjoy your meetings with hopefully better timeboxes.

# Usage

Example:

```sh
meeting-countdown --verbose 10:00 11:30 | led-matrix-remote http
```

```plaintext
Meeting Countdown 0.4.0
EdJoPaTo <meeting-countdown-rust@edjopato.de>
Prints out the rest time of a meeting compatible for led-matrix-remote

USAGE:
    meeting-countdown [FLAGS] [OPTIONS] <STARTTIME> <ENDTIME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show each time tick on stdout

OPTIONS:
        --end-text <STRING>      Text which is displayed when the countdown ends. [default: Meeting is over. Have a nice day!]
        --blink <INT>            Seconds before end where the time should blink. 0 to disable [default: 300]
        --start-text <STRING>    Text which is displayed before countdown starts.

ARGS:
    <STARTTIME>    Start time of the Meeting. From then the remaining time is published.
    <ENDTIME>      End time of the Meeting. Until then the remaining time is published.
```
