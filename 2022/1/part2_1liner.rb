p File.readlines('input').chunk_while {|line| line != "\n" }.map {|a| a.map(&:to_i).sum }.max(3).sum
