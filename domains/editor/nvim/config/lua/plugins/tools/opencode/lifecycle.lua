local M = {}

local function stop_opencode_cleanly()
	pcall(require("opencode.events").unsubscribe)

	pcall(require("opencode.provider").stop)

	vim.schedule(function()
		for _, buf in ipairs(vim.api.nvim_list_bufs()) do
			if vim.api.nvim_buf_is_valid(buf) then
				local bufname = vim.api.nvim_buf_get_name(buf)
				if bufname:match("term://.*opencode") then
					local chan = vim.bo[buf].channel
					if chan and chan > 0 then
						pcall(vim.fn.jobstop, chan)
					end
					pcall(vim.api.nvim_buf_delete, buf, { force = true })
				end
			end
		end
	end)
end

function M.setup()
	vim.api.nvim_create_autocmd("VimLeavePre", {
		callback = function()
			stop_opencode_cleanly()
		end,
		desc = "Cleanly stop OpenCode before Neovim exit",
	})

	vim.api.nvim_create_autocmd("QuitPre", {
		callback = function()
			local windows = vim.api.nvim_list_wins()
			local non_floating_windows = vim.tbl_filter(function(win)
				local config = vim.api.nvim_win_get_config(win)
				return not config.relative or config.relative == ""
			end, windows)

			if #non_floating_windows <= 1 then
				stop_opencode_cleanly()
			end
		end,
		desc = "Stop OpenCode when quitting last window",
	})
end

function M.stop()
	stop_opencode_cleanly()
	vim.notify("OpenCode stopped", vim.log.levels.INFO)
end

return M
