# rofi-totp

    A rofi plugin for Two-Factors Authentication / Time-Based Authentication Token / TOTP /  Google Authenticator

    From `0.2.0` we support ini format which is used by [alfred-workflow-gauth](https://github.com/moul/alfred-workflow-gauth)
    but it is fully backward compatible with old `yaml` config.

### Dependencies

#### On ubuntu

```
sudo apt install rofi xdotool
```

### Setup

#### ini config [alfred-workflow-gauth](https://github.com/moul/alfred-workflow-gauth#installation)

Create file named `.gauth` in your home directory and add apps in the format

```ini
[google - bob@gmail.com]
secret=xxxxxxxxxxxxxxxxxx

[evernote - robert]
secret=yyyyyyyyyyyyyyyyyy
```

#### YAML config

Create file named `2fa.yml` in your home directory and add apps in the format

```yml
apps:
  - label: Google
    secret: NUYJ2UDBUNZIQGDE
  - label: Amazon
    secret: NUYJ2UDBUNZIQGDF
  - label: Slack
    secret: NUYJ2UDBUNZIQGDG
```

### Installation

Download executable from [releases](https://github.com/revathskumar/rofi-totp/releases/latest) and keep it in you `$PATH`

### Development

```sh
RUST_BACKTRACE=1 cargo run
```

## License

Please see [License](https://github.com/revathskumar/rofi-totp/blob/master/License)
