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
  val ta, tb, (t,v) = (sc.nextInt, sc.nextInt)
  val n = sc.nextInt
  val h = Array.fill(n)((sc.nextInt, sc.nextInt))
  def ok(p: (Int, Int)): Boolean = {
    val d1 = Math.pow(ta._1 - p._1, 2) + Math.pow(ta._2 - p._2, 2)
    val d2 = Math.pow(tb._1 - p._1, 2) + Math.pow(tb._2 - p._2, 2)
    Math.sqrt(d1) + Math.sqrt(d2) < v * t + 1e-9
  }
  println(if (h.exists(ok)) "YES" else "NO")
}