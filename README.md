# lapce-volt

This repository serves as a starting point for writing [Lapce](https://lapce.dev/) plugins in the [Rust Programming Language](https://www.rust-lang.org).
If you haven't used Rust before, consider familiarizing yourself with the [documentation](https://doc.rust-lang.org/book/) .

## Getting started

- If not yet done, [install Rust](https://www.rust-lang.org/learn/get-started)
- This template uses [cargo-make](https://github.com/sagiegurari/cargo-make) as a build system. Install it using `cargo install --force cargo-make`.
- Generate your own repository based on this one by following the instructions [here](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template).
- [Clone it](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository) locally
- Ensure the wasi target is installed, using: `rustup target add wasm32-wasi`
- Compile your addon by executing `cargo make dev` in the cloned directory. This will create a `bin/lapce-plugin-template.wasm` file.
- Edit the extension's metadata in the `volt.toml` file.
  - Check the `activation` block. Here, you can configure when Lapce will load your plugin. The two corresponding settings, `language` and `workspace-contains` accept an array of RegExps.
    - Although this is **not recommended because of performance concerns**, to always load your extension, you can use `*`
    - Edit the `wasm` property to reflect the (relative) path of the built plugin.
- Edit the `Cargo.toml` name property to match the `volt.toml` name property.
- Lapce loads extensions that are placed inside its plugins folder.
  - If your filesystem supports symbolic links, symlink the plugin directory to `~/.local/share/lapce-stable/`, or `/Users/[USERNAME]/Library/Application Support/dev.lapce.Lapce-Stable/plugins` for Mac.
  - If it doesn't,
    - Make a folder in the plugins directory in the format of `author.plugin-name`, so for example `~/.local/share/lapce-stable/plugins/bob.awesome-plugin`
    - copy the forementioned `lapce-plugin-template.wasm` and the `volt.toml` files to this directory
  - Open Lapce, or, if already open, run the `Reload Window` command and check if your plugin appears in the sidebar.
    - If it does not, check that the relative path to the `.wasm` file is correct.

## Development

- The `src/main.rs` file provides plenty of comments to help you get started with writing your plugin.
- You can configure the user-facing settings of the plugin in the `config` header of the `volt.toml` file.
  - The plugin has read and write access to it's own folder only. **DO NOT use this to persist data**, because it'll be deleted when Lapce updates your plugin.
- You can quickly reload your extension without restarting lapce by clicking the cog icon besides the plugin's version in the UI and selecting `Reload Plugin`
- To show user-facing notifications, use `PLUGIN_RPC.window_show_message()`. _You can't use `print!()` and `eprint!()` from plugins for logging._
- The template includes [`anyhow`](https://github.com/dtolnay/anyhow) for pragmatic, flexible error handling.

## Publishing your plugin

- Before publishing, check that all information is correct in `volt.toml`.
- If there is one, Lapce will show the `README.md` file as the plugin's description. You might want to write one to help with discoverability.
- Register an account for [Lapce's extension directory](https://plugins.lapce.dev)
- Install the `volts` tool, used for publishing extensions with `cargo install volts`
  - Create an api token on the website.
  - Build your plugin, and navigate to the directory of it (in the `plugins` directory, from where it's loaded)
  - Execute `volts publish`. Paste the token, that you've just created, in.
  - In a few minutes, your plugin should be up.
