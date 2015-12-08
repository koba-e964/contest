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
  val que = new mu.Queue[(Int, Int)]
  val inf = 0x3fffffff
  val dist = Array.fill(n + 1)(inf)
  que += 0 -> 1
  while (que.nonEmpty) {
    val (d, t) = que.dequeue
    if (d < dist(t)) {
      dist(t) = d
      val bc = Integer.bitCount(t)
      if (t - bc >= 1) que += (d + 1) -> (t - bc)
      if (t + bc <= n) que += (d + 1) -> (t + bc)
    }
  }
  println(if (dist(n) == inf) -1 else dist(n) + 1)
}