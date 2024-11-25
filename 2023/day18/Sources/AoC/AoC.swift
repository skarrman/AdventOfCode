import Foundation

@main
public struct AoC {
    let input: [(String, Int)]
    
    init(_ data: [(String, Int)]) {
        self.input = data
    }
    
    public static func main() {
        let part = ProcessInfo.processInfo.environment["part"] ?? "part1"
        
        if part == "part2" {
            let input = parse(file: "input.txt", snd: true)
            print(AoC(input).getSolution())
        } else  {
            let input = parse(file: "input.txt")
            print(AoC(input).getSolution())
        }
    }
    
    static func parse(file filename: String, snd: Bool = false) -> [(String, Int)] {
        guard let content = try? String(contentsOfFile: filename) else {fatalError("Error parsing file \(filename)")}
            
        return content.components(separatedBy: .newlines).compactMap{
            row in
            let splits = row.split(separator: " ")
            if snd {
                let start = splits[2].index(after: splits[2].firstIndex(of: "#")!)
                let end = splits[2].index(start, offsetBy: 4)
        
                guard let num = Int(String(splits[2][start...end]), radix: 16) else {fatalError("Could not parse hex")}
                
                switch splits[2].dropLast().last {
                    case "0": return ("R", num)
                    case "1": return ("D", num)
                    case "2": return ("L", num)
                    case "3": return ("U", num)
                    default: fatalError("Error parsing dir")
                }
            } else {
                guard let num = Int(splits[1]) else {fatalError("Could not parse int")}
                return (String(splits[0]), num)}
            }
    }

    func getSolution() -> Int {
        var pos = Point(x:0,y:0)
        var trench = Set<Point>()
        var vertices: [Point] = [pos];
        trench.insert(pos)
        var area = 0
        for (dir, len) in input {
            let d = dir == "U" ? Point(x:0,y:-len) : (dir == "R" ? Point(x:len,y:0) : (dir == "D" ? Point(x:0,y:len) : Point(x:-len,y:0)))
            pos = pos + d
            area += dir == "U" || dir == "L" ? len : 0
            vertices.append(pos)
        }

        for i in 0..<vertices.count-1 {
            let j = i + 1
            area += (vertices[i].y + vertices[j].y) * (vertices[i].x - vertices[j].x) / 2
        }
        return area + 1
    }
}

struct Point {
    let x,y:Int
}
extension Point: Hashable{
    static func == (lhs: Point, rhs: Point) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y
    }

    static func + (lhs: Point, rhs: Point) -> Point {
        return Point(x: lhs.x+rhs.x, y: lhs.y+rhs.y)
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
    }
}