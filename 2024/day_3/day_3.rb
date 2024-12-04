# frozen_string_literal: true

class Day3
  attr_reader :content
  def initialize(content)
    @content = content
  end

  def call
    part1
    part2
  end

  private

  def part1
    puts self.parsed
      .select { |kw, _| kw == 'mul' }
      .sum { |_, a, b| a.to_i * b.to_i }
  end

  def part2
    tracking = true

    result = self.parsed.inject(0) do |memo, (kw, a, b)|
        case kw
        when "don't" then tracking = false
        when 'do' then tracking = true
        when 'mul' then memo += a.to_i * b.to_i if tracking
        end

        memo
      end

    puts result
  end

  def parsed
    @parsed ||= self.content.scan(/(do(?:n't)?|mul)\((?:(\d+),(\d+))?\)/)
  end
end

Day3.new(File.read(ARGV[0])).call
