# CLI

## Basic usage on local manifests

Comtrya works by running a manifest or set of manifests. Following are examples of running comtrya against manifests that are on the local machine:

```shell
# Run all manifests within your current directory
comtrya apply

# Run all manifests within your current directory and a specified configuration file
comtrya -d ./ -c /path/to/Comtrya/yaml

# --manifests, or -m, will run a subset of your manifests
comtrya -d ./ apply -m one,two,three

# Run all manifests within a specified directory
comtrya -d ./manifests apply
```

Please refer to the [commands](commands.md) section for more information about the usage of `apply`.

## Basic usage on remote manifests

Comtrya also has the ability to run remote manifests, normally hosted in a git repository on github.

```shell
# Manifests in a Git repository
comtrya -d https://github.com/rawkode/rawkode apply

# Manifests in a Git repository with a branch and path
comtrya -d https://github.com/rawkode/rawkode#main:dotfiles apply

# Manifests in a Git repository with a branch and path and a subset selector
comtrya -d https://github.com/rawkode/rawkode#main:dotfiles apply -m dev.git
```

## Help menu

Comtrya provides a help menu that can be shown by running the following command in your terminal:

```shell
comtrya -h
```

```shell
A tool to simplify reprovisioning a fresh OS. Installs packages and manages dotfiles.

Usage: comtrya [OPTIONS] <COMMAND>

Commands:
  apply            Apply manifests
  status           List manifests status (ALPHA)
  version          Print version information
  contexts         List available contexts
  gen-completions  Auto generate completions
  help             Print this message or the help of the given subcommand(s)

Options:
  -d, --manifest-directory <MANIFEST_DIRECTORY>

  -c, --config-path <CONFIG_PATH>
          Specify a configuration path (if invalid Comtrya will exit)
      --no-color
          Disable color printing
  -D, --defines <DEFINES>

  -v...
          Debug & tracing mode (-v, -vv)
  -h, --help
          Print help
  -V, --version
          Print versionA tool to simplify reprovisioning a fresh OS. Installs packages and manages dotfiles.
```

## Auto generate completions

Shell completions for comtrya can be generated by the desired shell via `gen-completions` subcommand.

```shell
comtrya gen-completions [SHELL]
```

for bash:
```shell
source <(comtrya gen-completions bash)
```

for fish:
```shell
comtrya gen-completions fish | source
```

## Define variables via CLI

Comtrya offers the ability to set variables to use throughout manifests. Variables have normally been defined via the Comtrya config file named `Comtrya.yaml` at the root directory of the manifests. However, variables can also be defined in the command line interface by using the `defines` options.

```shell
comtrya -v -d ./ --defines foo=bar apply -m sample
```

In the sample, a variable is created in comtrya with the name `foo` and is set to the value `bar`. An action, like the sample below, can utilize this variable.

```yaml
- action: command.run
  command: echo
  args:
    - "{{ variables.foo }}"
```
