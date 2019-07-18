class ResistorColors
  @color_numbers = {
    'black' => 0,
    'brown' => 1,
    'red' => 2,
    'orange' => 3,
    'yellow' => 4,
    'green' => 5,
    'blue' => 6,
    'violet' => 7,
    'grey' => 8,
    'white' => 9
  }

  def self.value(colors)
    total = 0
    colors.each do |color|
      total *= 10
      total += @color_numbers[color]
    end
    total
  end
end
