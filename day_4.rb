pwds = File.readlines('input.txt')

# step 1
value = pwds.inject(0) do |valid, pp|
  sec = pp.split(' ')
  valid += 1 if sec.uniq === sec

  valid
end

puts value

# step 2
value = pwds.inject(0) do |valid, pp|
  words = pp.split(' ').map { |w| w.split('').sort }
  valid += 1 if words.uniq == words

  valid
end

puts value
