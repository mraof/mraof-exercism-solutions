class Triangle
  def initialize(sides)
    @sides = sides.sort
  end

  def equilateral?
    valid? && (@sides[0] == @sides[1] && @sides[0] == @sides[2])
  end

  def isosceles?
    valid? && (@sides[0] == @sides[1] || @sides[0] == @sides[2] || @sides[1] == @sides[2])
  end

  def scalene?
    valid? && (@sides[0] != @sides[1] && @sides[0] != @sides[2] && @sides[1] != @sides[2])
  end

  def valid?
    @sides[0] > 0 && (@sides[0] + @sides[1]) >= @sides[2]
  end
end
