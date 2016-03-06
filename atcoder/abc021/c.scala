/**
 * Dijkstra's algorithm.
 * First, call addEdge() to add edges.
 * Second, call solve() to calculate the length of the shortest path from source to each vertex.
 * Len: The type of cost values, must be Numeric
 * Note: It uses scala.reflect.ClassManifest, which is deprecated since 2.10.
 * Verified by AtCoder ABC021-C
 */
final class Dijkstra(n: Int, inf: Long = 0x3fffffff) {
  import scala.collection.{mutable => mu}
  private[this] val edges = Array.fill(n)(mu.ArrayBuffer[(Int, Long)]())
  @inline
  def addEdge(from: Int, to: Int, cost: Long) {
    edges(from) += ((to, cost))
  }
  @inline
  def solve(src: Int): Array[Long] = {
    val d = Array.fill(n)(inf)
    val que = mu.PriorityQueue[(Long, Int)]()(Ordering.Tuple2[Long, Int].reverse)
    que += ((0L, src))
    while (que.nonEmpty) {
      val (dist, idx) = que.dequeue
      if (d(idx) > dist) {
        d(idx) = dist
        for ((eto, ecost) <- edges(idx)) {
          que += ((dist + ecost, eto))
        }
      }
    }
    d
  }
}
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
  val dijk = new Dijkstra(n)
  for ((x, y) <- e) {
    dijk.addEdge(x, y, 1)
    dijk.addEdge(y, x, 1)
  }
  val distA = dijk.solve(a)
  // create shortest path DAG
  val dag = Array.fill(n)(mu.ArrayBuffer[Int]())
  for ((x, y) <- e) {
    if (distA(x) - distA(y) == -1) {
      dag(x) += y
    }
    if (distA(x) - distA(y) == 1) {
      dag(y) += x
    }
  }
  val memo = Array.fill(n)(-1L)
  def rec(v: Int): Long = {
    if (memo(v) >= 0) memo(v)
    else
      if (v == b) 1L
      else {
        var sum: Long = 0
        for (n <- dag(v)) {
          sum += rec(n)
          sum %= 1000000007L
        }
        memo(v) = sum
        sum
      }
  }
  println(rec(a))
}