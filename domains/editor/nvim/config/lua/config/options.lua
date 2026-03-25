-- Indentation
vim.opt.tabstop = 2
vim.opt.softtabstop = 2
vim.opt.shiftwidth = 2
vim.opt.expandtab = true

-- Clipboard
vim.opt.clipboard = "unnamedplus"

-- Line numbers
vim.opt.relativenumber = true
vim.opt.number = true

-- Colors
vim.opt.termguicolors = true

-- Reduce timeout for leader key
vim.opt.timeoutlen = 300

vim.opt.updatetime = 250

-- UI Settings
vim.o.guicursor = "n-v-c:block"
vim.o.guicursor = vim.o.guicursor .. ",i-c:block-blinkon1"
vim.o.guicursor = vim.o.guicursor .. ",r:block"

vim.opt.cursorline = true
vim.api.nvim_set_hl(0, "CursorLine", { bg = "#2a2a2a" })

vim.opt.colorcolumn = "100,101"

vim.api.nvim_set_hl(0, "DiagnosticUnderlineError", { fg = "#6272a4", italic = true })
vim.api.nvim_set_hl(0, "DiagnosticVirtualTextError", { fg = "#6272a4" })

vim.api.nvim_set_hl(0, "DiagnosticUnnecessary", { fg = "#6272a4", italic = true })
vim.api.nvim_set_hl(0, "DiagnosticHint", { fg = "#6272a4" })
vim.api.nvim_set_hl(0, "DiagnosticVirtualTextHint", { fg = "#6272a4" })
vim.api.nvim_set_hl(0, "DiagnosticUnderlineHint", { sp = "#6272a4", underline = true })

-- Diagnostics Configuration
vim.diagnostic.config({
	virtual_text = true,
	severity_sort = true,
	float = { border = "none" },
	update_in_insert = false,
})
