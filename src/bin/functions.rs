use mlua::Lua;

fn main() {
    let lua = Lua::new();

    let script = lua.load(
        r#"
quote = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut dictum maximus accumsan. Duis in ligula massa. Sed sed dui egestas, pharetra dui vitae, interdum diam. Maecenas blandit sed tellus hendrerit eleifend. Aliquam vitae hendrerit purus. Etiam non leo aliquam, congue dui sit amet nullam sodales."

function config(str)
  return str .. "-config"
end
print(config("abc"))

function count_words(quote)
  local count = 0
  for _ in quote:gmatch("%S+") do
    count = count + 1
  end
  return count
end
print(count_words(quote))

function one_to_five()
  return "one","two","three","four","five"
end
print(select(2, one_to_five()))
print(select(5, one_to_five()))

function outer_func()
  local i = 0
  return function()
    i = i + 1
    return i
  end
end
inc_i = outer_func()

print(inc_i())
print(inc_i())
print(inc_i())
        "#,
    );

    let _ = script.exec();
}
