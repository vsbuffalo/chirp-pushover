# chirp-pushover — command line notifier for Pushover

[![Crate](https://img.shields.io/crates/v/chirp-pushover.svg)](https://crates.io/crates/chirp-pushover)

[Pushover](https://pushover.net) is a service that manages push notifications
for iPhone and Android devices. *chirp* is a command line tool that can push
notifications to your phone, e.g. for when a job completes, an error is
encountered, etc.

```
genome_assembly.sh && chirp success -m "Genome assembly complete"
```

Chirp also includes a simple Bash wrapper script (installed by `chirp config`)
that can be used to pass more information to the message about the command,
e.g. the total runtime duration and the exit status. For example,

```
chirper -t "Genome assembly" genome_assembly.sh
```

will send push notifications like "Command successful: genome_assembly.sh (Exit
code: 0, Duration: 12h 12m 3s)" or "Command failed: genome_assembly.sh (Exit
code: 1, Duration: 2h 12m 11s)" depending on the exit status, etc.

## Chip Installation and Pushover Setup

1. First, install the Chirp command line tool using Rust's `cargo` command. If
   you do not have Rust installed, it is extremely easy to get going: just
   follow the instructions here: https://rustup.rs. Then, install `chirp` with:

   ```
   cargo install chirp-pushover
   ```

2. Then, download the [iPhone](https://pushover.net/clients/ios) or
   [Android](https://pushover.net/clients/android) Pushover app, depending on
   your type of device.

1. Open the app. At this point, you probably want to pay the one-time $5 cost.

2. Visit https://pushover.net and save your **user key** somewhere.

3. Then, we need to register the chirp application with Pushover. To do this
   visit https://pushover.net/apps/build and for the name, fill "chirp" (or,
   whatever you like). You can leave the other boxes empty, but check the
   service terms box, and press submit. This should bring you to a page that
   has the **API token** on it.

4. Finally run the one-time configuration of the for your machine that links
   the `chirp` tool with Pushover by running:

    ```
    chirp config --api-token <API_TOKEN> --user-key <USER_KEY>
    ```

    This saves these tokens in `~.pushover_tokens.yml`, and also installs a
    Bash-wrapper script in `~/.local/bin/chirper`. This simple Bash script
    wraps command calls, which allows more information to be passed directly
    into messages sent by `chirp` (e.g. the exit status and run time duration).

## Using Chirp

Chirp is a very simply command line tool. See `chirp --help` for a list of all
subcommands. To send messages, you can use `chirp msg --title <title> --message
<message> --priority <priority>`. Since `chirp` is likely going to be used
predominantly to monitor events with success/fail termination statuses, the
subcommands `chirp success --message <message>` and `chirp failure --message
<message>` automatically create titles that indicate success or failure. A
priority can also be set; the `--priority emergency` priority will keep pinging the
individual until they acknowledge the notification.

The `chirper` Bash script is also quite simple, just specify your full command
after the `chirper` settings,

```
chirper -t "Genome Assembly" -m "big genome assembly"  -- genome_assembly.sh
```
