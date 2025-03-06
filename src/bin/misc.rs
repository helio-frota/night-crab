use mlua::Lua;

fn main() {
    let lua = Lua::new();

    let script = lua.load(
        r#"
-- coroutine
producer = coroutine.create(function()
  for i = 1,5,1 do
    print("producing:", i)
    coroutine.yield(i)
  end
end)

while coroutine.status(producer) ~= "dead" do
  local ok,value = coroutine.resume(producer)
  if ok then
    print("consumed:", value)
  end
end

-- files
file = io.open("quote.txt", "r+")
file:seek("set", 0)
print(file:read("*a"))
file:close()

-- modules
util_module = require("util")
print(util_module.add(1,2))

-- metatable
local base = {
  greet = function(self)
    print("hi, my name is " .. self.name)
  end
}

local person = { name = "FooBar" }
setmetatable(person, { __index = base })
person:greet()

Animal = { name = "No name", sound = "No sound" }

function Animal:new(name, sound)
  setmetatable({}, Animal)
  self.name = name
  self.sound = sound

  return self
end

function Animal:toString()
  animal_str = string.format("%s says %s", self.name, self.sound)
  return animal_str
end

fifi = Animal:new("Fifi", "Miau")
print(fifi:toString())

-- error message
error("Error: something is wrong")
print("error not showing ^, including this message")
        "#,
    );

    let _ = script.exec();
}
