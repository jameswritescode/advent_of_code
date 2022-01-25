# frozen_string_literal: true

class BinaryDiagnostic
  attr_reader :input

  def initialize
    @input = File.read('day3.txt').split("\n")
  end

  def power_consumption
    groups = []
    epsilon = ''
    gamma = ''

    loop_diagnostics do |n, i|
      groups[i] ||= [0, 0]
      groups[i][n.to_i] += 1
    end

    groups.each do |zero, one|
      zero_gt_one = zero > one

      gamma += zero_gt_one ? '0' : '1'
      epsilon += zero_gt_one ? '1' : '0'
    end

    puts "power_consumption: #{gamma.to_i(2) * epsilon.to_i(2)}"
  end

  def life_support_rating
    loop_diagnostics do |n, i, line|

    end

    puts "life_support_rating: TODO"
  end

  def answers
    power_consumption
    life_support_rating
  end

  private

  def loop_diagnostics(lines = input)
    lines.each do |line|
      line.split('').each_with_index do |n, i|
        yield n, i, line
      end
    end
  end
end

BinaryDiagnostic.new.answers
