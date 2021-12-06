# frozen_string_literal: true

class SonarSweep
  attr_reader :input

  def initialize
    @input = File.read('day1.txt').split("\n").map(&:to_i)
  end

  def increments
    increments = 0
    last = 0

    input.each_with_index do |el, i|
      increments += 1 if !i.zero? && el > last
      last = el
    end

    puts "increments: #{increments}"
  end

  def sliding_window_increments
    increments = 0
    curr_index = 0
    last = 0

    loop do
      sum = input[curr_index...curr_index + 3].sum
      increments += 1 if !curr_index.zero? && sum > last
      last = sum
      curr_index += 1

      break if curr_index == input.size - 1
    end

    puts "sliding_window_increments: #{increments}"
  end

  def answers
    increments
    sliding_window_increments
  end
end

SonarSweep.new.answers
