/**
 * Dijkstra's algorithm.
 * First, call addEdge() to add edges.
 * Second, call solve() to calculate the length of the shortest path from source to each vertex.
 * Len: The type of cost values, must be Numeric
 * Note: It uses scala.reflect.ClassManifest, which is deprecated since 2.10.
 * Verified by AtCoder ABC021-C (http://abc021.contest.atcoder.jp/submissions/654618)
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
