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
  val n, aa, bb, m = sc.nextInt
  val e = Array.fill(m)((sc.nextInt - 1, sc.nextInt - 1))
  val a = aa - 1
  val b = bb - 1
  val dist = Array.tabulate(n)(i => Array.tabulate(n)(j => if (i == j) 0 else 10000)
  val r = dist(a)(b).toInt
  var count: Long = 1
  for (i <- 1 until r) {
    val t: Long = (0 until n).count(j => distA(j) == i && distB(j) == r - i)
    count = (count * t) % 1000000007
  }
  println(count)
}