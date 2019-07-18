local hamming = {}

function hamming.compute(first, second)
    if string.len(first) ~= string.len(second) then
        return -1
    end
    local distance = 0
    for i = 1, string.len(first) do
        if string.sub(first, i, i) ~= string.sub(second, i, i) then
            distance = distance + 1
        end
    end
    return distance
end

return hamming
