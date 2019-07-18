class SpaceAge
  def initialize(seconds)
    @seconds = seconds
  end

  def on_earth
    @seconds / 31_557_600.0
  end

  def on_mercury
    @seconds / (31_557_600 * 0.2408467)
  end

  def on_venus
    @seconds / (31_557_600 * 0.61519726)
  end

  def on_mars
    @seconds / (31_557_600 * 1.8808158)
  end

  def on_jupiter
    @seconds / (31_557_600 * 11.862615)
  end

  def on_saturn
    @seconds / (31_557_600 * 29.447498)
  end

  def on_uranus
    @seconds / (31_557_600 * 84.016846)
  end

  def on_neptune
    @seconds / (31_557_600 * 164.79132)
  end
end
