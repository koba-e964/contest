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
  def calc(n : Int) : (Long, Int) = {
    var t : Long = 1
    var l : Int = 1
    var rem : Int = n
    while (true) {
      if (t >>> l != 0) {
        t = 1
        l += 1
      } else if (java.lang.Long.bitCount(t) % 3 != 0) {
        t += 2
      } else if (rem == 0) return (t, l)
      else {
        t += 2
        rem -= 1
      }
    }
    return (0,0)
  }
  val n = sc.nextInt
  val (res, len) = calc(n - 1)
  val str = (0 until len).map(i => if ((res & (1 << (len - i - 1))) != 0) "5" else "3").mkString
  println(str)
}