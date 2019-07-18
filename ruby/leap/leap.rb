class Year
  def self.leap?(year)
    (year % 400).zero? || ((year % 100) != 0 && (year % 4).zero?)
  end
end
