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

  val m = Array.fill(5)(sc.nextInt)
  val w = Array.fill(5)(sc.nextInt)
  val x = Array.tabulate(5)(i => (i + 1) * 500)
  val hs, hu = sc.nextInt
  println((0 until 5).map(i => (3 * x(i) / 10) max (x(i) - x(i) * m(i) / 250 - 50 * w(i))).sum + 100 * hs - 50 * hu)
}