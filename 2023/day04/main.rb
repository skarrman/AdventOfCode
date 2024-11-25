def file_content(filename="input.txt", winning_numbers, my_numbers)
    File.open(filename).readlines().each {|line|
    parts = line.split(" | ")
    winning = parts[0].split(": ")[1].split(" ")
    my = parts[1].split(" ")
    winning_numbers.append(winning);
    my_numbers.append(my)
 }
end

def get_solution_part1
    winning_numbers = [];
    my_numbers = [];
    file_content(winning_numbers, my_numbers)
    tot=0;
    winning_numbers.each_with_index {|values, index| 
        points=0;
        values.each {|val| 
            if my_numbers[index].include? val
                points = points == 0? 1: points*2
            end
         }
         tot+=points
    }
    return tot;
end

def get_solution_part2
    winning_numbers = [];
    my_numbers = [];
    file_content(winning_numbers, my_numbers)
    card={}
    winning_numbers.each_with_index {|values, index|
        original = card[index] == nil ? 1 : card[index]
        card[index] = original;
        matches=0;
        values.each {|val| 
            puts val
            if my_numbers[index].include? val
                matches += 1
            end
         }
         for i in 1..matches do
            if index+i >= winning_numbers.length
                next
            end
            if card[index+i] != nil
                card[index+i] += original
            else
                card[index+i] = 1 + original
            end
        end
    }
    
    return card.values.sum
end

if ENV["part"] == "part2" 
    puts get_solution_part2
else
    puts get_solution_part1
end
