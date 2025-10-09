# time-butler Configuration

`time-butler` uses a configuration file to determine where it stores its data and  generated reports.
You can specify a custom configuration file path using the `--config` CLI argument.
If not specified, it defaults to `tb-config.json` in the `working directory`. If not present, it will create a new one with the default configuration set.

## Using a Custom Configuration File

To use a custom configuration file, run:

```bash
time-butler --config /path/to/your-config.json
```

## Default Configuration Values

A default configuration file looks like this:

```json
{
  "time-butler-storage-directory": "/home/<youruser>/.local/time-butler",
  "time-butler-project-data-path": "/home/<youruser>/.local/time-butler/.app_storage/prj_data.bin",
  "time-butler-week-data-path": "/home/<youruser>/.local/time-butler/.app_storage/week_data.bin",
  "time-butler-report-generation-directory": "/home/<youruser>/.local/time-butler/.generated_reports"
}
```

- **time-butler-storage-directory**: Main directory for all time-butler data.
- **time-butler-project-data-path**: Path to the binary file storing project data.
- **time-butler-week-data-path**: Path to the binary file storing week data.
- **time-butler-report-generation-directory**: Directory where generated reports are saved.

The default paths are based on your home directory, but you can edit the configuration file to use any paths you prefer.

For more details, you can always run:

```bash
time-butler --help
```

## Dump configuration
In order to get a quick overview of the current configuration used, or to see where the configuration file are stored the `dump` functionality can be used. 
The configuration can be dumped both to the terminal or to a file. 

To dump configuration, use the `configuration` command. 

```bash
# To terminal
time-butler configuration dump --dump-terminal

# To file
time-butler configuration dump --dump-file tmp-storage/time-butler-config.dump
```
