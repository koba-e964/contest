/*
  TLE
 */
/**
 * Dijkstra's algorithm.
 * First, call add_edge() to add edges.
 * Second, call solve() to calculate the length of the shortest path from source to each vertex.
 * Len: The type of cost values, must be Numeric
 * Note: It uses scala.reflect.ClassManifest, which is deprecated since 2.10.
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
 
  val br = new java.io.BufferedReader(new java.io.InputStreamReader(System.in))
  val ntl = br.readLine.split(" ").map(Integer.parseInt)
  val n = ntl(0)
  val tl = ntl(1)
  val dijk = new Dijkstra(tl + 1, 1L << 60)
  for (_ <- 0 until n) {
    val lrc = br.readLine.split(" ").map(Integer.parseInt)
    dijk.addEdge(lrc(0), lrc(1), lrc(2))
  }
  for (i <- 0 until tl) dijk.addEdge(i + 1, i, 0)
  println(dijk.solve(0)(tl))
}