# Resync

Simple tool for syncing translation strings between iOS & Android

## Installation

1. Download the latest release from [here](https://github.com/0xSkash/resync-rs/releases).
2. Add resync to your shell path

 ```shell
export PATH=$PATH:/path/to/resync
```

3. Test

```shell
resync --help
```

## Running

| Command  | Description                                                                               |
|----------|-------------------------------------------------------------------------------------------|
| sync     | Generate a string resource file for the other platform (iOS to Android or Android to iOS) |
| convert  | Convert between resource string file formats                                              |
| generate | Generate a CSV file which can be used to generate platform string resource files          |


## TODO:
- Add support for format specifiers
- Add support for comments