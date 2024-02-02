# Usopp

<div align="center">

English | [简体中文](./README.zh.md)

</div>

A replication of the [rubick](https://github.com/rubickCenter/rubick) project based on [Tauri](https://tauri.app/).

The project was originally intended for learning Rust, and the code is currently messy. Refactoring and optimization will be done in the future.

If you have better implementations, feel free to submit a pull request.

### Features
- 📋 Application list
- 🔎 Pinyin search support
- 📌 System tray minimization
- 📁 Folder and file search support

### To-Do
- [ ] Plugin support
- [ ] System settings
- [ ] Mac adaptation

### Demo

#### Application and folder search
![demo](./public/demo1-20240202.gif)

#### Custom command activation
![demo](./public/demo2.gif)

#### Webview into Window effect
![demo](./public/demo3.gif)

### Development
Rust installation is required.
```
pnpm i

pnpm run tauri
```

### Commands
Currently in the exploration phase.
```
vscode: // Open with VS Code
idea: // Open with IntelliJ IDEA
```

## License

[MIT](https://en.wikipedia.org/wiki/MIT_License)
