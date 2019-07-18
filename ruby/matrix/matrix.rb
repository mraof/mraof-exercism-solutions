class Matrix
  def initialize(string)
    @rows = string.split("\n").collect { |r| r.split(' ').collect(&:to_i) }
  end

  attr_reader :rows

  def columns
    rows.transpose
  end
end
