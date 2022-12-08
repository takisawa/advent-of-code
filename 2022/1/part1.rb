
calories = 0
max_calories = 0

File.open('input', 'r') do |f|
  while line = f.gets
    line.chomp!

    case line
    when ""
      max_calories

      if calories > max_calories
        max_calories = calories
      end

      calories = 0
    else
      calories += line.to_i
    end
  end
end

p max_calories
