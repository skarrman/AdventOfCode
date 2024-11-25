package sk.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import static java.util.Arrays.stream;

public class Main {

    private final List<String> initializationSequences;

    Main(List<String> initializationSequences) {
        this.initializationSequences = initializationSequences;
    }

    int solve1() {
        return initializationSequences.stream().map(Main::hash).reduce(0, Integer::sum);
    }

    public record Lens(String label, int length) {
    }

    int solve2() {
        var table = new HashMap<Integer, List<Lens>>(255);
        for (var instString : initializationSequences) {
            var inst = instString.split("[=\\-]");
            var hash = hash(inst[0]);
            if (!table.containsKey(hash))
                table.put(hash, new ArrayList<>());
            var box = table.get(hash);
            var index = -1;
            for (var i = 0; i < box.size(); i++) {
                if (box.get(i).label.equals(inst[0])) {
                    index = i;
                    break;
                }
            }
            if (instString.endsWith("-") && index != -1)
                box.remove(index);
            if (instString.contains("=") && index == -1)
                box.add(new Lens(inst[0], Integer.parseInt(inst[1])));
            else if (instString.contains("="))
                box.set(index, new Lens(inst[0], Integer.parseInt(inst[1])));
        }
        return IntStream.range(0, 256).map(l -> IntStream.range(0, table.get(l) == null ? 0 : table.get(l).size())
                .reduce(0, (s, i) -> s + (l + 1) * (i + 1) * table.get(l).get(i).length)).reduce(0, Integer::sum);
    }

    private static int hash(String initializationSequence) {
        var value = 0;
        for (char c : initializationSequence.toCharArray()) {
            value += c;
            value *= 17;
            value %= 256;
        }
        return value;
    }

    public static void main(String[] args) throws IOException {
        var part = System.getenv("part") == null ? "part1" : System.getenv("part");
        var input = parseInput("input.txt");
        if (part.equals("part2"))
            System.out.println(new Main(input).solve2());
        else
            System.out.println(new Main(input).solve1());
    }

    private static List<String> parseInput(String filename) throws IOException {
        return stream(Files.readString(Path.of(filename)).split(",")).collect(Collectors.toList());
    }

}