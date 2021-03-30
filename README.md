# cald

A Calendar Daemon.

## Action substitution

| Format    | Replaced by                           |
|-----------|---------------------------------------|
| `%<name>` | Value in the data section by `<name>` |
| `%n`      | Name of this event                    |
| `%a`      | Action of this event                  |

## Event trigger notation

### Time components

`\d[yMwDdhms]`

| Suffix | Unit               |
|--------|--------------------|
| `y`    | Year               |
| `M`    | Month of a year    |
| `w`    | Week of a year     |
| `D`    | Day of a week      |
| `d`    | Day of a year      |
| `h`    | Hour of a day      |
| `m`    | Minute of an hour  |
| `s`    | Second of a minute |

### Compound triggers

- Match when all specified time parts are met: `<time component>+` 
- Match when all conditions are met: `a(<trigger>(,<trigger>)*)`
- Match when any of the conditions are met: `o(<trigger>(,<trigger>)*)`
