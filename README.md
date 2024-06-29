# get-blessed-rs

Terminal tool to get you the best crates for your rust projects with a few keybindings, curated by [blessed.rs](https://blessed.rs/crates)

## Motivation
As you may know, the website blessed.rs compiles the most popular crates that almost any rust project needs, so I decided to make a program that lets you look at those crates and add 
them to your rust projects right in your terminal. 

## Showcase 

![showcase](./public/showcase.gif)

## Features


- Check out a crate's documentation by pressing `<d>` (Opens a tab in your default web browser)
- Check out a crate's crates.io page by pressing `<c>` (Opens a tab in your default web browser)
- Move between categories with `<Tab>` or `<Shift + Tab>` 
- Move up and down with either `<Up>` and `<Down>` arrow keys or with `<j>` / `<k>`
- Select the crate you want in your project by pressing `<s>`
- Select all the crates from a category by pressing `<a>`
- Select a crate with features by pressing `<f>` (Opens a popup where you can select the features with `<s>`)
- Add the selected crates to your rust project by pressing `<Enter>`
- Close the application with `<q>` or `<Esc>`

## Installation

```bash
  cargo install get-blessed
```

Arch Linux users can install [from the AUR](https://aur.archlinux.org/packages/get-blessed) via `paru -S get-blessed`.

After you are done adding the best crates to your rust project also consider making a .gitignore [add-gitignore-cli](https://crates.io/crates/add-gitignore-cli)

Please feel free to suggest new features or report any bugs / inconsistencies
