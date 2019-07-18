class FlattenArray
  def self.flatten(array)
    array.compact.flat_map do |item|
      if item.is_a? Array
        flatten(item)
      else
        item
      end
    end
  end
end
