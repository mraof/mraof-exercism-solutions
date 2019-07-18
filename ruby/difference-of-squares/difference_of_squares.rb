class Squares
  def initialize(num)
    @num = num
  end

  def square_of_sum
    sum = (1..@num).sum
    sum * sum
  end

  def sum_of_squares
    (1..@num).map { |n| n * n }.sum
  end

  def difference
    square_of_sum - sum_of_squares
  end
end
