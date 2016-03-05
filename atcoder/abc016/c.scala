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

  val inf = 100000
  val n, m = sc.nextInt
  val e = Array.fill(m)((sc.nextInt - 1, sc.nextInt - 1))
  val dist = Array.fill(n)(Array.fill(n)(inf))

  for (i <- 0 until n) {
    dist(i)(i) = 0
  }
  for ((a, b) <- e) {
    dist(a)(b) = 1
    dist(b)(a) = 1
  }
  for (k <- 0 until n) {
    for (i <- 0 until n) {
      for (j <- 0 until n) {
        dist(i)(j) = dist(i)(j) min (dist(i)(k) + dist(k)(j))
      }
    }
  }
  println((0 until n).map(v => dist(v).count(_ == 2)).mkString("\n"))
}