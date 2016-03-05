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
  val mod: Long = 1000000007L
  def invmod(x: Long): Long = {
    var e = mod - 2
    var sum: Long = 1
    var cur: Long = x
    while (e > 0) {
      if (e % 2 == 1) {
        sum = (sum * cur) % mod
      }
      cur = (cur * cur) % mod
      e /= 2
    }
    sum
  }
  val n, k = sc.nextInt
  var s: Long = 1
  for (i <- 0 until k) {
    s = (s * (n + i)) % mod
    s = (s * invmod(i + 1)) % mod
  }
  println(s)
}