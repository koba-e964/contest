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
  @tailrec
  def calc(t : Long, l : Int, rem : Int) : (Long, Int) = {
    () match {
      case _ if (t >>> l != 0) => calc(1, l + 1, rem)
      case _ if (java.lang.Long.bitCount(t) % 3 != 0) => calc(t + 2, l, rem)
      case _ if (rem == 0) => (t, l)
      case _ => calc(t + 2, l, rem - 1)
    }
  }
  val n = sc.nextInt
  val (res, len) = calc(1, 1, n - 1)
  val str = (0 until len).map(i => if ((res & (1 << (len - i - 1))) != 0) "5" else "3").mkString
  println(str)
}