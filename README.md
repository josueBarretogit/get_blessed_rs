# get-blessed-rs

Terminal tool to get you the best crates for your rust projects with a few keybindings, curated by [blessed.rs](https://blessed.rs/crates)

## Motivation
As you may know, the website blessed.rs compiles the most popular crates that almost any rust project needs, so I decided to make a program that lets you look at those crates and add 
them to your rust projects right in your terminal. 

## Showcase 

![showcase](./public/showcase.gif)

## Features


- Check out a crate documentation you can by pressing `<d>` (Opens a tab in your default web browser)
- Check out a crate crates.io page you can by pressing `<c>` (Opens a tab in your default web browser)
- Change categories with `<Tab>` or `<Shift + Tab>` 
- Move up and down with either arroy keys or with `<j>` / `<k>`
- Select the crate you want to add to your project by pressing `<s>`
- Select all the crates from a category by pressing `<a>`
- Select a crate with features by pressing  `<f>` to select its features
- Add the selected crates to your rust project by pressing `<Enter>`
  
![features](./public/features.gif)


- After you are done press `<q>` or `<Esc>` to quit 

## Installation

```bash
  cargo install get-blessed
```

Arch Linux users can install [from the AUR](https://aur.archlinux.org/packages/get-blessed) via `paru -S get-blessed`.

After you are done adding the best crates to your rust project also consider making a .gitignore [add-gitignore-cli](https://crates.io/crates/add-gitignore-cli)

Please feel free to suggest new features or report any bugs / inconsistencies
