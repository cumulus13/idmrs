# idmrs

Command line downloader for **Internet Download Manager (IDM)** on Windows.
Rust port of [pyidm](https://github.com/cumulus13/pyidm) (`idm/idm.py`).

Sends a link to IDM via COM automation (`CIDMLinkTransmitter` /
`SendLinkToIDM2`), exactly like the Python original does through `comtypes`,
but with no Python runtime required.

> Windows only — IDM itself is a Windows-only application, so this tool
> refuses to run (with a clear error) on Linux/macOS, same as `idm.py`.

## Install

```
cargo install idmrs
```

Or download a prebuilt binary from [Releases](https://github.com/cumulus13/idmrs/releases).

## Usage

```
idmrs [OPTIONS] [URLS]...
```

| Flag | Description |
|---|---|
| `URLS` | One or more URLs to download, or `c` to read the URL from the clipboard |
| `-p, --path <PATH>` | Directory to save into |
| `-o, --output <NAME>` | Save with a different filename |
| `-c, --confirm` | Ask for confirmation before download starts |
| `-a, --add` | Add the link to IDM without starting the download |
| `-r, --referrer <URL>` | Referrer URL |
| `-C, --cookie <STRING>` | Cookie string |
| `-D, --post-data <STRING>` | POST data string |
| `-U, --username <USER>` | Username, if the download requires auth |
| `-P, --password <PASS>` | Password, if the download requires auth |
| `--user-agent <UA>` | Custom User-Agent string |
| `--config <SECTION:OPTION:VALUE>` | Persist a setting to `idm.ini`, next to the executable |
| `--config doc` | Print valid config section/option names |

### Examples

```
# Download a single URL
idmrs https://example.com/file.zip

# Grab the URL currently in the clipboard, save to D:\Downloads
idmrs c -p D:\Downloads

# Add without starting the download, with a custom user-agent
idmrs https://example.com/file.zip --add --user-agent "Mozilla/5.0"

# Persist a default download path
idmrs --config download:path:D:\Downloads
```

## Config file (`idm.ini`)

Stored next to the `idmrs.exe` binary. Recognized keys:

```
[download]
path = D:\Downloads
confirm = 0

[data]
user_agent = Mozilla/5.0 ...

[debug]
verbose = 0
```

Set any of these from the CLI with `--config section:option:value`,
e.g. `idmrs --config download:confirm:1`.

## Building from source

```
git clone https://github.com/cumulus13/idmrs
cd idmrs
cargo build --release --target x86_64-pc-windows-msvc
```

## Publishing (maintainers)

Pushing a tag matching `v*.*.*` runs the release workflow: it builds and
tests on Windows/Linux/macOS, then publishes to crates.io using the
`CARGO_REGISTRY_TOKEN` repository secret, then cuts a GitHub Release.

```
git tag v0.1.0
git push origin v0.1.0
```

## License

MIT © Hadi Cahyadi <cumulus13@gmail.com>

## 👤 Author
        
[Hadi Cahyadi](mailto:cumulus13@gmail.com)
    

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)