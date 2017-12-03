table = File.read('table.txt').split("\n").map { |l| l.split("\t").map(&:to_f) }

# step 1
puts table.map { |line| line.max - line.min }.sum.to_i

# step 2
value = table.inject(0) do |sum, line|
  line.each do |i|
    (line - [i]).each do |n|
      calc = i / n
      
      if calc % 1 == 0
        sum += calc
        break
      end
    end
  end

  sum
end

puts value.to_i
