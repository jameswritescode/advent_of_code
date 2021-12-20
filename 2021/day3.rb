# frozen_string_literal: true

class DiagnosticInfo
  attr_reader :store

  def initialize
    @store = Hash.new(0)
  end

  def add(n)
    store[n] += 1
  end

  def epsilon
    store.min do |(_n1, t1), (_n2, t2)|
      t1 <=> t2
    end[0]
  end

  def gamma
    store.max do |(_n1, t1), (_n2, t2)|
      t1 <=> t2
    end[0]
  end
end

class BinaryDiagnostic
  attr_reader :input, :diagnostics

  def initialize
    @input = File.read('day3.txt').split("\n")
    @diagnostics = []
  end

  def part1
    input.each do |line|
      line.split('').each_with_index do |n, i|
        diag = diagnostics[i] ||= DiagnosticInfo.new
        diag.add(n)
      end
    end

    epsilon = ''
    gamma = ''

    diagnostics.each do |diag|
      epsilon += diag.epsilon
      gamma += diag.gamma
    end

    puts "part1: #{epsilon.to_i(2) * gamma.to_i(2)}"
  end

  def part2
  end

  def answers
    part1
    part2
  end
end

BinaryDiagnostic.new.answers
