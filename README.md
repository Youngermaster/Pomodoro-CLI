# Pomodoro-CLI

This is a pomodoro CLI to be more productive using the Pomodoro method.

## How to run
```shell
cargo run 
# Or
caro run --release
```

## How to build
```shell
cargo build
# Or
cargo build --release
```

## Note:
Make sure to install `pulseaudio-alsa` or `libasound2-dev`.

## Troubleshooting
If ALSA mixer doesn't work, try this command as `sudo`:

```
echo "options snd-hda-intel index=1" > /etc/modprobe.d/alsa.conf
```

## TODO:

- [ ] Add installation support.
  - [ ] Windows.
  - [ ] Linux.
  - [ ] MacOS.
- [ ] Add project *"modularity"*.
