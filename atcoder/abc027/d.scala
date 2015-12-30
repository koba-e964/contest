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

  val sc = new ju.Scanner(System.in)
  val s = sc.next
  val n = s.length
  val m = s.count(_ == 'M') / 2
  var p = 0
  var t = 0
  val a = Array.fill(2 * m)(0)
  for (i <- 0 until n) {
    if (s(i) != 'M') {
      t += (if (s(i) == '+') 1 else -1)
    } else {
      a(p) = t
      p += 1
    }
  }
  val sor = a.sorted.reverse
  println(sor.slice(0, m).sum - sor.slice(m, 2 * m).sum)
}