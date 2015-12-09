/*
 * Reference: http://lizan.asia/blog/2012/12/11/scala-competitive/
 */

object Main extends App {
  import java.{util => ju}
  import scala.annotation._
  import scala.collection._
  import scala.collection.{mutable => mu}
  import scala.collection.JavaConverters._
  import scala.math._

  final val sc = new ju.Scanner(System.in)
  final val fact = Array(1,1,2,6,24,120,720,5040,40320,362880)
  final def solve(n:Int) = {
    @tailrec
    def fsum(a:Int, s:Int):Int =
      a match {
        case 0 => s
        case _ => fsum(a / 10, s + fact(a % 10))
      }
    fsum(n, 0) == n
  }
  // n should be in [10,7 * 10!]
  var cnt = 0
  (10 to (7 * 362880)) foreach { n =>
    if (solve(n)) {
      cnt += n
      println(n)
    }
  }
  println("sum=" + cnt)
}