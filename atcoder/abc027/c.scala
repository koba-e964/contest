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
  def l2(v: Long) : Int = 63 - java.lang.Long.numberOfLeadingZeros(v)
  def win(x: Long, n: Long) : Boolean = {
    if (x > n) {
      return true
    }
    val t = n / x
    val u = n / (x + 1)
    if (l2(t) == l2(u)) {
      l2(t) % 2 == 1
    } else {
      !(win(2 * x, n) && win(2 * x + 1, n))
    }
  }
  val n = sc.nextLong
  println(if (win(1, n)) "Takahashi" else "Aoki")
}