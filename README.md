# Rust Git Hooks
Git hooks are scripts that Git executes before or after events such as: commit, push, and receive. Git hooks are a built-in feature - no need to download anything. Git hooks are run locally.

### How do git hooks work?

Every Git repository has a `.git/hooks` folder with a script for each hook you can bind to. You're free to change or update these scripts as necessary, and Git will execute them when those events occur.

Here's a full list of hooks you can attach scripts to: [git hooks lists](https://githooks.com/).

## Environments

- rust -> 1.0 (nightly channel)
- git2 -> "0.10"
- time -> "0.1"
- futures -> "0.3.1"

## Running

To try the hooks, do the following to compile the hooks and symlink them into your git hooks folder.

```sh
cargo run
```

And then you will see the output like this:

```sh
Last Commit:
Id -> ...
Message -> ...
Committer Name -> ...
Committer Email -> ...
Time -> ...
Raw -> "..." 

Branch has been succesfully created! If you wanna check type command git branch -a
Branch has been succesfully renamed! If you wanna check type command git branch -a
Branch has been succesfully deleted! If you wanna check type command git branch -a
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.
