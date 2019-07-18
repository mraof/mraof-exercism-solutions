local DNA = { nucleotideCounts = { A = 0, T = 0, C = 0, G = 0 } }

function DNA:new(s)
    self.nucleotideCounts = { A = 0, T = 0, C = 0, G = 0 }
    for i = 1, string.len(s) do
        local c = string.sub(s, i, i)
        self.nucleotideCounts[c] = self.nucleotideCounts[c] + 1
    end
    return self
end

function DNA:count(n)
    if n ~= 'A' and n ~= 'T' and n ~= 'C' and n ~= 'G' then
        error('Invalid Nucleotide')
    end
    return self.nucleotideCounts[n]
end

return DNA
