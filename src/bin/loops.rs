use mlua::Lua;

fn main() {
    let lua = Lua::new();

    let script = lua.load(
        r#"
i = 1
while (i <= 5) do
  io.write(i)
  i = i + 1
end
print()

i = 2
while (i <= 8) do
  io.write(i)
  i = i + 1
  if i == 5 then break end
end

repeat
  print("Enter your guess: (answer: 5)")
  local guess = io.read()
until tonumber(guess) == 5

for i = 1,5,1 do
  io.write(i)
end

print()

for i = 2,10,2 do
  io.write(i)
end

local neovim_plugins = {
    "nvim-treesitter",
    "telescope.nvim",
    "nvim-lspconfig",
    "lazy.nvim",
    "nvim-cmp",
    "luasnip",
    "gitsigns.nvim",
    "which-key.nvim",
    "lualine.nvim",
    "mini.nvim"
}

print()

for key,value in pairs(neovim_plugins) do
  io.write(value, " | ")
end
        "#,
    );

    let _ = script.exec();
}
