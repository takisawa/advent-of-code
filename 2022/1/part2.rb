
calories = 0
max_calories = []

File.open('input', 'r') do |f|
  while line = f.gets
    line.chomp!

    case line
    when ""
      max_calories

      if calories > (max_calories[-1] || 0)
        max_calories = (max_calories << calories).max(3)
      end

      calories = 0
    else
      calories += line.to_i
    end
  end
end

p max_calories.sum
