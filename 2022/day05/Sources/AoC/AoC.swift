import Foundation

@main
public struct AoC {
    var crates: [Int: [String]]
    let moves: [(Int, Int, Int)]
    
    init(_ data: ([Int: [String]], [(Int, Int, Int)])) {
        self.crates = data.0
        self.moves = data.1
    }
    
    public static func main() {
        let part = ProcessInfo.processInfo.environment["part"] ?? "part1"
        let input = parse(file: "input.txt")

        var aoc = AoC(input)
        if part == "part2" {
            print(aoc.getSolutionPart2())
        } else  {
            print(aoc.getSolutionPart1())
        }
    }
    
    static func parse(file filename: String) -> ([Int: [String]], [(Int, Int, Int)]) {
        guard let content = try? String(contentsOfFile: filename) else {fatalError("Error parsing file \(filename)")}

        let sections = content.components(separatedBy: "\n\n")
        let crates = sections[0]
        let moves = sections[1]
        var cratesMap: [Int: [String]] = [:]
        for row in crates.components(separatedBy: .newlines){
            for (i, crate) in row.split(separator: " ").enumerated(){
                if crate == "[_]" || !crate.contains("[") {continue}
                var lst = cratesMap[i+1] ?? []
                lst.append(String(crate))
                cratesMap[i+1] = lst
            }
        }
        
        var ms: [(Int, Int, Int)] = []
        for move in moves.components(separatedBy: .newlines).filter({$0 != ""}){
            guard let i0 = move.firstIndex(of: " "), let i1 = move.firstIndex(of: "f"), let i2 = move.firstIndex(of: "t") else {fatalError("")}
            guard let m = Int(move[move.index(after: i0)..<move.index(i1, offsetBy: -1)]) else {fatalError("Error")}
            guard let f = Int(move[move.index(i1, offsetBy: 5)..<move.index(i2, offsetBy: -1)]) else {fatalError("Error")}
            guard let t = Int(move[move.index(i2, offsetBy: 3)..<move.endIndex]) else {fatalError("Error")}
            ms.append((m,f,t))
        }
        return (cratesMap, ms)
    }
    
    mutating func getSolutionPart1() -> String {
        for move in moves {
            for _ in 0..<move.0 {
                if let c = crates[move.1]?.remove(at: 0){
                    crates[move.2]?.insert(c, at: 0)
                }
            }
        }
        var s = ""
        for c in crates.keys.sorted() {
            s = s + (crates[c]?[0] ?? "Error")
        }
        return s
    }
    
    mutating func getSolutionPart2() -> String {
        for move in moves {
            var cs:[String] = []
            for _ in 0..<move.0 {
                if let c = crates[move.1]?.remove(at: 0){
                    cs.append(c)
                }
            }
            crates[move.2]?.insert(contentsOf: cs, at: 0)
        }
        var s = ""
        for c in crates.keys.sorted() {
            s = s + (crates[c]?[0] ?? "Error")
        }
        return s
    }
    
}
