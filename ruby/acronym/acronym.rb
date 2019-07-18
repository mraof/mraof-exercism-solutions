class Acronym
  def self.abbreviate(full)
    full.scan(/(?<!')\b[a-zA-Z]/).map(&:upcase).join
  end
end
