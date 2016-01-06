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
  val x = sc.nextInt - 1
  val h = Array.fill(n)(sc.nextInt)
  val edges = Array.fill(n)(mu.ArrayBuffer[Int]())
  for (_ <- 0 until (n - 1)) {
    val a, b = sc.nextInt - 1
    edges(a) += b
    edges(b) += a
  }
  def dfs(v: Int, prev: Int): (Int, Boolean) = {
    var sum = 0
    var hasChild = h(v) == 1
    for (t <- edges(v)) {
      if (t != prev) {
         val (sub, sub2) = dfs(t, v)
         if (sub2) {
           sum += sub + 1
           hasChild = true
         }
      }
    }
    (sum, hasChild)
  }
  println(dfs(x,-1)._1 * 2)
}