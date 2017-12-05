# step 1
# I couldn't figure this out on my own - this reddit post helped me figure out
# the math: https://goo.gl/yxtJTm
#
# I wanted to write that out so I could understand the math a little better.
num = 265149
i = num

loop do
  break if Math.sqrt(i) % 1 == 0 && i.odd?
  i += 1
end

grid = Math.sqrt(i)

puts ((grid - 1) - (i - num)).to_i

# step 2 (omg, coming later)
