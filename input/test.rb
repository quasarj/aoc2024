# frozen_string_literal: true

calibrations = []

File.foreach('day7.txt') do |line|
  target, values = line.split(':')
  calibrations << [target.to_i, values.strip.split.map(&:to_i)]
end

calibration_result = 0

calibrations.each do |target, values|
  results = [values.shift]
  values.each do |value|
    new_results = []
    results.each do |result|
      new_results << result + value
      new_results << result * value
    end
    results = new_results
  end
  calibration_result += target if results.include?(target)
end

puts calibration_result

calibrations = []

File.foreach('day7.txt') do |line|
  target, values = line.split(':')
  calibrations << [target.to_i, values.strip.split.map(&:to_i)]
end

calibration_result = 0

calibrations.each do |target, values|
  results = [values.shift]
  values.each do |value|
    new_results = []
    results.each do |result|
      new_results << result + value
      new_results << result * value
      new_results << "#{result}#{value}".to_i
    end
    results = new_results
  end
  calibration_result += target if results.include?(target)
end

puts calibration_result
