import scala.io.Source
import scala.collection.mutable.ListBuffer
import collection.mutable.Map

object Main {
  trait DirItem;

  case class Item(size: Int) extends DirItem;

  case class Dir(path: String) extends DirItem;

  trait PathItem;

  case class Size(value: Int) extends PathItem;

  case class DirItems(items: List[DirItem]) extends PathItem;


  def sizeOf(path: String, dirs: Map[String, PathItem]): Int = dirs(path) match {
    case Size(value) => value
    case DirItems(items) => {
      var size: Int = items.foldLeft(0)((totSize: Int, item: DirItem) => totSize + (item match {
        case Item(size) => size
        case Dir(path) => sizeOf(path, dirs)
      }));
      dirs(path) = Size(value = size);
      size
    }
  }


  def main(args: Array[String]) = {
    val rows = Source.fromFile("input.txt").getLines.toArray.map(_.toString)
    val part = sys.env.get("part").getOrElse("part1")
    var dirs = Map[String, PathItem]();
    var currPath = "";
    var i = 0;
    while (i < rows.length) {
      rows(i) match {
        case command
          if command.startsWith("$")
        => command match {
          case c if c.contains("cd") => {
            c.substring(5) match {
              case ".." => currPath = currPath.substring(0, currPath.substring(0, currPath.length - 2).lastIndexOf("/") + 1)
              case "/" => currPath = "/"
              case dir => currPath += s"$dir/"
            }
          }
          case c if c.contains("ls") => {
            var currDir = ListBuffer[DirItem]();
            while (i + 1 < rows.length && !rows(i + 1).contains("$")) {
              i = i + 1;
              rows(i) match {
                case dir if dir.contains("dir") => currDir += Dir(path = s"$currPath${dir.substring(4)}/")
                case size => currDir += Item(size = size.substring(0, size.indexOf(' ')).toInt)
              }
            }
            dirs(currPath) = DirItems(items = currDir.toList);
          }
          case _ => throw new IllegalArgumentException()
        }
      }
      i = i + 1;
    }
    var path = "";
    var dirSizes = Map[String, Int]();
    for (path <- dirs.keys) {
      val size = sizeOf(path, dirs)
      dirSizes(path) = size
    }
    val totalSize = dirSizes("/");
    val result = if (part == "part2") dirSizes.values.filter((s) => totalSize - s < 70000000 - 30000000).min else dirSizes.values.filter(_ < 100000).sum
    Console.println(result);
  }
}