"https://github.com/esm7/obsidian-vimrc-support

imap jj <Esc>
" Yank to system clipboard
set clipboard=unnamed

" Go back and forward with Ctrl+O and Ctrl+I
" (make sure to remove default Obsidian shortcuts for these to work)
exmap back obcommand app:go-back
nmap <C-o> :back
exmap forward obcommand app:go-forward
nmap <C-i> :go-forward


" vnorempa dont work in the obsidian-vimrc-support plugin
"vnoremap J :m '>+1<CR>gv=gv
"vnoremap K :m '<-2<CR>gv=gv
""nunmap J
""vunmap J
""nunmap K
""vunmap K
""" NOTE: must use 'map' and not 'nmap'
""map J :m '>+1<CR>gv=gv
""map K :m '<-2<CR>gv=gv

"" Have j and k navigate visual lines rather than logical ones 
"" Remap for dealing with word wrap (neovim)
"" (v:count is the count given for the last Normal mode command) 
""vim.keymap.set("n", "k", "v:count == 0 ? 'gk' : 'k'", { expr = true, silent = true })
""vim.keymap.set("n", "j", "v:count == 0 ? 'gj' : 'j'", { expr = true, silent = true })
nmap j gj
nmap k gk

" Surround
"Select some text in visual mode, then press s and then the desired surround character. e.g. s" to surround the selected text with double quotes.
"Place your cursor over a word in normal mode, then press s and then the desired surround character. e.g. s" to surround the word with double quotes.
exmap surround_wiki surround [[ ]]
exmap surround_double_quotes surround " "
exmap surround_single_quotes surround ' '
exmap surround_backticks surround ` `
exmap surround_brackets surround ( )
exmap surround_square_brackets surround [ ]
exmap surround_curly_brackets surround { }

" NOTE: must use 'map' and not 'nmap'
map [[ :surround_wiki
nunmap S
vunmap S
map S" :surround_double_quotes
map S' :surround_single_quotes
map S` :surround_backticks
map Sb :surround_brackets
map S( :surround_brackets
map S) :surround_brackets
map S[ :surround_square_brackets
map S[ :surround_square_brackets
map S{ :surround_curly_brackets
map S} :surround_curly_brackets

