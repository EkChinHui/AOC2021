
local function split (inputstr, sep)
        if sep == nil then
                sep = "%s"
        end
        local t={}
        for str in string.gmatch(inputstr, "([^"..sep.."]+)") do
                table.insert(t, str)
        end
        return t
end

local function sol_2a()
    local depth = 0
    local hor_pos = 0
    for line in io.lines("input.txt") do
        local dir= split(line, ' ')[1]
        local amt = tonumber(split(line, ' ')[2])
        -- print(dir)
        -- print(amt)
        if dir == "forward" then
            hor_pos = hor_pos + amt
        elseif dir == "down" then
            depth = depth + amt
        elseif dir == "up" then
            depth = depth - amt
        end
    end
    return depth * hor_pos
end

local ans = sol_2a()
print("Answer for 2a is ", ans)

local function sol_2b()
    local depth = 0
    local hor_pos = 0
    local aim = 0
    for line in io.lines("input.txt") do
        local dir= split(line, ' ')[1]
        local amt = tonumber(split(line, ' ')[2])
        -- print(dir)
        -- print(amt)
        if dir == "forward" then
            hor_pos = hor_pos + amt
            depth = depth + aim * amt
        elseif dir == "down" then
            aim = aim + amt
        elseif dir == "up" then
            aim = aim - amt
        end
    end
    print("Depth: ", depth)
    print("Horizontal Pos: ", hor_pos)
    return depth * hor_pos
end

local ans2 = sol_2b()
print("Answer for 2b is ", ans2)
