# frozen_string_literal: true

class Dive
  attr_reader :input

  def initialize
    @input = File.read('day2.txt').split("\n")
  end

  def final_horizontal_x_depth
    depth = 0
    horizontal = 0

    read_directions(
      down: ->(num) { depth += num },
      forward: ->(num) { horizontal += num },
      up: ->(num) { depth -= num }
    )

    puts "final_horizontal_x_depth: #{horizontal * depth}"
  end

  def final_horizontal_x_depth_with_aim
    aim = 0
    depth = 0
    horizontal = 0

    read_directions(
      down: ->(num) { aim += num },
      up: ->(num) { aim -= num },
      forward: lambda do |num|
        horizontal += num
        depth += aim * num
      end,
    )

    puts "final_horizontal_x_depth_with_aim: #{horizontal * depth}"
  end

  def answers
    final_horizontal_x_depth
    final_horizontal_x_depth_with_aim
  end

  private

  def read_directions(up:,down:,forward:)
    input.each do |instruction|
      direction, num = instruction.split
      num = num.to_i

      case direction
      when 'down' then down.call(num)
      when 'forward' then forward.call(num)
      when 'up' then up.call(num)
      end
    end
  end
end

Dive.new.answers
