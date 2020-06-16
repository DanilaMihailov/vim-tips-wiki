# Vim Tips Wiki
**1500+** tips downloaded from [Vim Tips Wiki](https://vim.fandom.com/wiki/Vim_Tips_Wiki), parsed and formatted to look and work like native Vim help files. 

You can use `:helpgrep` to search tips, `Ctrl-]` to jump tags and `gO` (thats capital o) to see table of contents. All tips are numbered and tagged in a format `vtw-N` where `N` is number of a tip. 

Try `:help vtw-1`.

**NOTE**: this plugin is *NOT* related to [vimwiki](https://github.com/vimwiki/vimwiki) (amazing plugin btw).

![Screenshot](https://github.com/DanilaMihailov/vim-wiki-tips/blob/master/screenshots/preview.png?raw=true)

## Installation

### [vim-plug](https://github.com/junegunn/vim-plug)
1. Add the following configuration to your `.vimrc`.

        Plug 'danilamihailov/vim-tips-wiki'

2. Install with `:PlugInstall`.

Or use your favorite plugin manager

## Why, though?
Sometimes I find myself using `:helpgrep` and not finding what I am looking for, often because I did not use correct word or phrase. I hope that having 1500+ tips written by users will help solve this problem.

## TODO
While this plugin is complete and ready to be used, I still want to add some features

- [ ] Use categories
- [ ] Add index of all tips
- [ ] Add :RandomVimTip command
