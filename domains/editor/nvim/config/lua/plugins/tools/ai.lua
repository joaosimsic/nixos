local manifest = require("config.manifest")
local selected_ai = manifest.tools.ai

local ai_plugins = {
	opencode = {
		"NickvanDyke/opencode.nvim",
		lazy = true,
		commit = "849a5f63514667e63318521330f28acaf13a4125",
		cmd = { "OpenCode", "OpenCodeChat" },
		keys = {
			{
				"<leader>gg",
				function()
					require("opencode").toggle()
				end,
				mode = { "n", "t" },
				desc = "Toggle opencode",
			},
			{
				"<leader>gs",
				function()
					require("opencode").ask("@this: ", { submit = true })
				end,
				mode = { "n", "v" },
				desc = "Ask opencode…",
			},
			{
				"<leader>gb",
				function()
					require("opencode").ask("@buffer: ", { submit = true })
				end,
				mode = "n",
				desc = "Ask opencode about file…",
			},
			{
				"<leader>gn",
				function()
					require("opencode").select()
				end,
				mode = { "n", "x" },
				desc = "Execute opencode action…",
			},
			{
				"go",
				function()
					return require("opencode").operator("@this ")
				end,
				mode = { "n", "x" },
				expr = true,
				desc = "Add range to opencode",
			},
			{
				"goo",
				function()
					return require("opencode").operator("@this ") .. "_"
				end,
				mode = "n",
				expr = true,
				desc = "Add line to opencode",
			},
			{
				"W",
				function()
					require("opencode").command("session.half.page.up")
				end,
				mode = "n",
				desc = "Scroll opencode up",
			},
			{
				"S",
				function()
					require("opencode").command("session.half.page.down")
				end,
				mode = "n",
				desc = "Scroll opencode down",
			},
			{
				"<leader>gl",
				function()
					require("opencode").select_session()
				end,
				mode = "n",
				desc = "List/resume OpenCode sessions",
			},
			{
				"<leader>gq",
				function()
					require("plugins.tools.opencode.lifecycle").stop()
				end,
				mode = "n",
				desc = "Stop OpenCode cleanly",
			},
		},
		dependencies = {
			{ "folke/snacks.nvim", opts = { input = {}, picker = {}, terminal = {} } },
		},
		config = function()
			require("plugins.tools.opencode.config")
			require("plugins.tools.opencode.events").setup()
			require("plugins.tools.opencode.session-management").setup()
			require("plugins.tools.opencode.lifecycle").setup()
		end,
	},
	copilot = {
		"zbirenbaum/copilot.lua",
		enabled = false,
		event = "VeryLazy",
		keys = {},
		config = function()
			require("copilot").setup({
				suggestion = {
					enabled = true,
					auto_trigger = true,
					accept = false,
				},
				panel = {
					enabled = false,
				},
				filetypes = {
					["*"] = true,
				},
			})
		end,
	},
	cursor = {
		"felixcuello/neovim-cursor",
		keys = {},
		config = function()
			require("neovim-cursor").setup({
				command = "agent",
				split = {
					position = "right",
					size = 0.4,
				},
			})
		end,
	},
	claudecode = {
		"coder/claudecode.nvim",
		cmd = { "ClaudeCode", "ClaudeCodeChat" },
		keys = {
			{
				"<leader>gg",
				"<cmd>ClaudeCode<cr>",
				mode = { "n", "t" },
			},
			{
				"<leader>gf",
				"<cmd>ClaudeCodeFocus<cr>",
				mode = "n",
			},
			{
				"<leader>gq",
				"<cmd>ClaudeCodeStop<cr>",
				mode = "n",
			},
			{
				"<leader>gl",
				function()
					vim.cmd("ClaudeCode")
					vim.defer_fn(function()
						local keys = vim.api.nvim_replace_termcodes("/rewind<cr>", true, false, true)
						vim.api.nvim_feedkeys(keys, "n", false)
					end, 150)
				end,
				mode = "n",
			},
		},
		config = function()
			require("claudecode").setup({
				terminal_cmd = "claude",
				focus_after_send = true,
				track_selection = true,
				terminal = {
					split_side = "right",
					split_width_percentage = 0.30,
					provider = "native",
					show_native_term_exit_tip = false,
					auto_close = true,
					env = {},
					snacks_win_opts = {},
				},
				diff_opts = {
					layout = "vertical",
					open_in_new_tab = false,
					keep_terminal_focus = true,
					open_in_current_tab = true,
					hide_terminal_in_new_tab = false,
					on_new_file_reject = "close_window",
				},
			})

			vim.api.nvim_create_autocmd("TermOpen", {
				group = vim.api.nvim_create_augroup("ClaudeTerminalClose", { clear = false }),
				pattern = "term://*claude*",
				callback = function()
					local opts = { buffer = true, noremap = true, silent = true }

					vim.keymap.set("t", "<Esc>", [[<C-\><C-n>]], opts)
					vim.keymap.set("t", "<C-c>", [[<C-\><C-n>]], opts)
					vim.keymap.set("t", "q", [[<C-\><C-n>]], opts)
				end,
			})
		end,
	},
}

local selected_plugin = ai_plugins[selected_ai]

if not selected_plugin then
	vim.notify("AI Setup: No plugin found for " .. tostring(selected_ai), vim.log.levels.WARN)
	return {}
end

return selected_plugin
