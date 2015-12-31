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
  val mod = 1000000007L
  @inline
  def invmod(x: Long) : Long = powmod(x, mod - 2)
  @inline
  def powmod(x: Long, e: Long): Long = {
    var sum: Long = 1
    var cur: Long = x
    var exp: Long = e
    while (exp > 0) {
      if (exp % 2 == 1) {
        sum *= cur
        sum %= mod
      }
      cur = cur * cur % mod
      exp /= 2
    }
    sum
  }
  val r,c,x,y,d,l = sc.nextInt
  if (d + l != x * y) {
    throw new RuntimeException
  }
  var tot: Long = (r - x + 1) * (c - y + 1) % mod
  for (i <- 1 to l) {
    tot = tot * invmod(i) % mod
    tot = tot * (x * y - i + 1) % mod
  }
  println(tot)
}