A little quick-and-dirty sleep timer app, that powers off your system every 15 minutes when you have to go to bed.

Most likely has to be run as root in order to power off the system.

use --dry-run to just print the time until poweroff

see sleep_timer.service for an example systemd service for starting sleep-timer

Works with a config file with the following entries:

```
timezone_offset_from_utc

wakeup_time_monday
wakeup_time_tuesday
wakeup_time_wednesday
wakeup_time_thursday
wakeup_time_friday
wakeup_time_saturday
wakeup_time_sunday

timer_offset
```

for example:

```
1

6:40
6:40
7:30
6:40
7:30
10:00
10:00

8:30
```
