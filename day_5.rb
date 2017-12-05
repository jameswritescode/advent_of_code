def hash
  inst = File.readlines('input.txt').map(&:to_i)

  {}.tap do |h|
    inst.each.with_index { |v, i| h[i] = v }
  end
end

# step 1
s1 = hash
steps = 0
on_index = 0
door = s1.keys.last

loop do
  break if on_index > door

  value = s1[on_index]
  s1[on_index] += 1
  on_index += value unless value.zero?
  steps += 1
end

puts steps

# step 2

s2 = hash
steps = 0
on_index = 0

loop do
  break if on_index > door

  value = s2[on_index]

  if value >= 3
    s2[on_index] -= 1
  else
    s2[on_index] += 1
  end

  on_index += value unless value.zero?
  steps += 1
end

puts steps
