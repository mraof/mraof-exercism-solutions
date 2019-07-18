require 'set'

class Pangram
  def self.pangram?(sentence)
    sentence.upcase.scan(/[A-Z]/).to_set.length == 26
  end
end
