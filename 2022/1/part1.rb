
food_calories = 0
max_food_calories = 0

File.open('input', 'r') do |f|
  while line = f.gets
    line.chomp!

    case line
    when ""
      max_food_calories

      if food_calories > max_food_calories
        max_food_calories = food_calories
      end

      food_calories = 0
    else
      food_calories += line.to_i
    end
  end
end

p max_food_calories
