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
