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
  def c(n: Int, k: Int): Double =
    (1 to k).map(y => Math.log((n - y + 1)) - Math.log(y)).sum
  def solve(n: Int, x: Int, y: Int): Double = {
    if ((x + y + n) % 2 == 0) {
      val u = (n - x - y) / 2
      val v = (n + x - y) / 2
      // C(n, u) * C(n, v) / 4^n
      if (u < 0 || u > n || v < 0 || v > n) 0.0
      else {
        Math.exp(c(n, u) + c(n, v) - n * Math.log(4))
      }
    } else 0.0
  }
  val n, d, x, y = sc.nextInt
  println (if (x % d == 0 && y % d == 0) solve(n, x / d, y / d) else 0.0)
}