function solution(snd)
  sum = 0
  for line in input
    its = [line]
    last = line
    while true
      diffs = []
      for i in 1:length(last)-1
        push!(diffs, last[i+1]-last[i])
      end
      push!(its, diffs)
      last = diffs
      vals = Set(diffs)
      if length(vals) == 1 && collect(vals)[1] == 0
        e = 0
        for it in reverse(its)
          e = if snd
            it[1] - e
          else
            it[length(it)] + e
          end
        end
        sum += e
        break
      end
    end
  end
  sum
end

input = open("input.txt") do file
  [[parse(Int32, num) for num in split(line)] for line in eachline(file)]
end

part = get(Base.ENV, "part", "part1")
println(solution(part == "part2"))

