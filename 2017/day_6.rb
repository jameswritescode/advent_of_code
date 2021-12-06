# step 1 & 2
blocks = File.read('input.txt').split("\t").map(&:to_i)
history = [blocks]

loop do
  blocks = blocks.clone
  max = blocks.max
  max_index = blocks.index(max)
  blocks[max_index] = 0
  rotate_to = max_index + 1

  max.times do
    rotate = blocks.rotate(rotate_to)
    rotate[0] += 1
    blocks = rotate.rotate(-rotate_to)
    rotate_to += 1
  end

  if history.include?(blocks)
    puts "step 1: #{history.count}"
    puts "step 2: #{history.count - history.index(blocks)}"
    break
  end

  history << blocks
end

# Couple of notes here for me and anyone bothering to read:
# TIL of Array#rotate (which feels kind of dirty)
# TIL from my friend @joeltaylor that a nice alternative is array[index % array.length]
# so 13-15 could be replaced with:
#  
#   blocks[rotate_to % blocks.count] += 1
