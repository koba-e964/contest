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
  val delim = Array(0, 9, 189, 2889, 38889, 488889, 5888889, 68888889)
  val st =    Array(1,10, 100, 1000, 10000, 100000, 1000000, 10000000)
  final def nth(n: Int): Int = {
    val pos = delim.filter(_ <= n - 1).length - 1
    val off = n - delim(pos) - 1
    val dlen = pos + 1
    (off / dlen + st(pos)).toString()(off % dlen) - '0'
  }
  println((0 to 6).map(st).map(nth))
}