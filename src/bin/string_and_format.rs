use mlua::Lua;

fn main() {
    let lua = Lua::new();

    let script = lua.load(
        r#"
quote = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut dictum maximus accumsan. Duis in ligula massa. Sed sed dui egestas, pharetra dui vitae, interdum diam. Maecenas blandit sed tellus hendrerit eleifend. Aliquam vitae hendrerit purus. Etiam non leo aliquam, congue dui sit amet nullam sodales."
print("length", string.len(quote))
print("length", #quote) -- same as above
print(quote)
print("Replace dictum with foobar\n", string.gsub(quote, "dictum", "foobar"))
print("idex of Duis", string.find(quote, "Duis"))
print(string.upper(quote))
print(string.lower(quote))
print("----------")
print(math.random(1, 3))
math.random(os.time())
print(math.random())
print(string.format("value = %.3f", 1.2345))
        "#,
    );

    let _ = script.exec();
}
