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
  val n = sc.nextInt
  val m = Array.fill(n)(sc.nextInt).sorted
  val tot = m.sum
  val res = Array.tabulate(n)(i => (tot + i) / n)
  println((0 until n).map(i => (m(i) - res(i)) max 0).sum)
}