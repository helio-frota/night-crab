use mlua::Lua;

fn main() {
    let lua = Lua::new();

    let script = lua.load(
        r#"
cat_name = "foobar"

print(cat_name)

description = [[
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed eu accumsan quam. Cras justo tortor, luctus ut suscipit vel,
venenatis imperdiet enim. Morbi dolor velit, tempus eget leo vel orci aliquam
]]

io.write(description, "\n")

description = description .. cat_name

io.write(description, "\n")

friday = true

print(type(description))
print(type(friday))
print(type(new_var_here))
print(type(123))
        "#,
    );

    let _ = script.exec();
}
