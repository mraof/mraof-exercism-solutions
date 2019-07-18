class Complement
  def self.of_dna(dna)
    dna.split('').map do |n|
      case n
      when 'G'
        'C'
      when 'C'
        'G'
      when 'T'
        'A'
      when 'A'
        'U'
      else
        ''
      end
    end.join
  end
end
